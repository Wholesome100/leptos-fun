use leptos::mount::mount_to_body;
use leptos_fun::App;

// Start Tailwind compiler with:
// npx @tailwindcss/cli -i ./input.css -o ./output.css --watch

// Install trunk with:
// cargo install trunk
// Run app with:
// trunk serve --open

// Format view! code with: leptosfmt .

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(App)
}

