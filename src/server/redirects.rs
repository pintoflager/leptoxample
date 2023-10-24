#[cfg(any(feature = "leptos_ssr_axum", feature = "leptos_ssr_actix"))]
use leptos::*;

// Leptos server redirects
// https://leptos-rs.github.io/leptos/server/27_response.html
//
// Usable from server side only!

// curl -X POST -vvvvv "http://127.0.0.1:3000/api/redirect-home"
#[cfg(feature = "leptos_ssr_axum")]
#[server(RedirectHome, "/api", "Url", "redirect-home")]
pub async fn redirect_home() -> Result<(), ServerFnError> {
    use leptos_axum::redirect;

    redirect("/");
    Ok(())
}

#[cfg(not(feature = "leptos_ssr_axum"))]
#[cfg(feature = "leptos_ssr_actix")]
#[server(RedirectHome, "/api", "Url", "redirect-home")]
pub async fn redirect_home() -> Result<(), ServerFnError> {
    use leptos_actix::redirect;

    redirect("/");
    Ok(())
}
