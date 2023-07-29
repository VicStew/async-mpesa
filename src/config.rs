use reqwest::header::{ HeaderMap, AUTHORIZATION };

///Default v1 API base url
pub const MPESA_API_URL: &str = "https://sandbox.safaricom.co.ke/mpesa";

pub trait Config: Clone {
    fn headers(&self) -> HeaderMap;
    fn url(&self, path: &str) -> String;
    fn api_url(&self) -> &str;
    fn access_token(&self) -> &str;
}

///Configuration for Mpesa API
#[derive(Clone, Debug)]
pub struct MpesaConfig {
    api_url: String,
    access_token: String,
}

impl Default for MpesaConfig {
    fn default() -> Self {
        Self { 
            api_url: MPESA_API_URL.to_string(), 
            access_token: std::env::var("MPESA_ACCESS_TOKEN").unwrap_or_else(|_| "".to_string())
        }
    }
}

impl MpesaConfig {
    /// Creates a client with default [MPESA_API_URL] and default API key from the MPESA_ACCESS_TOKEN env var
    pub fn new() -> Self {
        Default::default()
    }

    /// To use a different API key different from the default MPESA_ACCESS_TOKEN env var
    pub fn with_access_token<S: Into<String>>(mut self, access_token: S) -> Self {
        self.access_token = access_token.into();
        self
    }

    /// To use an API_URL different from the default [MPESA_API_URL]
    pub fn with_api_url<S: Into<String>>(mut self, api_url: S) -> Self {
        self.api_url = api_url.into();
        self
    }
}

impl Config for MpesaConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        headers.insert(
            AUTHORIZATION, 
            format!("Bearer {}", self.access_token).as_str().parse().unwrap(),
        );

        headers
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", MPESA_API_URL, path)
    }

    fn api_url(&self) -> &str {
        &self.api_url
    }

    fn access_token(&self) -> &str {
        &self.access_token
    }
}
