#![feature(impl_trait_in_assoc_type)]

use std::net::SocketAddr;
use hw5::FilterLayer;
use hw5::LogLayer;
use hw5::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "[::]:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::volo::example::RedisServiceServer::new(S::new())
        .layer_front(LogLayer)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
