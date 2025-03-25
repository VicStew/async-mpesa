use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{CancelInvoiceRequest, CancelInvoiceResponse}
};

/// Client to call Cancel Invoice API
pub struct CancelInvoice<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> CancelInvoice<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: CancelInvoiceRequest,
    ) -> Result<CancelInvoiceResponse, MpesaError> {
        self.client.post("/v1/billmanager-invoice/cancel-single-invoice", request).await
    }
}