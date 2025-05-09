[![Rust](https://github.com/rickrain/hello-web-server/actions/workflows/rust.yml/badge.svg)](https://github.com/rickrain/hello-web-server/actions/workflows/ci.yml)

# Multithreaded Web Server

This is my implementation of the final project from [The Rust Programming Language](https://doc.rust-lang.org/book/) book. It is a primitive multithreaded web server that has two valid routes; `/` and `/sleep`. Both routes will return the text in `hello.html`, but the `/sleep` will sleep for 5 seconds before returning the response. Any other route requested is unknown to this web server and will therefore return the text in `404.html`.

## Getting Started

1. Start the web server by running `cargo run`.

2. Open a browser and navigate to `127.0.0.1:7878/` to see the hello message.

3. Open another browser tab and navigate to `127.0.0.1:7878/sleep`. After 5 seconds, the hello message will be returned.

4. Open another browser tab and navigate to `127.0.0.1:7878/foobar` to see the `404` message.

5. Refresh the browser tab for the `/sleep` route and then immediately after refresh the browser tab to the `/` route. The `/` will return immediately while the `/sleep` will return after 5 seconds.

To stop the server at any time, press `Ctrl-C`. However, this is not a graceful shutdown. To observe the graceful shutdown logic, continue sending requests to the server. After 10 requests, the server will shutdown gracefully, which can be observed by watching the output in the terminal window.