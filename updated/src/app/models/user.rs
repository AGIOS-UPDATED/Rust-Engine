use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
}

impl User {
    // Helper functions for interacting with the database
  
}
