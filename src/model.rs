use dotenv::dotenv;
use mongodb::{Client};
use juniper::{FieldResult, Context};

#[derive(Default, Clone)]
pub struct Database {}
impl Context for Database {}

impl Database {
    pub fn new() -> Database { 
        Database {      
        }
    }
    
    pub async fn connect(&self) -> FieldResult<Client> {
        dotenv().ok();
        let db_url = std::env::var("MONGODB_URL").expect("MONGODB_URL must be set");
        let client = Client::with_uri_str(&db_url).await?;
        return Ok(client);
    }
}
