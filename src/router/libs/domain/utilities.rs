use actix_web::{web,Error};
use chrono::format::format;
use serde_json::Value;
use serde_json::json;
use std::time::{SystemTime,UNIX_EPOCH};
use rand::Rng;
use std::sync::{Arc, Mutex,MutexGuard};
use crate::router::libs::domain::model::{Mobile,Otp,State,Event,Authentication,Info,Login,Verify};
use crate::router::libs::domain::validator::{
    validate_mobile_number,validate_otp_code,validate_role,
    validate_token
};
use crate::router::libs::constant::{
    DATA_EMPTY,DATA_NOT_COLLECT_LENGTH,DATA_NOT_MATCHED,DATA_PASSED,
    DATA_NOT_NUMERIC,HTTP_BAD_REQUEST,HTTP_OK,EVENT_NEW_OTP
};

pub fn packet(message : String , data : Vec<Value> , status : u64,success : bool) -> Value {
    let reply : Value = json!({
        "code": status,
        "success": success,
        "payload" : {
            "message" : message,
            "data" : data
        }
    });
    return reply;
}

pub fn expire_time(offset : u64) -> u64 {
    let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

    return timestamp as u64 + 1000*60*offset;
}

pub async fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(0..999999);
    let s = format!("{:06}",number);
    return s.to_string();
}

