use leptos::*;

// Leptos server responses
// https://leptos-rs.github.io/leptos/server/27_response.html
//
// Usable from server side only!

pub fn set_status(code: http::status::StatusCode) {
    #[cfg(feature = "leptos_ssr_axum")]
    axum_response(code);

    #[cfg(feature = "leptos_ssr_actix")]
    actix_response(code);
}

#[cfg(feature = "leptos_ssr_axum")]
fn axum_response(code: http::status::StatusCode) {
    let response = use_context::<leptos_axum::ResponseOptions>();
    
    if let Some(response) = response {
        response.set_status(code);
    }
}

#[cfg(feature = "leptos_ssr_actix")]
fn actix_response(code: http::status::StatusCode) {
    let response = use_context::<leptos_actix::ResponseOptions>();
    
    if let Some(response) = response {
        response.set_status(code);
    }
}
