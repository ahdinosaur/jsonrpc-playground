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

---

you can also use `curl` as a client:

```
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "hello", "params": ["dinosaur"], "id":1 }' 127.0.0.1:5555
```

you should see

```json
{"jsonrpc":"2.0","result":"Hello, dinosaur!","id":1}
```

---

now try the `error` method:

```
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "error", "params": ["dinosaur"], "id":1 }' 127.0.0.1:5555
```

you should see

```json
{"jsonrpc":"2.0","error":{"code":1,"message":"server::PlaygroundError","data":"a wild error appears for dinosaur"},"id":1}
```
