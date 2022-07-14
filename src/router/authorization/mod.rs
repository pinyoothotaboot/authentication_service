use actix_web::{get,post,put,patch, web, HttpResponse, Responder};

use crate::router::libs::domain::usecase::{
    verify_new_access_token
};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{Verify,Info};
use crate::router::libs::constant::{HTTP_BAD_REQUEST};

#[patch("/api/v1/authorization/{id}/")]
async fn renew_access_token(state: web::Data<AppState>,verify: web::Json<Verify>,info : web::Path<Info>) -> impl Responder {
    let result = verify_new_access_token(state,verify,info).await;
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

pub fn init(cfg :&mut web::ServiceConfig) {
    cfg.service(renew_access_token);
}