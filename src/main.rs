use leptos::*;
mod components;
use components::App;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
