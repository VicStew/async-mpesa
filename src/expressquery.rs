use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{ExpressQueryRequest, ExpressQueryResponse}
};

/// Creates a request amd returns a response
pub struct ExpressQuery<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> ExpressQuery<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: ExpressQueryRequest,
    ) -> Result<ExpressQueryResponse, MpesaError> {
        self.client.post("/stkpushquery/v1/query", request).await
    }
}