pub mod functions;

#[cfg(feature = "ssr")]
pub mod responses;

#[cfg(feature = "ssr")]
pub mod redirects;

#[cfg(feature = "ssr")]
pub mod extractors;

#[cfg(all(feature = "ssr", feature = "leptos_ssr_axum"))]
pub mod axum;

#[cfg(all(feature = "ssr", feature = "leptos_ssr_actix"))]
pub mod actix;

// Server starter function
#[cfg(feature = "ssr")]
pub fn leptos_server() {
    #[cfg(feature = "leptos_ssr_axum")]
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        axum::leptos_server().await
    });

    #[cfg(feature = "leptos_ssr_actix")]
    actix::leptos_server()
}
