use yew::prelude::*;
use crate::components::button::Default;

#[function_component(App)]
fn app() -> Html {
    // Here we use Tailwind CSS classes!
    html! {
        <main class="h-screen w-screen flex items-center justify-center bg-gray-900">
            <h1 class="text-4xl font-bold text-teal-400">
                { "Hello, World from Yew + Tailwind!" }
            </h1>
            <button class="btn">Default</button>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}