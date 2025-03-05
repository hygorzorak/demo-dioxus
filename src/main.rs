use dioxus::prelude::*;

use components::Navbar;
use views::TodoList;

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    TodoList {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { 
            rel: "stylesheet", 
            href: "https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap" 
        }

        Router::<Route> {}
    }
}
