use yew::prelude::*;

#[function_component]
fn Header() -> Html {
    html! {
        <div class={classes!("PopOut")}>
            {"test"}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}