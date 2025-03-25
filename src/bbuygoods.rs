use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{BusinessBuyGoodsRequest, BusinessBuyGoodsResponse}
};

/// Client to call the business buy goods API
pub struct Bbuygoods<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Bbuygoods<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: BusinessBuyGoodsRequest,
    ) -> Result<BusinessBuyGoodsResponse, MpesaError> {
        self.client.post("/mpesa/b2b/v1/paymentrequest", request).await
    }
}
