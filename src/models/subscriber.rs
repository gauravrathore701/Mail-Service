use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub client_id: String,
}
