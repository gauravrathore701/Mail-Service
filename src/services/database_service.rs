use mongodb::{Client, Collection};
use crate::models::subscriber::Subscriber;

#[derive(Clone)]
pub struct DatabaseService {
    subscriber_collection: Collection<Subscriber>,
}

impl DatabaseService {
    pub async fn new() -> Self {
        let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
        let client = Client::with_uri_str(uri).await.expect("Failed to connect to MongoDB");
        let db = client.database("notification_service");
        let subscriber_collection = db.collection("subscribers");
        
        Self { subscriber_collection }
    }

    pub async fn save_subscriber(&self, subscriber: Subscriber) -> Result<(), String> {
        match self.subscriber_collection.insert_one(subscriber).await {
            Ok(_) => {
                println!("Subscriber saved successfully");
                Ok(())
            }
            Err(e) => {
                println!("Error saving subscriber: {:?}", e);
                Err(e.to_string())
            }
        }
    }
}
