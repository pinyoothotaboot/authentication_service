use actix_web::{web};
use dotenv::dotenv;
use std::env;
use std::str::FromStr;
use mongodb::{bson::oid::ObjectId,bson,options::FindOneAndUpdateOptions};
use bson::{doc, Bson, Document};
use crate::router::libs::infrastructure::appstate::AppState;
use crate::router::libs::domain::model::{Authentication};

pub struct AuthenCollection {
    pub client : web::Data<AppState>
}

impl AuthenCollection {
    pub async fn new (client : web::Data<AppState>) -> AuthenCollection {
        AuthenCollection {client}
    }

    pub async fn next_indentity(&self) -> String {
        let id = ObjectId::default().to_string();
        return id;
    }

    async fn doc_authentication(&self,entity : Authentication) -> Document {
        let value = doc! {
            "$set" : {
                "version": bson::to_bson(&entity.version).unwrap(),
                "id": bson::to_bson(&entity.id).unwrap() ,
                "mobile_number" : bson::to_bson(&entity.mobile_number).unwrap(),
                "events": bson::to_bson(&entity.events).unwrap(),
                "state": bson::to_bson(&entity.state).unwrap(),
                "deleted" : bson::to_bson(&entity.deleted).unwrap(),
            }
        };
        return value;
    }

    async fn doc_to_authentication(&self,doc : Result<Option<Authentication>, mongodb::error::Error>) -> Result<Authentication,&'static str> {
        let result = match doc {
            Ok(reply) => reply,
            Err(e) => None,
        };
        let all_order = if let Some(reply) = result {
            Ok(reply)
        } else {
            Err("Not found collection")
        };
        return all_order;
    }

    pub async fn from_id(&self,mobile : &String) -> Result<Authentication,&'static str> {
        dotenv().ok();
        let mongo_collection : String = env::var("MONGO_COLLECTION")
        .expect("Not found mongodb collection in .env file");
        let name = format!("{}", mongo_collection);
        let collection = self.client.db.collection::<Authentication>(&name);
        let filter = doc! {
            "mobile_number" : mobile,
            "deleted" : false
        };
        let doc: Result<Option<Authentication>, mongodb::error::Error> = collection.find_one(filter, None).await;
        let authen = self.doc_to_authentication(doc).await;
        return authen;
    }

    pub async fn save(&self,entity : Authentication,current_version : u64) -> bool  {
        dotenv().ok();
        let mongo_collection : String = env::var("MONGO_COLLECTION")
        .expect("Not found mongodb collection in .env file");
        let name = format!("{}", mongo_collection);
        let id  = entity.clone().id;
        let update = self.doc_authentication(entity).await;
        let filter = doc! {
            "id" : bson::to_bson(&id).unwrap(), 
            "version" : bson::to_bson(&current_version).unwrap()
        };
        let options = FindOneAndUpdateOptions::builder().upsert(true).build();
        let collection = self.client.db.collection::<Authentication>(&name);
        let result = collection.find_one_and_update(filter,update,options).await;
        
        match result {
            Ok(_reply) => {
                println!("Insert or Update order to database successfully");
                return true; 
            },
            Err(e) => {
                println!("Cannot insert or update order to database : {:?}",e);
                return false; 
            },
        }
    }
}