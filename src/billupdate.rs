use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{BillUpdateRequest, BillUpdateResponse}
};

/// Client to call Bill Optin details Update API
pub struct BillUpdate<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> BillUpdate<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: BillUpdateRequest,
    ) -> Result<BillUpdateResponse, MpesaError> {
        self.client.post("/v1/billmanager-invoice/change-optin-details", request).await
    }
}