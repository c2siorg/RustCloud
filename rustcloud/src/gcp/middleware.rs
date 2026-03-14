use std::collections::HashMap;

use reqwest::{header::AUTHORIZATION, Client, Method};
use serde_json::Value;

use crate::gcp::errors::GcpApiError;

pub fn required_string_from_map(
    request: &HashMap<String, String>,
    field: &'static str,
) -> Result<String, GcpApiError> {
    let value = request
        .get(field)
        .ok_or(GcpApiError::MissingField { field })?
        .trim()
        .to_string();

    if value.is_empty() {
        return Err(GcpApiError::InvalidFieldValue {
            field,
            message: "value cannot be empty".to_string(),
        });
    }

    Ok(value)
}

pub fn required_json_string(
    request: &HashMap<String, Value>,
    field: &'static str,
) -> Result<String, GcpApiError> {
    let value = request
        .get(field)
        .ok_or(GcpApiError::MissingField { field })?
        .as_str()
        .ok_or(GcpApiError::InvalidFieldType {
            field,
            expected: "string",
        })?
        .trim()
        .to_string();

    if value.is_empty() {
        return Err(GcpApiError::InvalidFieldValue {
            field,
            message: "value cannot be empty".to_string(),
        });
    }

    Ok(value)
}

pub fn array_from_json_field<'a>(
    value: &'a Value,
    field: &'static str,
) -> Result<&'a Vec<Value>, GcpApiError> {
    value.as_array().ok_or(GcpApiError::InvalidFieldType {
        field,
        expected: "array",
    })
}

pub fn object_from_json_field<'a>(
    value: &'a Value,
    field: &'static str,
) -> Result<&'a serde_json::Map<String, Value>, GcpApiError> {
    value.as_object().ok_or(GcpApiError::InvalidFieldType {
        field,
        expected: "object",
    })
}

pub async fn send_authorized_json_request(
    client: &Client,
    method: Method,
    url: &str,
    token: &str,
    body: Option<String>,
) -> Result<HashMap<String, String>, GcpApiError> {
    let mut request = client
        .request(method, url)
        .header("Content-Type", "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", token));

    if let Some(payload) = body {
        request = request.body(payload);
    }

    let response = request.send().await?;
    let status = response.status().as_u16().to_string();
    let response_body = response.text().await?;

    let mut out = HashMap::new();
    out.insert("status".to_string(), status);
    out.insert("body".to_string(), response_body);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_string_from_map_returns_value() {
        let mut request = HashMap::new();
        request.insert("projectid".to_string(), "demo".to_string());
        let value = required_string_from_map(&request, "projectid").unwrap();
        assert_eq!(value, "demo");
    }

    #[test]
    fn required_string_from_map_rejects_missing() {
        let request = HashMap::<String, String>::new();
        let error = required_string_from_map(&request, "projectid").unwrap_err();
        assert!(matches!(error, GcpApiError::MissingField { field: "projectid" }));
    }

    #[test]
    fn required_json_string_rejects_non_string() {
        let mut request = HashMap::new();
        request.insert("projectid".to_string(), Value::Bool(true));
        let error = required_json_string(&request, "projectid").unwrap_err();

        assert!(matches!(
            error,
            GcpApiError::InvalidFieldType {
                field: "projectid",
                expected: "string"
            }
        ));
    }
}
