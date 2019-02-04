extern crate serde;
extern crate jsonrpc_core;
extern crate jsonrpc_derive;
extern crate jsonrpc_http_server;

use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi};

#[rpc]
pub trait Rpc {
	#[rpc(name = "hello")]
	fn hello(&self, _: String) -> Result<String>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn hello(&self, name: String) -> Result<String> {
       Ok(format!("Hello, {}!", name))
	}
}

fn main() {
	let mut io = jsonrpc_core::IoHandler::new();
    let rpc = RpcImpl;
	io.extend_with(rpc.to_delegate());

    let server = ServerBuilder::new(io)
		.threads(3)
		.rest_api(RestApi::Unsecure)
        .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Any]))
		.start_http(&"127.0.0.1:5555".parse().unwrap())
        .expect("Unable to start RPC server");

	server.wait();
}
