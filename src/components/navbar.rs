use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        header {
            class: "navbar",
            div {
                class: "navbar-title",
                "Todo App"
            }
        }

        Outlet::<Route> {}
    }
}
