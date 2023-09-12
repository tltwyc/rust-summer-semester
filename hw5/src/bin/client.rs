use lazy_static::lazy_static;
use volo_gen::volo::example::{RedisRequest, RequestType};
use std::{net::SocketAddr, io::Write};
use hw5::{LogLayer, FilterLayer};

lazy_static! {
    static ref CLIENT: volo_gen::volo::example::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::volo::example::RedisServiceClientBuilder::new("mini-redis")
            .layer_outer(LogLayer)
            .layer_outer(FilterLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    // let mut args: Vec<String> = env::args().collect();
    // let mut buf: String = String::with_capacity(1024);
    loop {
        print!("mini-redis >> ");
        let _ = std::io::stdout().flush();
        // buf.clear();
        let mut tmp_line = String::new();
        std::io::stdin().read_line(&mut tmp_line).unwrap();
        let tmp_line : Vec<&str> = tmp_line.trim().split_whitespace().filter(|s| !s.is_empty()).collect();
        let mut cmd_line: Vec<String> = vec![];
        for w in tmp_line {
            cmd_line.push(w.to_string());
        }
        if cmd_line.is_empty() {
            continue;
        }
        let req = match cmd_line[0].to_lowercase().as_str() {
            "get" => {
                if cmd_line.len() != 2 {
                    prt_arg_num_err();
                    continue;
                }
                let key = cmd_line.get(1).unwrap().clone();
                RedisRequest {
                    req_type: RequestType::Get,
                    key: Some(key.into()),
                    value: None,
                    expire_time: None
                }                
            }
            "set" => {
                if cmd_line.len() != 3 {
                    prt_arg_num_err();
                    continue;
                }
                let key = cmd_line.get(1).unwrap().clone();
                let val = cmd_line.get(2).unwrap().clone();
                RedisRequest {
                    req_type: RequestType::Set,
                    key: Some(key.into()),
                    value: Some(val.into()),
                    expire_time: None
                }
            }
            "ping" => {
                if cmd_line.len() > 2 {
                    prt_arg_num_err();
                    continue;
                }
                RedisRequest {
                    req_type: RequestType::Ping,
                    key: None,
                    value: if cmd_line.len() == 2 {
                        Some(cmd_line.get(1).unwrap().clone().into())
                    } else {None},
                    expire_time: None
                }
            }
            "del" => {
                if cmd_line.len() != 2 {
                    prt_arg_num_err();
                    continue;
                }
                let key = cmd_line.get(1).unwrap().clone();
                RedisRequest {
                    req_type: RequestType::Del,
                    key: Some(key.into()),
                    value: None,
                    expire_time: None
                }        
            }
            "publish" => {
                RedisRequest {
                    req_type: RequestType::Publish,
                    key: None,
                    value: None,
                    expire_time: None
                }
            }
            "subscribe" => {
                RedisRequest {
                    req_type: RequestType::Subscribe,
                    key: None,
                    value: None,
                    expire_time: None
                }
            }
            "quit" => {
                println!("Bye!");
                break;
            }
            _ => {
                println!("Command not Found!");
                continue;
            }
        };
        let resp = CLIENT.redis_command(req).await;
        match resp {
            Ok(info) => tracing::info!("{:?}", info.value.unwrap()),
            Err(e) => {
                match e {
                    volo_thrift::ResponseError::Application(err) => {
                        tracing::error!("{}", err.message)
                    }
                    _ => {
                        tracing::error!("{:?}", e)
                    }
                }
            }
        } 

    }
    
       
}

pub fn prt_arg_num_err() {
    println!("Error: Num of Arguments not Correct!");
}