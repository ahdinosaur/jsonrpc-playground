#[macro_use] extern crate failure;
extern crate serde;
extern crate jsonrpc_core;
extern crate jsonrpc_derive;
extern crate jsonrpc_http_server;

use failure::Fail;
use jsonrpc_core as rpc;
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi};

#[rpc]
pub trait Rpc {
	#[rpc(name = "hello")]
	fn hello(&self, _: String) -> rpc::Result<String>;

	#[rpc(name = "error")]
	fn error(&self, _: String) -> rpc::Result<String>;
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn hello(&self, name: String) -> rpc::Result<String> {
       Ok(format!("Hello, {}!", name))
	}

	fn error(&self, name: String) -> rpc::Result<String> {
       Err(rpc::Error::from(PlaygroundError::Wild { name }))
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

#[derive(Debug, Fail)]
pub enum PlaygroundError {
    #[fail(display = "a wild error appears for {}", name)]
    Wild {
        name: String
    },

    #[fail(display = "oh no!")]
    OhNo {}
}

impl From<PlaygroundError> for rpc::Error {
	fn from(err: PlaygroundError) -> Self {
		match &err {
			PlaygroundError::Wild { name: _ } => rpc::Error {
				code: rpc::ErrorCode::ServerError(1),
				message: err.name().unwrap().to_string(),
                data: Some(err.to_string().into())
			},
			err => rpc::Error {
                code: rpc::ErrorCode::InternalError,
                message: "Internal error!".into(),
                data: Some(format!("{:?}", err).into())
            }
		}
	}
}
