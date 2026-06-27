use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{C2bRegisterRequest, C2bRegisterResponse}
};

/// Client to call the C2bRegister API
pub struct C2bRegister<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> C2bRegister<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: C2bRegisterRequest,
    ) -> Result<C2bRegisterResponse, MpesaError> {
        self.client.post("/mpesa/c2b/v2/registerurl", request).await
    }
}
