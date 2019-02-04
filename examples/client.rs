#[macro_use]
extern crate jsonrpc_client_core;
extern crate jsonrpc_client_http;

use jsonrpc_client_http::HttpTransport;

jsonrpc_client!(pub struct RpcClient {
    pub fn hello(&mut self, name: String) -> RpcRequest<String>;
});

fn main() {
    let transport = HttpTransport::new().standalone().unwrap();
    let transport_handle = transport
        .handle("http://localhost:5555")
        .unwrap();
    let mut client = RpcClient::new(transport_handle);
    let name = "dinosaur".to_string();
    let result = client.hello(name).call().unwrap();
    println!("{}", result);
}
