mod app;
mod api;
mod components;

use leptos::*;

use crate::app::App;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx, <App/> })
}

