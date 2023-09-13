namespace rs volo.example

enum RequestType {
    GET,
    SET,
    DEL,
    PING,
    SUBSCRIBE,
    PUBLISH,
}

struct RedisRequest {
    1: required RequestType req_type,
    2: optional string key,
    3: optional string value,
    4: optional i32 expire_time, 
}

enum ResponseType {
    Done,
    Trapped,
}

struct RedisResponse {
    1: required ResponseType resp_type,
    2: optional string value, 
}

service RedisService {
    RedisResponse RedisCommand(1: RedisRequest req),
}

