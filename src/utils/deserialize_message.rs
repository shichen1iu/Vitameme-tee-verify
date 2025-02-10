use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// The custom signed session
#[derive(Debug, Serialize, Deserialize)]
pub struct VitaSignedSession {
    /// The version of the session
    pub version: String,
    /// The meta data of the session
    pub meta: SessionMeta,
    /// The signature of the session
    pub signature: String,
    /// The application data of the session
    pub application_data: String,
    /// The attributes of the session
    pub attributes: Vec<Attribute>,
}

/// The meta data of the session
#[derive(Debug, Serialize, Deserialize)]
pub struct SessionMeta {
    /// The url of the notary
    #[serde(rename = "notaryUrl")]
    pub notary_url: String,
    /// The url of the websocket proxy
    #[serde(rename = "websocketProxyUrl")]
    pub websocket_proxy_url: String,
}

/// The attribute of the session
#[derive(Debug, Serialize, Deserialize)]
pub struct Attribute {
    /// The hex encoded attribute
    pub attribute_hex: String,
    /// The name of the attribute
    pub attribute_name: String,
    /// The signature of the attribute
    pub signature: String,
}

pub fn deserialize_message(message: &str) -> Result<VitaSignedSession, ApiError> {
    let vita_signed_session: VitaSignedSession =
        serde_json::from_str(message).map_err(|e| ApiError::InvalidMessage(e.to_string()))?;
    Ok(vita_signed_session)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_valid_message() {
        let json_str = r#"{
            "version": "1.0",
            "meta": {
                "notaryUrl": "https://notary.example.com",
                "websocketProxyUrl": "wss://proxy.example.com"
            },
            "signature": "test_signature",
            "application_data": "test_data",
            "attributes": [
                {
                    "attribute_hex": "0x123",
                    "attribute_name": "test_attr",
                    "signature": "attr_signature"
                }
            ]
        }"#;

        let result = deserialize_message(json_str);
        assert!(result.is_ok());

        let session = result.unwrap();
        assert_eq!(session.version, "1.0");
        assert_eq!(session.meta.notary_url, "https://notary.example.com");
        assert_eq!(session.meta.websocket_proxy_url, "wss://proxy.example.com");
        assert_eq!(session.signature, "test_signature");
        assert_eq!(session.application_data, "test_data");

        let attr = &session.attributes[0];
        assert_eq!(attr.attribute_hex, "0x123");
        assert_eq!(attr.attribute_name, "test_attr");
        assert_eq!(attr.signature, "attr_signature");
    }

    #[test]
    fn test_deserialize_invalid_json() {
        let invalid_json = r#"{
            "version": "1.0",
            "invalid_json"
        }"#;

        let result = deserialize_message(invalid_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_missing_fields() {
        let missing_fields_json = r#"{
            "version": "1.0",
            "meta": {
                "notaryUrl": "https://notary.example.com",
                "websocketProxyUrl": "wss://proxy.example.com"
            }
        }"#;

        let result = deserialize_message(missing_fields_json);
        assert!(result.is_err());
    }
}
