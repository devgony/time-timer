use dioxus::prelude::*;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 125);
    let num = if count.le(&270) {
        count + 0
    } else {
        count - 180
    };
    let color = if count.le(&270) { "white" } else { "red" };
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
            div {
                background_color: "red",
                border: "1px solid blue",
                width: "50vh",
                height: "50vh",
                border_radius: "50%",
                display: "flex",
                align_items: "center",
                justify_content: "center",

                background_image:
                    "linear-gradient({num}deg, transparent 50%, {color} 50%),
                    linear-gradient(90deg, white 50%, transparent 50%)",
                // margin: "auto",
                // div {
                //     background_color: "red",
                //     width: "49vh",
                //     height: "49vh",
                //     border_radius: "50%",
                // }
            }
            div {
                button { onclick: move |_| count += 10, "Start" }
                button { onclick: move |_| count -= 10, "Stop" }
            }
        }
    ))
}
