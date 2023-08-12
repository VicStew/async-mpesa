use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{SingleInvoicingRequest, SingleInvoicingResponse},
};

/// Client to call the single invoicing API.
pub struct SingleInvoice<'m, C: Config> {
    client: &'m Client<C>
}

impl <'m, C: Config> SingleInvoice<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: SingleInvoicingRequest,
    ) -> Result<SingleInvoicingResponse, MpesaError> {
        self.client.post("/billmanager-invoice/single-invoicing", request).await
    }
}