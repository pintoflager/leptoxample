use leptos::*;

// Leptos server functions
// https://leptos-rs.github.io/leptos/server/25_server_functions.html
//
// These can be called from (and actually defined in) frontend!

// Macro options that define response data type:
// #[server(AddTodo, "/api", "Url", "optional-url-path")]
// #[server(AddTodo, "/api", "GetJson", "optional-url-path")]
// #[server(AddTodo, "/api", "Cbor", "optional-url-path")]
// #[server(AddTodo, "/api", "GetCbor", "optional-url-path")]

// curl -X POST -H "Content-Type: application/json" http://127.0.0.1:3000/api/say
#[server(Say, "/api", "Url", "say")]
pub async fn say() -> Result<String, ServerFnError> {
    Ok("hello".to_string())
    // Err(ServerFnError::ServerError(e.to_string()))
}

// curl -X POST -H "Content-Type: application/json" http://127.0.0.1:3000/api/hello
#[server(Hello, "/api", "GetJson", "hello")]
pub async fn hello() -> Result<Vec<String>, ServerFnError> {
    Ok(vec!["hello".to_string(), "world".to_string()])
    // Err(ServerFnError::ServerError(e.to_string()))
}
