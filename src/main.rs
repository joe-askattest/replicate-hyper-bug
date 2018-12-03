
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use futures::future::Future;
use futures::stream::Stream;

//static URL : &'static str = "https://api.github.com/repos/joe-askattest/replicate-hyper-bug/test-file.tx";
static URL : &'static str = "https://raw.githubusercontent.com/joe-askattest/replicate-hyper-bug/master/README.md";

static USER_AGENT : &'static str = "replicate-hyper-bug (Rust)";

/// This is a replication of a bug I am hitting.
///
/// This test program will download a large file from Github,
/// and then returns the length of the request.
fn main() {
    println!("Make some tokio and hyper shizzazzle");
    let mut tokio_runtime = tokio::runtime::Runtime::new().unwrap();
    let tokio_executor = tokio_runtime.executor();
    let https = hyper_tls::HttpsConnector::new(4).unwrap();
    let client = hyper::client::Client::builder()
        .executor(tokio_executor)
        .build::<_, hyper::Body>(https);

    println!("make request");
    let mut request_builder = hyper::Request::builder();

    println!("making request to {}", &URL);
    request_builder.method(hyper::Method::GET).uri(URL);

    println!("set headers");
    request_builder.header("User-Agent", USER_AGENT);

    println!("set body");
    let body = hyper::Body::empty();
    let request = request_builder.body(body).unwrap();

    println!("make request future");
    let future = client
        .request(request)
        .map(|res| {
            println!("transform request to response object");
            let status = res.status().as_u16();
            let body = response_to_string(res);
            println!("transform request to response object, has status {}", status);

            body.len()
        })
        .map_err(|err| {
            println!(" !!! ERROR !!!");
            println!("{:?}", err);

            err
        });

    println!("block on future request");
    let response = tokio_runtime.block_on(future);

    let length = response.unwrap();

    println!(" >>> SUCCESS! >>> ");
    println!("Response length was ... {}", length)
}

fn response_to_string(response: hyper::Response<hyper::body::Body>) -> String {
    println!(">>>> response_to_string");
    let mut count = 0;

    let r = response
        .into_body()
        .map_err(|err| {
            println!(" !!! ERROR! !!!");
            println!("{:?}", err);
            ()
        })
        .fold(vec![], |mut acc, chunk| {
            println!(">>>> chunking {}", count);
            count = count+1;
            acc.extend_from_slice(&chunk);
            println!("<<<< chunking");
            Ok(acc)
        })
        .and_then(|v| {
            println!(">>>> chunk to string");
            let r = String::from_utf8(v).map_err(|_| ());
            println!("<<<< chunk to string");
            r
        })
        .wait()
        .unwrap();

    println!("<<<< response_to_string");

    r
}
