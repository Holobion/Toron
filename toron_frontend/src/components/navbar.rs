use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { id: "navbar",
            Link { to: Route::Home {}, "Tâches" }
        }
        main { Outlet::<Route> {} }
    }
}
