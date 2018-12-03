# replicate-hyper-bug
There is a bug I keep hitting with hyper. This is a project aiming to replicate it.

## To run

 * Check it out.
 * `cargo run`

## What does it do?

It downloads this README.md file from the internet, turns the response into a strong, and then prints the length of the response.

## What should I expect?

Most of the time it runs and works.

When this happens it should print ...

```
>> cargo run

  replicate-hyper-bug cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target\debug\replicate-hyper-bug.exe`
Make some tokio and hyper shizzazzle
make request
making request to https://raw.githubusercontent.com/joe-askattest/replicate-hyper-bug/master/README.md
set headers
set body
make request future
block on future request
transform request to response object
>>>> response_to_string
>>>> chunking 0
<<<< chunking
>>>> chunk to string
<<<< chunk to string
<<<< response_to_string
transform request to response object, has status 200
 >>> SUCCESS! >>>
Response length was ... 106
```

## What goes wrong?

For me, it fails to run in about 1 in 5, or 1 in 10 attempts.

When this happens it prints ...

```
>> cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target\debug\replicate-hyper-bug.exe`
Make some tokio and hyper shizzazzle
make request
making request to https://raw.githubusercontent.com/joe-askattest/replicate-hyper-bug/master/README.md
set headers
set body
make request future
block on future request
transform request to response object
>>>> response_to_string
>>>> chunking 0
<<<< chunking
```

 ... and then just hangs.

