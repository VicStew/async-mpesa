use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{RedeemPointsRequest, RedeemPointsResponse}
};

/// Client to call the Sim Redeem Bonga Points API
pub struct RedeemPoints<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> RedeemPoints<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: RedeemPointsRequest,
    ) -> Result<RedeemPointsResponse, MpesaError> {
        self.client.post("/v1/lipa/na/bonga/redeem-paybill", request).await
    }
}
