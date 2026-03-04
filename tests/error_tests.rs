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
fn test_error_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Error>();
}
