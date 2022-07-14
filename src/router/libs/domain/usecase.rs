use serde_json::Value;
use serde_json::json;
use actix_web::{web};
use crate::router::libs::constant::ERROR;
use crate::router::libs::constant::HTTP_BAD_REQUEST;
use crate::router::libs::constant::{HTTP_OK};
use crate::router::libs::domain::utilities::{packet};
use crate::router::libs::adapter::log::usecase::{add_log};
use crate::router::libs::constant::{SUCCESS};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{Mobile,Otp,Info,Login,Verify};
use crate::router::libs::domain::utilities::{
    validate_mobile,validate_login,validate_authorization
};
use crate::router::libs::domain::repository::{
    find_or_create_authentication,resend_otp,confirm_authentication,renew_access_token
};

pub fn get_home() -> Value {
    let reply = packet(
        "Hello,I'am authentication service".to_string(),
        [].to_vec(), 
        HTTP_OK, 
        true
    );
    return reply;
}

pub async fn create_new_authentication(state: web::Data<AppState>,mobile : web::Json<Mobile>) -> Result<Value,Value>  {
    let validate = validate_mobile(&mobile);
    if !validate.0 {
        return validate.1;
    }
    
    let reply = find_or_create_authentication(state, mobile).await;
    match reply["success"].as_bool() {
        Some(success) => {
            match success {
                true => {
                    let data : Result<Value,Value> = Ok(reply);
                    return data;
                },
                false => {
                    let data : Result<Value,Value> = Err(reply);
                    add_log(
                        "Cannot create or update authentication".to_string(),
                        "create_new_authentication()".to_string(),
                        ERROR
                    ).await;
                    return data;
                }
            }
        },
        None => {
            let data : Result<Value,Value> = Err(reply);
            add_log(
                "Create new authentication has problem".to_string(),
                "create_new_authentication()".to_string(),
                ERROR
            ).await;
            return data;
        }
    };
}

pub async fn resend_new_otp(state: web::Data<AppState>,mobile : web::Json<Mobile>,info : web::Path<Info>) -> Result<Value,Value> {
    let validate = validate_mobile(&mobile);
    if !validate.0 {
        return validate.1;
    }

    let reply = resend_otp(state, mobile, info).await;
    match reply["success"].as_bool() {
        Some(success) => {
            match success {
                true => {
                    let data : Result<Value,Value> = Ok(reply);
                    return data;
                },
                false => {
                    let data : Result<Value,Value> = Err(reply);
                    add_log(
                        "Cannot resend OTP".to_string(),
                        "resend_new_otp()".to_string(),
                        ERROR
                    ).await;
                    return data;
                }
            }
        },
        None => {
            let data : Result<Value,Value> = Err(reply);
            add_log(
                "Resend new OTP has problem".to_string(),
                "resend_new_otp()".to_string(),
                ERROR
            ).await;
            return data;
        }
    };
}

pub async fn submit_authentication(state: web::Data<AppState>,login : web::Json<Login>,info : web::Path<Info>) -> Result<Value,Value>  {
    let validate = validate_login(&login,&info);
    if !validate.0 {
        return validate.1;
    }
    
    let reply = confirm_authentication(state,login,info).await;
    match reply["success"].as_bool() {
        Some(success) => {
            match success {
                true => {
                    let data : Result<Value,Value> = Ok(reply);
                    return data;
                },
                false => {
                    let data : Result<Value,Value> = Err(reply);
                    add_log(
                        "Cannot login".to_string(),
                        "submit_authentication()".to_string(),
                        ERROR
                    ).await;
                    return data;
                }
            }
        },
        None => {
            let data : Result<Value,Value> = Err(reply);
            add_log(
                "Login has problem".to_string(),
                "submit_authentication()".to_string(),
                ERROR
            ).await;
            return data;
        }
    };
}

pub async fn verify_new_access_token(state: web::Data<AppState>,verify : web::Json<Verify>,info : web::Path<Info>) -> Result<Value,Value>  {
    let validate = validate_authorization(&verify,&info);
    if !validate.0 {
        return validate.1;
    }

    let reply = renew_access_token(state,verify,info).await;
    match reply["success"].as_bool() {
        Some(success) =>{
            match success {
                true => {
                    let data : Result<Value,Value> = Ok(reply);
                    return data;
                },
                false => {
                    let data : Result<Value,Value> = Err(reply);
                    add_log(
                        "Cannot verify authorization".to_string(),
                        "verify_new_access_token()".to_string(),
                        ERROR
                    ).await;
                    return data;
                }
            }
        },
        None => {
            let data : Result<Value,Value> = Err(reply);
            add_log(
                "Verify authorization has problem".to_string(),
                "verify_new_access_token()".to_string(),
                ERROR
            ).await;
            return data;
        }
    };
}