use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            {"help"}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}