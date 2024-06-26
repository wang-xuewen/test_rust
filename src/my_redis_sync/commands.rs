use super::RUDIS_DB;
use resp::Value;
use crate::log_a;

pub fn process_client_request(decoded_msg: Value) -> Vec<u8> {
    let reply = if let Value::Array(v) = decoded_msg {
        match &v[0] {
            Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
            Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
            // Value::Bulk(ref s) if s == "COMMAND" || s == "command" => Ok(Value::Null),
            other => unimplemented!("{:?} is not supported as of now", other),
        }
    } else {
        Err(Value::Error("Invalid Command".to_string()))
    };

    match reply {
        Ok(r) | Err(r) => r.encode(),
    }

    // match reply {
    //     Ok(r) => match r {
    //         Value::Null => Vec::new(),
    //         _ => r.encode(),
    //     },
    //     Err(r) => {
    //         // 当 reply 是 Err 时的处理逻辑
    //         // 这里可以是你之前已经实现的逻辑
    //         r.encode()
    //     }
    // }
}

pub fn handle_get(v: Vec<Value>) -> Result<Value, Value> {

    let cmd = &v[0];

    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() {
        return Err(Value::Error(
            "Expected 1 argument for GET command".to_string(),
        ));
    }

    log_a!("handle_get {:?} {:?}",cmd,&v[0]);

    let db_ref = RUDIS_DB.lock().unwrap();
    let reply = if let Value::Bulk(ref s) = &v[0] {
        db_ref
            .get(s)
            .map(|e| Value::Bulk(e.to_string()))
            .unwrap_or(Value::Null)
    } else {
        Value::Null
    };
    Ok(reply)
}

pub fn handle_set(v: Vec<Value>) -> Result<Value, Value> {

    let cmd = &v[0];

    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error(
            "Expected 2 arguments for SET command".to_string(),
        ));
    }

    log_a!("handle_set {:?} {:?} {:?}",cmd,&v[0],&v[1]);

    match (&v[0], &v[1]) {
        (Value::Bulk(k), Value::Bulk(v)) => {
            let _ = RUDIS_DB
                .lock()
                .unwrap()
                .insert(k.to_string(), v.to_string());
        }
        _ => unimplemented!("SET not implemented for {:?}", v),
    }
    Ok(Value::String("OK".to_string()))
}
