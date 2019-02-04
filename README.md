# jsonrpc-playground

playing around with [JSON-RPC in Rust](https://github.com/paritytech/jsonrpc)

## demo

[with Rust installed](https://rustup.rs/):

```shell
git clone https://github.com/ahdinosaur/jsonrpc-playground
cd jsonrpc-playground
```

in one terminal, run the [example server](./examples/server.rs):

```shell
cargo run --example server
```

in another terminal, run the [example client](./examples/client.rs):

```shell
cargo run --example client
```

in the client output, you should see

```txt
hello, dinosaur!
```

yay! :heart:
