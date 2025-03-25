use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{ReconciliationRequest, ReconciliationResponse}
};

/// Client to call Bill Reconciliation API
pub struct Reconciliation<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Reconciliation<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: ReconciliationRequest,
    ) -> Result<ReconciliationResponse, MpesaError> {
        self.client.post("/v1/billmanager-invoice/reconciliation", request).await
    }
}