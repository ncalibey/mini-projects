messages-actix
===

This is an example Actix web server from the Fullstack Rust book (chapters 2 and 3). To run it, just `cargo run`.

### API Examples
Create a new message:
```bash
$ curl -X POST -H "Content-Type: application/json" -d '{"message": "foo"}' localhost:8080/send
```

Get all messages:
```bash
$ curl localhost:8080
```

Get a specific message:
```bash
$ curl localhost:8080/lookup/<index>
```

Clear all messages:
```bash
$ curl -X POST localhost:8080/clear
```
