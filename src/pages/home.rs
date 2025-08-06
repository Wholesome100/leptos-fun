use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <main class="flex h-screen bg-gradient-to-b from-gray-700 to-gray-900 text-white">
                <section class="mt-4 ml-6 animate-[fadeSlideLeft_0.8s_ease-out_forwards]">
                //animate-[wiggle_1s_ease-in-out_infinite]
                    <h1 class="text-3xl font-bold">"Welcome to Leptos"</h1>
                    <p class="text-2xl">"Click around and glimpse a future with WASM!"</p>
                </section>
            </main>

        </ErrorBoundary>
    }
}
