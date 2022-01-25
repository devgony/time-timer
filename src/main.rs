use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 5);
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
                div {class: "pie no-round", style: "--p:{count};--b:100%;--c:red;--w:50vh;"}
            }
            div {
                button { onclick: move |_| count += 5, "+" }
                button { onclick: move |_| count -= 5, "-" }
                button { onclick: move |_| count -= 0, "Start" }
            }
        }
    ))
}
