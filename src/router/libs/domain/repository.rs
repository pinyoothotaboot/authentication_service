use serde_json::Value;
use serde_json::json;
use actix_web::{web};
use crate::router::libs::constant::ERROR;
use crate::router::libs::constant::{
    HTTP_BAD_REQUEST,HTTP_CREATED,HTTP_NOT_FOUND
};
use crate::router::libs::constant::{HTTP_OK,EVENT_NEW_OTP};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{Mobile,Otp,Event,Authentication,State,Info,Login,Verify};
use crate::router::libs::adapter::mongo::{AuthenCollection};
use crate::router::libs::domain::utilities::{
    generate_otp,expire_time,create_new_authentication,packet,
    update_authentication,get_id,verify_login,update_token
};
use crate::router::libs::adapter::kafka::{MessageBroker};
use crate::router::libs::domain::jwt::{encode_token,validate_token};

pub async fn find_or_create_authentication(state: web::Data<AppState>,mobile : web::Json<Mobile>) -> Value {
    let mutex = state.clone();
    let lock;
    loop {
        match mutex.rl.lock(&mobile.mobile_number.as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let repo = AuthenCollection::new(state.clone()).await;
    let authen = repo.from_id(&mobile.mobile_number).await;
    let new_otp = generate_otp().await;
    let expired = expire_time(5);

    match authen {
        Ok(result) => {
            let current_version = result.version.clone();
            let update_authen = update_authentication(current_version.clone(),result.clone(),new_otp,expired.clone());
            let do_save = repo.save(update_authen.clone(),current_version).await;
            mutex.rl.unlock(&lock);

            match do_save {
                true => {
                    let resp = json!({
                        "id" : result.id,
                        "mobile" : mobile.mobile_number.to_string(),
                        "expired" : expired
                    });
        
                    let reply = packet(
                        "Update authentication successfully".to_string(),
                        [resp].to_vec(), 
                        HTTP_OK, 
                        true
                    );

                    let broker = MessageBroker::new(state).await;
                    broker.produce(update_authen, EVENT_NEW_OTP.to_string()).await;
                    return reply;
                },
                false => {
                    let reply = packet(
                        "Cannot update authentication!.".to_string(),
                        [].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return reply;
                }
            }
        },
        Err(_e) => {
            let id = repo.next_indentity().await;
            let new_authen = create_new_authentication(id.clone(), new_otp, mobile.mobile_number.clone().to_string(), expired.clone());
            let current_version  = new_authen.version.clone();
            let do_save = repo.save(new_authen.clone(),current_version).await;
            mutex.rl.unlock(&lock);
            
            match do_save {
                true => {
                    let resp = json!({
                        "id" : id,
                        "mobile" : mobile.mobile_number.to_string(),
                        "expired" : expired
                    });
        
                    let reply = packet(
                        "Create new authentication successfully".to_string(),
                        [resp].to_vec(), 
                        HTTP_CREATED, 
                        true
                    );

                    let broker = MessageBroker::new(state).await;
                    broker.produce(new_authen, EVENT_NEW_OTP.to_string()).await;
                    return reply;
                },
                false => {
                    let reply = packet(
                        "Cannot create new authentication!.".to_string(),
                        [].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return reply;
                }
            }
        }
    };
}

pub async fn resend_otp(state: web::Data<AppState>,mobile : web::Json<Mobile>,info : web::Path<Info>) -> Value {
    let mutex = state.clone();
    let lock;
    loop {
        match mutex.rl.lock(&mobile.mobile_number.as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let repo = AuthenCollection::new(state.clone()).await;
    let authen = repo.from_id(&mobile.mobile_number).await;
    let new_otp = generate_otp().await;
    let expired = expire_time(5);

    match authen {
        Ok(result) => {
            let current_version = result.version.clone();
            let update_authen = update_authentication(current_version.clone(),result.clone(),new_otp,expired.clone());
            let do_save = repo.save(update_authen.clone(),current_version).await;
            mutex.rl.unlock(&lock);

            match do_save {
                true => {
                    let resp = json!({
                        "id" : result.id,
                        "mobile" : mobile.mobile_number.to_string(),
                        "expired" : expired
                    });
        
                    let reply = packet(
                        "Resend OTP successfully".to_string(),
                        [resp].to_vec(), 
                        HTTP_OK, 
                        true
                    );

                    let broker = MessageBroker::new(state).await;
                    broker.produce(update_authen, EVENT_NEW_OTP.to_string()).await;
                    return reply;
                },
                false => {
                    let reply = packet(
                        "Cannot resend OTP!.".to_string(),
                        [].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return reply;
                }
            }
        },
        Err(_e) => {
            mutex.rl.unlock(&lock);
            let resp = json!({
                "id" : get_id(&info),
                "mobile" : mobile.mobile_number.to_string()
            });

            let reply = packet(
                "Not found data".to_string(),
                [resp].to_vec(), 
                HTTP_NOT_FOUND, 
                false
            );
            return reply;
        }
    };
}

pub async fn confirm_authentication(state: web::Data<AppState>,login : web::Json<Login>,info : web::Path<Info>) -> Value {
    let mutex = state.clone();
    let lock;
    loop {
        match mutex.rl.lock(&login.mobile_number.as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let repo = AuthenCollection::new(state.clone()).await;
    let authen = repo.from_id(&login.mobile_number).await;

    match authen {
        Ok(result) => {
            let verify = verify_login(&result,&login,&info);
            match verify.0 {
                true => {
                    let current_version = result.version.clone();
                    let id = get_id(&info);
                    let tokens = encode_token(id.clone(),login.role.clone().to_string()).await;
                    let updated_authen = update_token(current_version,result,tokens.0.clone().to_string(),tokens.1.to_string(),login.role.to_string());
                    let do_save = repo.save(updated_authen.clone(),current_version).await;
                    mutex.rl.unlock(&lock);
                    match do_save {
                        true => {
                            let resp = json!({
                                "id" : id,
                                "access_token" : tokens.0.to_string(),
                            });
                
                            let reply = packet(
                                "Login successfully".to_string(),
                                [resp].to_vec(), 
                                HTTP_OK, 
                                true
                            );
                            return reply;
                        },
                        false => {
                            let resp = json!({
                                "id" : id,
                                "mobile" : login.mobile_number.to_string()
                            });
                
                            let reply = packet(
                                "Cannot login failed!.".to_string(),
                                [resp].to_vec(), 
                                HTTP_NOT_FOUND, 
                                false
                            );
                            return reply;
                        }
                    }
                },
                false => {
                    mutex.rl.unlock(&lock);
                    return verify.1;
                }
            }
        },
        Err(_e) => {
            mutex.rl.unlock(&lock);
            let resp = json!({
                "id" : get_id(&info),
                "mobile" : login.mobile_number.to_string()
            });

            let reply = packet(
                "Not found data".to_string(),
                [resp].to_vec(), 
                HTTP_NOT_FOUND, 
                false
            );
            return reply;
        }
    };
}

pub async fn renew_access_token(state: web::Data<AppState>,verify : web::Json<Verify>,info : web::Path<Info>) -> Value {
    let mutex = state.clone();
    let lock;
    loop {
        match mutex.rl.lock(&verify.mobile_number.as_bytes(), 1000) {
            Some(l) => {
                lock = l;
                break
            },
            None => ()
        }
    }

    let access_token = verify.access_token.clone();
    let id = get_id(&info);
    let resp = json!({
        "id" : id.clone(),
        "mobile" : verify.mobile_number.to_string()
    });
    let validate_access_token = validate_token(access_token.clone(),resp.clone()).await;
    match validate_access_token.0 {
        true => {
            mutex.rl.unlock(&lock);
            let resp = json!({
                "id" : id,
                "access_token" : access_token.to_string(),
            });
            let reply = packet(
                "Access token has not expired!.".to_string(),
                [resp].to_vec(), 
                HTTP_OK, 
                true
            );
            return reply;
        },
        false => {
            match validate_access_token.1["code"].as_u64() {
                Some(status) => {
                    if status != HTTP_NOT_FOUND {
                        mutex.rl.unlock(&lock);
                        return validate_access_token.1;
                    } else {
                        let repo = AuthenCollection::new(state.clone()).await;
                        let authorize = repo.from_id(&verify.mobile_number).await;
                        match authorize {
                            Ok(result) => {
                                let current_version = result.version.clone();
                                let role = result.state.role.clone();
                                let refresh_token = result.state.refresh_token.clone();
                                
                                let validate_refresh_token = validate_token(refresh_token,resp.clone()).await;
                                match validate_refresh_token.0 {
                                    true => {
                                        let tokens = encode_token(id.clone(),role.to_string()).await;
                                        let updated_authorize = update_token(current_version,result,tokens.0.clone().to_string(),tokens.1.to_string(),role.to_string());
                                        let do_save = repo.save(updated_authorize.clone(),current_version).await;
                                        mutex.rl.unlock(&lock);
                    
                                        match do_save {
                                            true => {
                                                let resp = json!({
                                                    "id" : id,
                                                    "access_token" : tokens.0.to_string(),
                                                });
                                    
                                                let reply = packet(
                                                    "Verify authorization successfully".to_string(),
                                                    [resp].to_vec(), 
                                                    HTTP_OK, 
                                                    true
                                                );
                                                return reply;
                                            },
                                            false => {
                                                let reply = packet(
                                                    "Cannot verify authorization failed!.".to_string(),
                                                    [resp].to_vec(), 
                                                    HTTP_NOT_FOUND, 
                                                    false
                                                );
                                                return reply;
                                            }
                                        }
                                    },
                                    false => {
                                        mutex.rl.unlock(&lock);
                                        return validate_refresh_token.1;
                                    }
                                }
                                
                            },
                            Err(_e) => {
                                mutex.rl.unlock(&lock);
                                let resp = json!({
                                    "id" : get_id(&info),
                                    "mobile" : verify.mobile_number.to_string()
                                });
                    
                                let reply = packet(
                                    "Not found data".to_string(),
                                    [resp].to_vec(), 
                                    HTTP_NOT_FOUND, 
                                    false
                                );
                                return reply;
                            }
                        }
                    
                    }
                },
                None => {
                    mutex.rl.unlock(&lock);
                    return validate_access_token.1;
                }
            }
        }
    };
}