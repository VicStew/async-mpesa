use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{AccountBalanceRequest, AccountBalanceResponse},
};

/// Will call the AccountBalance API
pub struct AccountBalance<'m, C: Config> {
    client: &'m Client<C>
}

impl <'m, C: Config> AccountBalance<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: AccountBalanceRequest,
    ) -> Result<AccountBalanceResponse, MpesaError> {
        self.client.post("/accountbalance/v1/query", request).await
    }
}