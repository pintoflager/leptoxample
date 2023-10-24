
fn main() {
    // Server side main function
    #[cfg(feature = "ssr")]
    leptoxample::server::leptos_server()

    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
