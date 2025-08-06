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

            <div class="flex items-center justify-center h-screen bg-gradient-to-b from-gray-700 to-gray-900 text-white">
                <h1 class="text-3xl font-bold
                animate-[wiggle_1s_ease-in-out_infinite]">
                "Welcome to Leptos"</h1>
            </div>
        </ErrorBoundary>
    }
}
