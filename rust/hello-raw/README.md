hello-raw
===

hello-raw is a small WASM project from chapter 5 of Fullstack Rust. The primary purpose of this Wasm module is to experience what one has to do in order to glue all the pieces together to get something to work without a Wasm framework.

The module itself just calls a `greet` function that will log `"Hello, Rust!"` to the console. To see this, run the `serve.py` server, go to `localhost:8080/www/` and examine the console.
