use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let sec = use_ref(&cx, || 5);
    use_future(&cx, || {
        let sec = sec.clone(); // future static var should be cloned
        async move {
            loop {
                TimeoutFuture::new(1_000).await;
                *sec.write() += 5;
            }
        }
    });
    let cur_sec = sec.read();
    cx.render(rsx!(
        body {
            margin: "0",
            width: "100vw",
            height: "100vh",
            background_color: "gray",
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            style { [include_str!("../src/style.css")] }
            div {
                width: "50vh",
                height: "50vh",
                background_color: "white",
                border_radius: "50%",
                div {class: "pie no-round", style: "--p:{cur_sec};--b:100%;--c:red;--w:50vh;"}
            }
            div {
                h1 {"{cur_sec}"}
                // button { onclick: move |_| {sec += 5}, "+" }
                // button { onclick: move |_| set_state(sec - 5), "-" }
                // button { onclick: move |_| set_state(sec + 0), "Start" }
            }
        }
    ))
}
