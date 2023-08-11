use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{BusinessPayBillRequest, BusinessPayBillResponse}
};

/// Client to call the business paybill API
pub struct Bpaybill<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Bpaybill<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: BusinessPayBillRequest,
    ) -> Result<BusinessPayBillResponse, MpesaError> {
        self.client.post("/b2b/v1/paymentrequest", request).await
    }
}