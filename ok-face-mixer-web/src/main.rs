use leptos::prelude::ElementChild;
use leptos::{mount::mount_to_body, view};

fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Debug);

    mount_to_body(|| {
        view! {
            <p>"Hello, world!"</p>
        }
    });
}
