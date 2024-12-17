use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub email: String,
    pub full_name: Option<String>,
}

impl User {
    // Helper functions for interacting with the database
    pub async fn create_user(client: &supabase::Client, user: &User) -> Result<(), String> {
        let response = client
            .from("users")
            .insert(vec![user])
            .execute()
            .await;

        response.map_err(|err| err.to_string())
    }

    pub async fn get_user_by_email(
        client: &supabase::Client,
        email: &str,
    ) -> Result<Option<User>, String> {
        let response = client
            .from("users")
            .eq("email", email)
            .select("*")
            .execute()
            .await;

        response
            .map_err(|err| err.to_string())
            .and_then(|res| res.json::<Vec<User>>().map(|users| users.into_iter().next()))
    }

    pub async fn update_user(
        client: &supabase::Client,
        email: &str,
        updated_user: &User,
    ) -> Result<(), String> {
        let response = client
            .from("users")
            .eq("email", email)
            .update(updated_user)
            .execute()
            .await;

        response.map_err(|err| err.to_string())
    }

    pub async fn delete_user(client: &supabase::Client, email: &str) -> Result<(), String> {
        let response = client
            .from("users")
            .eq("email", email)
            .delete()
            .execute()
            .await;

        response.map_err(|err| err.to_string())
    }
}
