use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{CalculatePointsRequest, CalculatePointsResponse}
};

/// Client to call the Sim Calculate Bonga Points API
pub struct CalculatePoints<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> CalculatePoints<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: CalculatePointsRequest,
    ) -> Result<CalculatePointsResponse, MpesaError> {
        self.client.post("/v1/lipa/na/bonga/calculate-points", request).await
    }
}
