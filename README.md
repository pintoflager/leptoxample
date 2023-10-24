<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum / Actix-web Example

Example of [Leptos](https://github.com/leptos-rs/leptos) starter template + [Axum](https://github.com/tokio-rs/axum) or [Actix-web](https://github.com/actix/actix-web) as the server. This is SSR mode only example.

## Where to start

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos
```


```bash
git clone git@github.com:pintoflager/leptoxample.git
```

Then go where the files are
```bash
cd leptoxample
```

## Which server to use

By default Axum is selected

To change that, modify one line and one line only
```toml
bin-features = ["ssr", "leptos_ssr_axum"]
```

to
```toml
bin-features = ["ssr", "leptos_ssr_actix"]
```

## Running locally

Run
```bash
cargo leptos watch
```

or run
```bash
cargo leptos serve
```

or only
```bash
cargo leptos build
```
and then
```bash
cargo leptos serve
```

## Kicking the tires

Curl or browser:
- http://127.0.0.1:3000
- http://127.0.0.1:3000/non-leptos-route
- http://127.0.0.1:3000/not-around-am-i

Curl (or similar):
- curl -X POST -H "Content-Type: application/json" http://127.0.0.1:3000/api/say
- curl -X POST -H "Content-Type: application/json" http://127.0.0.1:3000/api/hello
- curl -X POST -vvvvv "http://127.0.0.1:3000/api/redirect-home"

Extracted something from server: 
- curl -X POST -H "Content-Type: application/json" "http://127.0.0.1:3000/api/axum-extract?key=hello"
- curl -X POST -H "Content-Type: application/json" "http://127.0.0.1:3000/api/actix-extract?key=hello"