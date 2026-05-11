use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::{Result, VoidBlockError};

pub struct DohClient {
    client: reqwest::Client,
    endpoints: Vec<reqwest::Url>,
}

impl DohClient {
    pub fn new(endpoints: Vec<reqwest::Url>) -> Result<Self> {
        if endpoints.is_empty() {
            return Err(VoidBlockError::Resolver("at least one DoH endpoint is required".to_string()));
        }
        let client = reqwest::Client::builder().build()?;
        Ok(Self { client, endpoints })
    }

    pub async fn forward(&self, query: &[u8]) -> Result<Vec<u8>> {
        let mut last_error: Option<reqwest::Error> = None;
        for endpoint in &self.endpoints {
            let response = self
                .client
                .post(endpoint.clone())
                .header(CONTENT_TYPE, "application/dns-message")
                .header(ACCEPT, "application/dns-message")
                .body(query.to_vec())
                .send()
                .await;
            match response {
                Ok(resp) => {
                    let bytes = resp.error_for_status()?.bytes().await?;
                    return Ok(bytes.to_vec());
                }
                Err(error) => last_error = Some(error),
            }
        }

        match last_error {
            Some(error) => Err(error.into()),
            None => Err(VoidBlockError::Resolver("no DoH endpoint produced a response".to_string())),
        }
    }
}
