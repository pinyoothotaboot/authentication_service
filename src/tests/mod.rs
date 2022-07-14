#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, web, App};
    use serde_json::Value;
    use serde_json::json;
    use dotenv::dotenv;
    use crate::router;
    use rand::Rng;
    use crate::router::libs::infrastructure::appstate::{AppState};
    use crate::router::libs::infrastructure::distributed_lock::{Locking};
    use crate::router::libs::infrastructure::kafka::{broker_producer};
    use crate::router::libs::infrastructure::mongo::{connect_mongo,connect_database};
    use crate::router::libs::domain::model::{Response};
    use crate::router::libs::constant::{HTTP_OK,HTTP_CREATED,HTTP_BAD_REQUEST};

    async fn app_state() -> AppState {
        dotenv().ok();
        let client = connect_mongo().await;
        let database = connect_database(client).await;
        let producer = broker_producer().await.expect("failed to create kafka producer");
        let rl = Locking().await;
        AppState {
            db : database , 
            producer : producer,
            rl:rl
        }
    }

    fn generate_mobile_number() -> String {
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen_range(0..999999999);
        let s = format!("0{:09}",number);
        return s.to_string();
    }

    fn get_mobile_number() -> Value {
        let new_mobile_number = generate_mobile_number();
        let data = json!({
            "mobile_number" : new_mobile_number
        });
        return data;
    }

    fn get_mobile_number_empty() -> Value {
        let data = r#"{
            "mobile_number" : ""
        }"#;
        return serde_json::from_str(data).unwrap();
    }

    fn get_incorect_mobile_number()-> Value {
        let data = json!({
            "mobile_number" : "088-123-abxz"
        });
        return data; 
    }

    fn get_less_mobile_number() -> Value {
        let data = json!({
            "mobile_number" : "0881821"
        });
        return data;
    }

    fn get_login_mobile_number(mobile_number : String , otp_code : String) -> Value {
        let data = json!(
            {
                "mobile_number" : mobile_number,
                "otp_code" : otp_code,
                "role": "customer"
            }
        );
        return data;
    }

    fn get_login_mobile_number_empty_data() -> Value {
        let data = json!(
            {
                "mobile_number" : "",
                "otp_code" : "",
                "role": ""
            }
        );
        return data;
    }

    fn get_login_mobile_number_incorrect_data() -> Value {
        let data = json!(
            {
                "mobile_number" : "088-182-1234",
                "otp_code" : "1234h6",
                "role": "Test"
            }
        );
        return data;
    }
    
    fn get_current_access_token(mobile_number : String) -> Value {
        let data = json!(
            {
                "mobile_number" : mobile_number.to_string(),
                "access_token" : "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI2MmJjNGYyN2VhY2VmZjQ0ZWRmNjZlNjEiLCJyb2xlIjoiY3VzdG9tZXIiLCJpYXQiOjE2NTY1NDgyNTIsImV4cCI6MTY1NjU0ODI1Mn0.3asAOdCdcAJwbDzQ-QctK8N3fq_hznsLdEM9WaGH7fg"
            }
        );
        return data;
    }

    fn get_current_access_token_empty() -> Value {
        let data = json!(
            {
                "mobile_number" : "",
                "access_token" : ""
            }
        );
        return data;
    }

    fn get_current_access_token_incorrect() -> Value {
        let data = json!(
            {
                "mobile_number" : "088-128-12ss",
                "access_token" : "Test access token"
            }
        );
        return data;
    }

    #[actix_rt::test]
    async fn test_get_home() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::home::init);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/api/v1").to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_OK as u16);
    }

    #[actix_rt::test]
    async fn test_new_authentication_with_data_passed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_CREATED as u16);
    }

    #[actix_rt::test]
    async fn test_new_authentication_with_data_empty_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number_empty();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
    }

    #[actix_rt::test]
    async fn test_new_authentication_with_data_incorrect_mobile_number_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_incorect_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
    }

    #[actix_rt::test]
    async fn test_new_authentication_with_data_less_mobile_number_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_less_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
    }

    #[actix_rt::test]
    async fn test_resend_new_otp_with_data_passed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.clone().to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["id"].as_str() {
            Some(id) => {
                let path = format!("/api/v1/authentication/{}/",id);
                let req = test::TestRequest::patch()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_OK as u16);
            },
            None => {
                panic!("Not found");
            }
        }
    }

    #[actix_rt::test]
    async fn test_resend_new_otp_with_empty_data_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.clone().to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["id"].as_str() {
            Some(id) => {
                let path = format!("/api/v1/authentication/{}/",id);
                let data = get_mobile_number_empty();
                let req = test::TestRequest::patch()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
            },
            None => {
                panic!("Not found");
            }
        }
    }

    #[actix_rt::test]
    async fn test_resend_new_otp_with_incorrect_mobile_number_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.clone().to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["id"].as_str() {
            Some(id) => {
                let path = format!("/api/v1/authentication/{}/",id);
                let data = get_incorect_mobile_number();
                let req = test::TestRequest::patch()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
            },
            None => {
                panic!("Not found");
            }
        }
    }

    #[actix_rt::test]
    async fn test_resend_new_otp_with_less_mobile_number_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.clone().to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["id"].as_str() {
            Some(id) => {
                let path = format!("/api/v1/authentication/{}/",id);
                let data = get_less_mobile_number();
                let req = test::TestRequest::patch()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
            },
            None => {
                panic!("Not found");
            }
        }
    }

    // TODO :: Login data passed wait mock

    #[actix_rt::test]
    async fn test_login_with_empty_data_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.clone().to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["id"].as_str() {
            Some(id) => {
                let path = format!("/api/v1/authentication/{}/",id);
                let data = get_login_mobile_number_empty_data();
                let req = test::TestRequest::put()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
            },
            None => {
                panic!("Not found");
            }
        }
    }

    #[actix_rt::test]
    async fn test_login_with_incorrect_data_failed() {
        let app = App::new()
            .data(app_state().await)
            .configure(router::authentication::init);
        let mut app = test::init_service(app).await;
        let data = get_mobile_number();
        let req = test::TestRequest::post()
        .uri("/api/v1/authentication/")
        .insert_header(ContentType::json())
        .set_payload(data.clone().to_string())
        .to_request();
        let res = test::call_service(&mut app,req).await;
        let reply : Response = test::read_body_json(res).await;
        let result  = json!(reply);
        match result["payload"]["data"][0]["id"].as_str() {
            Some(id) => {
                let path = format!("/api/v1/authentication/{}/",id);
                let data = get_login_mobile_number_incorrect_data();
                let req = test::TestRequest::put()
                .uri(&path)
                .insert_header(ContentType::json())
                .set_payload(data.to_string())
                .to_request();
                let res = test::call_service(&mut app,req).await;
                assert_eq!(res.status().as_u16(),HTTP_BAD_REQUEST as u16);
            },
            None => {
                panic!("Not found");
            }
        }
    }

    //TODO :: Wait login passed
    
}