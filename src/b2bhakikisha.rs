use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{B2bHakikishaRequest, B2bHakikishaResponse}
};

/// Client to call the B2b Hakikisha API
/// This API returns the name and applicable tariff of an M-Pesa organization account. This information can be used to ensure the target B2B transaction recipient is correct, and help account for applicable transaction fees before hand.
pub struct B2bHakikisha<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> B2bHakikisha<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: B2bHakikishaRequest,
    ) -> Result<B2bHakikishaResponse, MpesaError> {
        self.client.post("/sfcverify/v1/query/info", request).await
    }
}
