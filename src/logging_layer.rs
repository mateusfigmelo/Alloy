use alloy::{
    node_bindings::Anvil,
    providers::{Provider, ProviderBuilder},
    rpc::{
        client::ClientBuilder,
        json_rpc::{RequestPacket, ResponsePacket},
    },
    transports::TransportError,
};
use eyre::Result;
use std::{
    fmt::Debug,
    future::{Future, IntoFuture},
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};

struct LoggingLayer;
impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        LoggingService { inner }
    }
}

#[derive(Debug, Clone)]
struct LoggingService<S> {
    inner: S,
}

impl<S> Service<RequestPacket> for LoggingService<S>
where
    S: Service<RequestPacket, Response = ResponsePacket, Error = TransportError>,
    S::Future: Send + 'static,
    S::Response: Send + 'static + Debug,
    S::Error: Send + 'static + Debug,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
    fn call(&mut self, req: RequestPacket) -> Self::Future {
        println!("Request: {:?}", req);
        let fut = self.inner.call(req);
        Box::pin(async move {
            let res = fut.await;
            println!("Response: {:?}", res);
            res
        })
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("*********testing logging layer*********");
    let anvil = Anvil::new().spawn();
    let rpc_url = anvil.endpoint_url();
    let client = ClientBuilder::default().layer(LoggingLayer).http(rpc_url);
    let provider = ProviderBuilder::new().on_client(client);
    for _ in 0..10 {
        let _block_number = provider.get_block_number().into_future().await?;
    }
    Ok(())
}
