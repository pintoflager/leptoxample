#[cfg(any(feature = "leptos_ssr_axum", feature = "leptos_ssr_actix"))]
use {leptos::*, serde::Deserialize};

// Leptos server extractors
// https://leptos-rs.github.io/leptos/server/26_extractors.html
//
// Usable from server side only!

#[cfg(any(feature = "leptos_ssr_axum", feature = "leptos_ssr_actix"))]
#[derive(Debug, Deserialize)]
struct MyQuery {
    key: String
}

// curl -X POST -H "Content-Type: application/json" "http://127.0.0.1:3000/api/axum-extract?key=hello"
#[cfg(feature = "leptos_ssr_axum")]
#[server(AxumExtract, "/api", "Url", "axum-extract")]
pub async fn axum_extract() -> Result<String, ServerFnError> {
    use axum::{extract::Query, http::Method};
    use leptos_axum::extract;

    extract(|method: Method, res: Query<MyQuery>| async move {
        format!("{method:?} and key: {}", res.key)
    })
    .await
    .map_err(|_| ServerFnError::ServerError(
        "Could not extract method and query...".to_string()))
}

// curl -X POST -H "Content-Type: application/json" "http://127.0.0.1:3000/api/actix-extract?key=hello"
#[cfg(feature = "leptos_ssr_actix")]
#[server(ActixExtract, "/api", "Url", "actix-extract")]
pub async fn actix_extract() -> Result<String, ServerFnError> {
    use leptos_actix::extract;
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::Query;

    extract(|query: Query<MyQuery>, connection: ConnectionInfo| async move {
        format!(
            "key: {}\nconnection = {:?}",
            query.key,
            connection
        )
    }).await
}
