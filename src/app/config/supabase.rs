use std::sync::Arc;
use postgrest::Postgrest;

pub fn init_supabase(url: &str, key: &str) -> Arc<Postgrest> {
    let client = Postgrest::new(url)
        .insert_header("apikey", key)
        .insert_header("Authorization", format!("Bearer {}", key));
    Arc::new(client)
}
