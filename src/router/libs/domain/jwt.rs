use dotenv::dotenv;
use std::env;
use serde_json::json;
use serde_json::Value;
use time::{Duration, OffsetDateTime};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::router::libs::domain::model::{Claims};
use crate::router::libs::constant::{
    EXPIRE_ACCESS_TOKEN,EXPIRE_REFRESH_TOKEN,
    HTTP_BAD_REQUEST,HTTP_OK,HTTP_NOT_FOUND
};
use crate::router::libs::domain::utilities::{packet};

async fn generate_access_token(sub : String,role : String,salt : String) -> String {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::days(EXPIRE_ACCESS_TOKEN);
    let claims = Claims::new(sub.clone(),role, iat, exp);
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(salt.as_ref()))
    .expect("Failed to encode claims");
    return token;
}

async fn generate_refresh_token(sub : String,role : String,salt : String) -> String {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + Duration::days(EXPIRE_REFRESH_TOKEN);
    let claims = Claims::new(sub.clone(),role, iat, exp);
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(salt.as_ref()))
    .expect("Failed to encode claims");
    return token;
}

pub async fn encode_token(id : String , role : String) -> (String,String) {
    dotenv().ok();
    let key : String = env::var("SECRET_KEY").expect("Not found secret key in .env file");
    let salt = format!("{}", key);
    let access_token = generate_access_token(id.clone(), role.clone(), salt.clone()).await;
    let refresh_token = generate_refresh_token(id.clone(), role.clone(), salt.clone()).await;
    return (access_token,refresh_token);
}

pub async fn validate_token(token : String,resp : Value) -> (bool,Value)  {
    dotenv().ok();
    let key : String = env::var("SECRET_KEY").expect("Not found secret key in .env file");
    let salt = format!("{}", key);
    match decode::<Claims>(&token,&DecodingKey::from_secret(salt.as_ref()), &Validation::new(Algorithm::HS256)) {
        Ok(_result) => {
            let reply = packet(
                "Validate token passed".to_string(),
                [resp].to_vec(), 
                HTTP_OK, 
                true
            );
            return (true,reply);
        },
        Err(e) => {
            match *e.kind() {
                ErrorKind::ExpiredSignature => {
                    let reply = packet(
                        "Token has expired!.".to_string(),
                        [resp].to_vec(), 
                        HTTP_NOT_FOUND, 
                        false
                    );
                    return (false,reply);
                },
                ErrorKind::InvalidToken =>{
                    let reply = packet(
                        "Token has invalid!.".to_string(),
                        [resp].to_vec(), 
                        HTTP_BAD_REQUEST, 
                        false
                    );
                    return (false,reply);
                },
                ErrorKind::InvalidIssuer => {
                    let reply = packet(
                        "Token has invalid issue!.".to_string(),
                        [resp].to_vec(), 
                        HTTP_BAD_REQUEST, 
                        false
                    );
                    return (false,reply);
                },
                _=> {
                    let reply = packet(
                        "Token not verify!.".to_string(),
                        [resp].to_vec(), 
                        HTTP_BAD_REQUEST, 
                        false
                    );
                    return (false,reply);
                }
            }
        }
    };
}