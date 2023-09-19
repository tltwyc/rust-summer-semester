use lazy_static::lazy_static;
use pilota::lazy_static;
use std::net::SocketAddr;
use volo_gen::volo::example::GetItemRequest;
lazy_static! {
    static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 1 {
            tracing::error!("No Argument for Address!");
        }
        let addr = format!("127.0.0.1:{}", args.last().unwrap());
        // println!("formed addr: {}", addr);
        let addr: SocketAddr = addr.parse().unwrap();
        volo_gen::volo::example::ItemServiceClientBuilder::new(args[1].clone())
            .address(addr)
            .build()
    };
}

// args: [command[]..] [port]

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let mut args: Vec<String> = std::env::args().collect();
    let mut req = GetItemRequest {op: " ".into(), key: " ".into(), val: " ".into()};
    let opcode = args.remove(1).clone().to_lowercase().to_string();
    match opcode.as_str() {
        "set" => {
            req = GetItemRequest {
                op: "set".into(),
                key: args.remove(1).clone().into(),
                val: args.remove(1).clone().into(),

            };
            // println!("You are going to set {} to {}", req.clone().key, req.clone().val);
        }
        "get" => {
            req = GetItemRequest {
                op: "get".into(),
                key: args.remove(1).clone().into(),
                val: " ".into(),
            };
        }
        "del" => {
            req = GetItemRequest {
                op: "del".into(),
                key: args.remove(1).clone().into(),
                val: " ".into(),
            };
        }
        "ping" => {
            req = GetItemRequest {
                op: "ping".into(),
                key: " ".into(),
                val: " ".into(),
            };
            //println!("requeset ping!");
        }
        "subscribe" => {
            req = GetItemRequest {
                op: "subscribe".into(),
                key: args.remove(1).clone().into(),
                val: " ".into(),
            };
        }
        "publish" => {
            req = GetItemRequest {
                op: "publish".into(),
                key: args.remove(1).clone().into(),
                val: args.remove(1).clone().into(),
            }
        }
        _ => {
            println!("ILLEGAL!");
        }
    }
    println!("request got sent!");
    let resp = CLIENT.get_item(req).await;
    println!("responsed!");
    match resp {
        Ok(info)=>{
            match info.op.as_str() {
                "set" => {
                    match info.status {
                        true => {
                            println!("set success");
                        }
                        false => {
                            println!("set failed");
                        }
                    }
                }
                "get" => {
                    match info.status {
                        true => {
                            println!("get success, the value is {}", info.val);
                        }
                        false => {
                            println!("not found");
                        }
                    }
                }
                "del" => {
                    match info.status {
                        true => {
                            println!("deleted");
                        }
                        false => {
                            println!("delete failed");
                        }
                    }
                }
                "ping" => {
                    match info.status {
                        true => {
                            println!("pong!");
                        }
                        false => {
                            println!("failed....");
                        }
                    }
                }
                "subscribe" => {
                    match info.status {
                        true => {
                            println!("{} is published", info.val);
                        }
                        false => {
                            println!("not published......");
                        }
                    }
                }
                "publish" => {
                    match info.status {
                        true => {
                            println!("the number is {}", info.val);
                        }
                        false => {
                            println!("not found!");
                        }
                    }
                }
                _ => {
                    println!("invalid operation!");
                }
            }
        },
        Err(e) => tracing::error!("{:?}", e),
    }
}
