use reqwest::blocking::Client;

pub struct CritiqueClient {
    client: Client,
}

impl CritiqueClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_user_collection(&self, user_id: &str) -> Result<reqwest::Response, reqwest::Error> {
        todo!();
    }
}
