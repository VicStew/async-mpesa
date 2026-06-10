use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{RegisterPullRequest, RegisterPullResponse}
};

/// Client to call the RegisterPull API
pub struct RegisterPull<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> RegisterPull<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: RegisterPullRequest,
    ) -> Result<RegisterPullResponse, MpesaError> {
        self.client.post("/pulltransactions/v1/register", request).await
    }
}