pub fn validate_mobile(mobile : &web::Json<Mobile>) -> (bool,Result<Value,Value>) {
    let mobile_number = format!("{}", &mobile.mobile_number);
    let validate_mobile = validate_mobile_number(&mobile_number);
    if validate_mobile != DATA_PASSED {
        let message = match validate_mobile {
            DATA_EMPTY => "Mobile number input is empty!.".to_string(),
            DATA_NOT_COLLECT_LENGTH => "Mobile number has not collect length!.".to_string(),
            DATA_NOT_NUMERIC => "Mobile number has not numeric!.".to_string(),
            _ => "None data".to_string()
        };
        let result = packet(
            message,
            [].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        let data : Result<Value,Value> = Err(result);
        return (false,data);
    } else {
        let result = packet(
            "Validate mobile data passed".to_string(),
            [].to_vec(), 
            HTTP_OK, 
            true
        );
        let data : Result<Value,Value> = Err(result);
        return (true,data);
    }
}

pub fn validate_login(login : &web::Json<Login>,info : &web::Path<Info>) -> (bool,Result<Value,Value>) {
    let mobile_number = format!("{}",&login.mobile_number);
    let otp_code = format!("{}",&login.otp_code);
    let role = format!("{}",&login.role);

    let validate_mobile = validate_mobile_number(&mobile_number);
    let validate_otp = validate_otp_code(&otp_code);
    let validate_role = validate_role(&role);
    let id = get_id(info);
    let resp = json!({
        "id" : id,
        "mobile" : mobile_number,
    });

    if validate_mobile != DATA_PASSED {
        let message = match validate_mobile {
            DATA_EMPTY => "Mobile number input is empty!.".to_string(),
            DATA_NOT_COLLECT_LENGTH => "Mobile number has not collect length!.".to_string(),
            DATA_NOT_NUMERIC => "Mobile number has not numeric!.".to_string(),
            _ => "None data".to_string()
        };
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        let data : Result<Value,Value> = Err(result);
        return (false,data);
    }

    if validate_otp != DATA_PASSED {
        let message = match validate_otp {
            DATA_EMPTY => "OTP code input is empty!.".to_string(),
            DATA_NOT_COLLECT_LENGTH => "OTP code has not collect length!.".to_string(),
            DATA_NOT_NUMERIC => "OTP code has not numeric!.".to_string(),
            _ => "None data".to_string()
        };
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        let data : Result<Value,Value> = Err(result);
        return (false,data);
    }
    
    if validate_role != DATA_PASSED {
        let message = match validate_role {
            DATA_EMPTY => "Role input is empty!.".to_string(),
            _ => "Not matched role!.".to_string()
        };
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        let data : Result<Value,Value> = Err(result);
        return (false,data);
    }

    let result = packet(
        "Validate login data passed".to_string(),
        [resp].to_vec(), 
        HTTP_OK, 
        true
    );
    let data : Result<Value,Value> = Ok(result);
    return (true,data);
}

pub fn validate_authorization(verify : &web::Json<Verify>,info : &web::Path<Info>) -> (bool,Result<Value,Value>) {
    let mobile_number = format!("{}",&verify.mobile_number);
    let access_token = format!("{}",&verify.access_token);
    let id = get_id(info);
    let resp = json!({
        "id" : id,
        "mobile" : mobile_number,
    });

    let validate_mobile = validate_mobile_number(&mobile_number);
    let validate_access_token = validate_token(&access_token);

    if validate_mobile != DATA_PASSED {
        let message = match validate_mobile {
            DATA_EMPTY => "Mobile number input is empty!.".to_string(),
            DATA_NOT_COLLECT_LENGTH => "Mobile number has not collect length!.".to_string(),
            DATA_NOT_NUMERIC => "Mobile number has not numeric!.".to_string(),
            _ => "None data".to_string()
        };
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        let data : Result<Value,Value> = Err(result);
        return (false,data);
    }

    if validate_access_token != DATA_PASSED {
        let message = match validate_access_token {
            DATA_EMPTY => "Access token input is empty!".to_string(),
            _=> "None data".to_string()
        };
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        let data : Result<Value,Value> = Err(result);
        return (false,data);
    }

    let result = packet(
        "Validate authorization data passed".to_string(),
        [resp].to_vec(), 
        HTTP_OK, 
        true
    );
    let data : Result<Value,Value> = Ok(result);
    return (true,data);
}

fn id(info: &web::Path<Info>) -> Result<String, Error> {
    Ok(format!("{}", info.id))
}

pub fn get_id(info: &web::Path<Info>) -> String {
    let index_id = id(info);
    let id = match index_id {
        Ok(id) => id,
        Err(e) => "".to_string(),
    };
    return id;
}

pub fn create_new_authentication(id : String , opt_code : String , mobile : String  , expired : u64 ) -> Authentication {
    let version = 1;
    let event = Event {otp_code : opt_code.to_string() ,expired : expired , version : version , event_type : EVENT_NEW_OTP.to_string() };
    let created = expire_time(0);
    let state = State {role : "".to_string() , access_token : "".to_string(),refresh_token : "".to_string(),created : created , updated : created};
    let new_authen = Authentication {id : id,version : version, mobile_number : mobile.to_string(),events : vec![event] , state : state , deleted : false};
    return new_authen;
}

pub fn update_authentication(current_version : u64 , result : Authentication,opt_code : String , expired : u64) -> Authentication {
    let next_version = current_version + 1;
    let event = Event {otp_code : opt_code,expired : expired ,version : next_version,event_type : EVENT_NEW_OTP.to_string()};
    let mut events = result.events;
    events.push(event);
    let updated = expire_time(0);
    let state = State {
        role :result.state.role.to_string() ,
        access_token : result.state.access_token.to_string(),
        refresh_token : result.state.refresh_token.to_string(),
        created : result.state.created , 
        updated : updated
    };
    let new_updated_authen = Authentication {
        id : result.id , 
        version : next_version,
        mobile_number : result.mobile_number , 
        events : events , 
        state : state , 
        deleted : false
    };

    return new_updated_authen;
}

pub fn verify_login(result : &Authentication,login : &web::Json<Login>,info : &web::Path<Info>) -> (bool,Value) {
    let current_time = expire_time(0);
    let check_mobile_number = &result.mobile_number.to_string();
    let events = &result.events;
    let event = &events[events.len() -1];
    let check_otp_code = &event.otp_code.to_string();
    let check_expired = event.expired;

    let id = get_id(info);
    let resp = json!({
        "id" : id,
        "mobile" : login.mobile_number,
    });

    if current_time > check_expired {
        let message = "Timeout enter OTP!.".to_string();
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        return (false,result);
    }

    if check_mobile_number.to_string() != login.mobile_number.to_string() {
        let message = "Mobile number has not matched!.".to_string();
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        return (false,result);
    }

    if check_otp_code.to_string() != login.otp_code.to_string() {
        let message = "OTP code has not matched!.".to_string();
        let result = packet(
            message,
            [resp].to_vec(), 
            HTTP_BAD_REQUEST, 
            false
        );
        return (false,result);
    }

    let result = packet(
        "Verify login data passed".to_string(),
        [resp].to_vec(), 
        HTTP_OK, 
        true
    );
    return (true,result);
}

pub fn update_token(current_version : u64 , result : Authentication,access_token : String , refresh_token : String,role : String) -> Authentication {
    let next_version = current_version + 1;
    let events = result.events;
    let updated = expire_time(0);
    let state = State {
        role :role.to_string() ,
        access_token : access_token.to_string(),
        refresh_token : refresh_token.to_string(),
        created : result.state.created , 
        updated : updated
    };
    let updated_authen = Authentication {
        id : result.id , 
        version : next_version,
        mobile_number : result.mobile_number , 
        events : events , 
        state : state , 
        deleted : false
    };
    return updated_authen;
}