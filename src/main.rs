#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    #[cfg(feature = "frontend")]
    dioxus_web::launch_cfg(App, dioxus_web::Config::new().hydrate(true));

    #[cfg(feature = "backend")]
    {
        use axum::response::Html;
        use axum::{Router, Server};
        use http::uri::Uri;
        use std::net::SocketAddr;
        use tokio::fs;
        use tokio::runtime::Runtime;
        use tower_http::services::ServeDir;

        Runtime::new().unwrap().block_on(async move {
            let addr = SocketAddr::from(([127, 0, 0, 1], 9001));
            let serve_dir = ServeDir::new("dist/assets");
            let routes =
                Router::new()
                    .nest_service("/assets", serve_dir)
                    .fallback(move |_uri: Uri| {
                        let mut app = VirtualDom::new(App);
                        let _ = app.rebuild();

                        let html = dioxus_ssr::pre_render(&app);

                        async move {
                            let index = fs::read_to_string("dist/index.html").await.unwrap();
                            let (prefix, suffix) = index.split_once(r#"<div id="main">"#).unwrap();

                            Html(format!(r#"{prefix}<div id="main">{html}{suffix}"#))
                        }
                    });

            println!("Listening on {}", addr);
            Server::bind(&addr)
                .serve(routes.into_make_service())
                .await
                .unwrap();
        });
    }
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Counter {}
    })
}

fn Counter(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    })
}
