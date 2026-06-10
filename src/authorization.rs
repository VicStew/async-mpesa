use base64::{Engine, engine::general_purpose};

use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{AuthorizationRequest, AuthorizationResponse},
};

/// Client to call the authorization API.
pub struct Authorization<'m, C: Config> {
    client: &'m Client<C>
}

impl <'m, C: Config> Authorization<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: AuthorizationRequest,
    ) -> Result<AuthorizationResponse, MpesaError> {
        let encoded_auth = general_purpose::URL_SAFE.encode(format!("{}:{}", request.ConsumerKey, request.ConsumerSecret));
        self.client.get("/oauth/v1/generate?grant_type=client_credentials", encoded_auth).await
    }
}
