use yew::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <div class={classes!("header")}>
            {"test"}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <header />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}