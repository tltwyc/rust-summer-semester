#![feature(impl_trait_in_assoc_type)]

use std::{sync::Mutex, collections::HashMap};
use anyhow::Ok;
use volo_gen::volo::example::{RequestType, RedisResponse, ResponseType};

pub struct S {
	kvtbl: Mutex<HashMap<String, String>>
}

impl S {
	pub fn new() -> Self {
		S { kvtbl: Mutex::new(HashMap::new()) }
	}
}

#[volo::async_trait]
impl volo_gen::volo::example::RedisService for S {
	async fn redis_command(
		&self, 
		_req: volo_gen::volo::example::RedisRequest
	) -> ::core::result::Result<volo_gen::volo::example::RedisResponse, ::volo_thrift::AnyhowError>
	{
		match _req.req_type {
			RequestType::Set => {
				self.kvtbl.lock().unwrap().insert(
					_req.key.unwrap().into_string(), 
					_req.value.unwrap().into_string()
				);
				return Ok(RedisResponse {
					resp_type: ResponseType::Done,
					value: Some("OK".into())
				})
			}
			RequestType::Get => {
				match self.kvtbl.lock().unwrap().get(
					&_req.key.unwrap().into_string()
				) {
					Some(val) => {
						return Ok(RedisResponse {
							resp_type: ResponseType::Done,
							value: Some(val.clone().into())
						});
					}
					None => {
						return Ok(RedisResponse {
							resp_type: ResponseType::Done,
							value: Some("(nil)".into())
						});
					}
				}
			}
			RequestType::Del => {
				match self.kvtbl.lock().unwrap().remove(
					&_req.key.unwrap().into_string()
				) {
					Some(_val) => {
						return Ok(RedisResponse {
							resp_type: ResponseType::Done,
							value: Some("(integer) 1".into())
						});
					}
					None => {
						return Ok(RedisResponse {
							resp_type: ResponseType::Done,
							value: Some("(integer) 0".into())
						});
					}
				}
			}
			RequestType::Ping => {
				return Ok(RedisResponse{
					resp_type: ResponseType::Done,
					value: match _req.value {
						Some(val) => {
							Some(val)
						}
						None => {
							Some("PONG".into())
						}
					}
				});
			}
			RequestType::Publish => {

			}
			RequestType::Subscribe => {

			}
		}
		Ok(Default::default())
	}
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}

#[derive(Clone)]
pub struct FilterService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for FilterService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug,
	anyhow::Error: Into<S::Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
		let info = format!("{:?}", req);
		if info.contains("rust") {
			Err(anyhow::anyhow!("\"rust\" is filtered.").into())
		} else {
			self.0.call(cx, req).await
		}
	}
}

pub struct FilterLayer;

impl<S> volo::Layer<S> for FilterLayer {
	type Service = FilterService<S>;

	fn layer(self, inner: S) -> Self::Service {
		FilterService(inner)
	}
}