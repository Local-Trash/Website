use yew::prelude::*;

#[function_component]
fn App() -> Html {


    html! {
        <>
            <p id = {classes!("mainText")}>{ "Welcome to the offical website of KeyCap Studio's" }</p>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}