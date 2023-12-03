# Example HTTP api written in Rust

This is a simple example of a HTTP api written in Rust using the warp crate.

## Running

The server can be run with the following command:

```bash
cargo run

...

Running example server on localhost:3030
```

## Endpoints

The example provides a very simple endpoint that returns a JSON response:

```bash
curl localhost:3030/todos/1

{"id":1,"title":"Example todo","completed":false}
```

## Testing

The example provides a simple test suite that can be run with the following command:

```bash
cargo test
```

If you want coverage information, you can run the following command (assuming you have [cargo-llvm-cov](https://lib.rs/crates/cargo-llvm-cov)):

```bash
cargo llvm-cov
```


