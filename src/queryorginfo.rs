use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{QueryOrgInfoRequest, QueryOrgInfoResponse}
};

/// Client to call the Query Org Info API
pub struct QueryOrgInfo<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> QueryOrgInfo<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: QueryOrgInfoRequest,
    ) -> Result<QueryOrgInfoResponse, MpesaError> {
        self.client.post("/sfcverify/v1/query/info", request).await
    }
}
