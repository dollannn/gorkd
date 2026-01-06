use gorkd_core::LlmError;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OpenAiErrorResponse {
    pub error: OpenAiError,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicErrorResponse {
    #[serde(rename = "type")]
    pub error_type: String,
    pub error: AnthropicError,
}

#[derive(Debug, Deserialize)]
pub struct AnthropicError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
}

pub fn map_openai_error(status: StatusCode, body: &str) -> LlmError {
    let parsed: Result<OpenAiErrorResponse, _> = serde_json::from_str(body);

    match status {
        StatusCode::UNAUTHORIZED => LlmError::Provider("invalid API key".to_string()),
        StatusCode::TOO_MANY_REQUESTS => LlmError::RateLimited,
        StatusCode::BAD_REQUEST => {
            if let Ok(resp) = parsed {
                if resp.error.code.as_deref() == Some("context_length_exceeded") {
                    return LlmError::ContextLengthExceeded {
                        max_tokens: 0,
                        got_tokens: 0,
                    };
                }
                LlmError::Provider(resp.error.message)
            } else {
                LlmError::Provider(body.to_string())
            }
        }
        StatusCode::NOT_FOUND => {
            if let Ok(resp) = parsed {
                LlmError::ModelUnavailable {
                    model: resp.error.message,
                }
            } else {
                LlmError::ModelUnavailable {
                    model: "unknown".to_string(),
                }
            }
        }
        StatusCode::SERVICE_UNAVAILABLE | StatusCode::BAD_GATEWAY | StatusCode::GATEWAY_TIMEOUT => {
            LlmError::Provider(format!("service unavailable: {}", status))
        }
        _ => {
            if let Ok(resp) = parsed {
                LlmError::Provider(resp.error.message)
            } else {
                LlmError::Provider(format!("HTTP {}: {}", status, body))
            }
        }
    }
}

pub fn map_anthropic_error(status: StatusCode, body: &str) -> LlmError {
    let parsed: Result<AnthropicErrorResponse, _> = serde_json::from_str(body);

    match status {
        StatusCode::UNAUTHORIZED => LlmError::Provider("invalid API key".to_string()),
        StatusCode::TOO_MANY_REQUESTS => LlmError::RateLimited,
        StatusCode::BAD_REQUEST => {
            if let Ok(resp) = parsed {
                if resp.error.error_type == "invalid_request_error"
                    && resp.error.message.contains("token")
                {
                    return LlmError::ContextLengthExceeded {
                        max_tokens: 0,
                        got_tokens: 0,
                    };
                }
                LlmError::Provider(resp.error.message)
            } else {
                LlmError::Provider(body.to_string())
            }
        }
        StatusCode::NOT_FOUND => {
            if let Ok(resp) = parsed {
                LlmError::ModelUnavailable {
                    model: resp.error.message,
                }
            } else {
                LlmError::ModelUnavailable {
                    model: "unknown".to_string(),
                }
            }
        }
        StatusCode::SERVICE_UNAVAILABLE
        | StatusCode::BAD_GATEWAY
        | StatusCode::GATEWAY_TIMEOUT
        | StatusCode::INTERNAL_SERVER_ERROR => {
            if let Ok(resp) = parsed {
                if resp.error.error_type == "overloaded_error" {
                    return LlmError::RateLimited;
                }
            }
            LlmError::Provider(format!("service unavailable: {}", status))
        }
        _ => {
            if let Ok(resp) = parsed {
                LlmError::Provider(resp.error.message)
            } else {
                LlmError::Provider(format!("HTTP {}: {}", status, body))
            }
        }
    }
}

pub fn map_reqwest_error(err: reqwest::Error) -> LlmError {
    if err.is_timeout() {
        LlmError::Timeout { timeout_secs: 0 }
    } else if err.is_connect() {
        LlmError::Network(format!("connection failed: {}", err))
    } else {
        LlmError::Network(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_openai_rate_limit() {
        let error = map_openai_error(StatusCode::TOO_MANY_REQUESTS, "{}");
        assert!(matches!(error, LlmError::RateLimited));
        assert!(error.is_retryable());
    }

    #[test]
    fn maps_openai_unauthorized() {
        let error = map_openai_error(StatusCode::UNAUTHORIZED, "{}");
        assert!(matches!(error, LlmError::Provider(_)));
    }

    #[test]
    fn maps_openai_context_length() {
        let body = r#"{"error":{"message":"context too long","type":"invalid_request_error","code":"context_length_exceeded"}}"#;
        let error = map_openai_error(StatusCode::BAD_REQUEST, body);
        assert!(matches!(error, LlmError::ContextLengthExceeded { .. }));
    }

    #[test]
    fn maps_anthropic_rate_limit() {
        let error = map_anthropic_error(StatusCode::TOO_MANY_REQUESTS, "{}");
        assert!(matches!(error, LlmError::RateLimited));
        assert!(error.is_retryable());
    }

    #[test]
    fn maps_anthropic_overloaded() {
        let body = r#"{"type":"error","error":{"type":"overloaded_error","message":"overloaded"}}"#;
        let error = map_anthropic_error(StatusCode::SERVICE_UNAVAILABLE, body);
        assert!(matches!(error, LlmError::RateLimited));
    }

    #[test]
    fn maps_anthropic_unauthorized() {
        let error = map_anthropic_error(StatusCode::UNAUTHORIZED, "{}");
        assert!(matches!(error, LlmError::Provider(_)));
    }
}
