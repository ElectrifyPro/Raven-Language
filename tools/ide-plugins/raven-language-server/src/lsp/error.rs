use json::JsonValue;
use crate::lsp::Jsonable;

pub struct ResponseError {
    pub code: i32,
    pub message: String,
    pub data: Option<Box<dyn Jsonable>>
}

impl Jsonable for ResponseError {
    fn to_json(&self) -> JsonValue {
        let mut output = JsonValue::new_object();
        output.insert("code", self.code).unwrap();
        output.insert("message", self.message.clone()).unwrap();
        if let Some(found) = &self.data {
            output.insert("data", found.to_json()).unwrap();
        }
        return output;
    }
}

// Defined by JSON-RPC
pub static PARSE_ERROR: i32 = -32700;
pub static INVALID_REQUEST: i32 = -32600;
pub static METHOD_NOT_FOUND: i32 = -32601;
pub static INVALID_PARAMS: i32 = -32602;
pub static INTERNAL_ERROR: i32 = -32603;

/**
 * This is the start range of JSON-RPC reserved error codes.
 * It doesn't denote a real error code. No LSP error codes should
 * be defined between the start and end range. For backwards
 * compatibility the `SERVER_NOT_INITIALIZED` and the `UNKNOWN_ERROR_CODE`
 * are left in the range.
 *
 * @since 3.16.0
 */
pub static JSONRPC_RESERVED_ERROR_RANGE_START: i32 = -32099;
/** @deprecated use JSONRPC_RESERVED_ERROR_RANGE_START */
pub static SERVER_ERROR_START: i32 = JSONRPC_RESERVED_ERROR_RANGE_START;

/**
 * Error code indicating that a server received a notification or
 * request before the server has received the `initialize` request.
 */
pub static SERVER_NOT_INITIALIZED: i32 = -32002;
pub static UNKNOWN_ERROR_CODE: i32 = -32001;

/**
 * This is the end range of JSON-RPC reserved error codes.
 * It doesn't denote a real error code.
 *
 * @since 3.16.0
 */
pub static JSONRPC_RESERVED_ERROR_RANGE_END: i32 = -32000;
/** @deprecated use JSONRPC_RESERVED_ERROR_RANGE_END */
pub static SERVER_ERROR_END: i32 = JSONRPC_RESERVED_ERROR_RANGE_END;

/**
 * This is the start range of LSP reserved error codes.
 * It doesn't denote a real error code.
 *
 * @since 3.16.0
 */
pub static LSP_RESERVED_ERROR_RANGE_START: i32 = -32899;

/**
 * A request failed but it was syntactically correct, e.g the
 * method name was known and the parameters were valid. The error
 * message should contain human readable information about why
 * the request failed.
 *
 * @since 3.17.0
 */
pub static REQUEST_FAILED: i32 = -32803;

/**
 * The server cancelled the request. This error code should
 * only be used for requests that explicitly support being
 * server cancellable.
 *
 * @since 3.17.0
 */
pub static SERVER_CANCELLED: i32 = -32802;

/**
 * The server detected that the content of a document got
 * modified outside normal conditions. A server should
 * NOT send this error code if it detects a content change
 * in it unprocessed messages. The result even computed
 * on an older state might still be useful for the client.
 *
 * If a client decides that a result is not of any use anymore
 * the client should cancel the request.
 */
pub static CONTENT_MODIFIED: i32 = -32801;

/**
 * The client has canceled a request and a server as detected
 * the cancel.
 */
pub static REQUEST_CANCELLED: i32 = -32800;

/**
 * This is the end range of LSP reserved error codes.
 * It doesn't denote a real error code.
 *
 * @since 3.16.0
 */
pub static LSP_RESERVED_ERROR_RANGE_END: i32 = -32800;
