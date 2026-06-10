use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{PullTransactionsRequest, PullTransactionsResponse}
};

/// Client to call the PullTransactions API
pub struct PullTransactions<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> PullTransactions<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: PullTransactionsRequest,
    ) -> Result<PullTransactionsResponse, MpesaError> {
        self.client.post("/pulltransactions/v1/register", request).await
    }
}
