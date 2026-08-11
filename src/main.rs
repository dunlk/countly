mod app;
mod features;
mod tauri;
use leptos::mount::*;

fn main() {
    mount_to_body(app::App)
}
