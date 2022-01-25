use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 30);
    cx.render(rsx!(
        body {
            margin: "0",
            width: "100%",
            height: "100%",
            background_color: "gray",
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            justify_content: "center",
            style { [include_str!("../src/style.css")] }
            div {class: "pie no-round", style: "--p:{count};--b:100%;--c:red;--w:50%;"}
            div {
                button { onclick: move |_| count += 1, "Start" }
                button { onclick: move |_| count -= 1, "Stop" }
            }
        }
    ))
}
