use actix_web::{get,post,put,patch, web, HttpResponse, Responder};

use crate::router::libs::domain::usecase::{
    create_new_authentication,resend_new_otp,
    submit_authentication
};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{Mobile,Info,Login};
use crate::router::libs::constant::{HTTP_BAD_REQUEST,HTTP_OK,HTTP_CREATED};

#[post("/api/v1/authentication/")]
async fn create_authen(state: web::Data<AppState>,mobile : web::Json<Mobile>) -> impl Responder {
    let result = create_new_authentication(state,mobile).await;
    match result {
        Ok(reply) => {
            match reply["code"].as_u64() {
                Some(_code) => {
                    if _code == HTTP_OK {
                        HttpResponse::Ok().json(reply)
                    }else {
                        HttpResponse::Created().json(reply)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(e) => {
            match e["code"].as_u64() {
                Some(_code) => {
                    if _code == HTTP_BAD_REQUEST {
                        HttpResponse::BadRequest().json(e)
                    } else {
                        HttpResponse::NotFound().json(e)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[patch("/api/v1/authentication/{id}/")]
async fn resend_otp(state: web::Data<AppState>,mobile : web::Json<Mobile>,info : web::Path<Info>) -> impl Responder {
    let result = resend_new_otp(state,mobile,info).await;
    match result {
        Ok(reply) => HttpResponse::Ok().json(reply),
        Err(e) => {
            match e["code"].as_u64() {
                Some(_code) => {
                    if _code == HTTP_BAD_REQUEST {
                        HttpResponse::BadRequest().json(e)
                    } else {
                        HttpResponse::NotFound().json(e)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

#[put("/api/v1/authentication/{id}/")]
async fn confirm_authen(state: web::Data<AppState>,login : web::Json<Login>,info : web::Path<Info>) -> impl Responder {
    let result = submit_authentication(state,login,info).await;
    match result {
        Ok(reply) => HttpResponse::Created().json(reply),
        Err(e) => {
            match e["code"].as_u64() {
                Some(_code) => {
                    if _code == HTTP_BAD_REQUEST {
                        HttpResponse::BadRequest().json(e)
                    } else {
                        HttpResponse::NotFound().json(e)
                    }
                },
                None => {
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
    }
}

pub fn init(cfg :&mut web::ServiceConfig) {
    cfg.service(create_authen);
    cfg.service(resend_otp);
    cfg.service(confirm_authen);
}