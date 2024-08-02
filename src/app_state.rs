#[derive(Clone, Debug)]
pub struct AppState {
    pub max_retries: u16,
    pub min_number: u8,
    pub access_token: Option<String>,
}
