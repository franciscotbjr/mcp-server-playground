//! Tests for the centralized error types.

use mcp_server_playground::Error;

#[test]
fn test_error_display() {
    let err = Error::IoError("file not found".into());
    assert_eq!(err.to_string(), "I/O error: file not found");
}

#[test]
fn test_error_from_io() {
    let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "missing");
    let err: Error = io_err.into();
    assert!(matches!(err, Error::IoError(_)));
}

#[test]
fn test_error_from_serde_json() {
    let json_err = serde_json::from_str::<String>("not json").unwrap_err();
    let err: Error = json_err.into();
    assert!(matches!(err, Error::JsonError(_)));
}

#[test]
fn test_protocol_error_display() {
    let err = Error::ProtocolError("invalid version".into());
    assert_eq!(err.to_string(), "MCP protocol error: invalid version");
}

#[test]
fn test_tool_error_display() {
    let err = Error::ToolError("execution failed".into());
    assert_eq!(err.to_string(), "Tool execution error: execution failed");
}

#[test]
fn test_data_not_found_error_display() {
    let err = Error::DataNotFoundError("calendar.json".into());
    assert_eq!(err.to_string(), "Data file not found: calendar.json");
}

#[test]
fn test_invalid_request_error_display() {
    let err = Error::InvalidRequestError("missing params".into());
    assert_eq!(err.to_string(), "Invalid request: missing params");
}

#[test]
fn test_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Error>();
}
