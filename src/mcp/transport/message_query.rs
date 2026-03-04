//! Query parameters for the `POST /message` endpoint.

#[derive(Debug, serde::Deserialize)]
pub(crate) struct MessageQuery {
    #[serde(rename = "sessionId")]
    pub(crate) session_id: String,
}
