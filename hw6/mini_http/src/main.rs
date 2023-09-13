use std::net::SocketAddr;
use std::sync::Arc;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Form, Router,
};
const DEFAULT_ADDR: &str = "127.0.0.1:8080";
use faststr::FastStr;
use serde::Deserialize;
use volo_gen::volo::example::{RedisServiceClient, RedisServiceClientBuilder, RedisRequest};

type RpcClient = RedisServiceClient;
type RpcClientBuilder = RedisServiceClientBuilder;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = DEFAULT_ADDR.parse().unwrap();
    let rpc_cli = RpcClientBuilder::new("rpcdemo").address(addr).build();

    // build the application with router
    let app = Router::new()
        .route("/ping/:keys", get(ping_key).with_state(rpc_cli.clone()))
        .route("/ping", get(ping))
        .route("/get/:keys", get(get_key).with_state(rpc_cli.clone()))
        .route(
            "/set",
            get(show_set_form).post(set_key).with_state(rpc_cli.clone()),
        )
        .route("/del", get(show_del_form).post(del_key).with_state(rpc_cli));

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ping() -> (StatusCode, &'static str) {
    (StatusCode::OK, "pong")
}

async fn ping_key(
    Path(key): Path<String>, 
    State(rpc_cli): State<RpcClient>
) -> Response {
    let res = rpc_cli.redis_command(
        RedisRequest {
            req_type: volo_gen::volo::example::RequestType::Ping,
            key: None,
            value: Some(FastStr::from(Arc::new(key))),
            expire_time: None
        }
    ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

/// Get a key
async fn get_key(Path(key): Path<String>, State(rpc_cli): State<RpcClient>) -> Response {
    let res = rpc_cli.redis_command(
        RedisRequest {
            req_type: volo_gen::volo::example::RequestType::Get,
            key: Some(FastStr::from(Arc::new(key))),
            value: None,
            expire_time: None
        }
    ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

#[derive(Deserialize, Debug)]
struct FormKey {
    key: String,
    value: String
}

/// Show the form for set a key
async fn show_set_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/set" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <label for="key">
                        Enter value:
                        <input type="text" name="value">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

/// Set a key
async fn set_key(State(rpc_cli): State<RpcClient>, Form(setkey): Form<FormKey>) -> Response {
    let res = rpc_cli.redis_command(
        RedisRequest {
            req_type: volo_gen::volo::example::RequestType::Set,
            key: Some(FastStr::from(Arc::new(setkey.key))),
            value: Some(FastStr::from(Arc::new(setkey.value))),
            expire_time: None
        }
    ).await;
    match res {
        Ok(v) => {
            (StatusCode::OK, v.value.unwrap().to_string()).into_response()
        },
        Err(e) => {
            (StatusCode::NOT_FOUND, e.to_string()).into_response()
        }
    }
}

async fn show_del_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/del" method="post">
                    <label for="key">
                        Enter key:
                        <input type="text" name="key">
                    </label>
                    <input type="submit" value="Subscribe!">
                </form>
            </body>
        </html>
        "#,
    )
}

#[derive(Deserialize, Debug)]
struct FormKey2 {
    key: String
}

async fn del_key(
    State(rpc_cli): State<RpcClient>,
    Form(delkey): Form<FormKey2>,
) -> (StatusCode, &'static str) {
    let res = rpc_cli.redis_command(
        RedisRequest {
            req_type: volo_gen::volo::example::RequestType::Del,
            key: Some(FastStr::from(Arc::new(delkey.key))),
            value: None,
            expire_time: None
        }
    ).await;
    match res {
        Ok(_v) => {
            (StatusCode::OK, "Successful delete")
        },
        Err(_e) => {
            (StatusCode::NOT_FOUND, "Delete error")
        }
    }
}