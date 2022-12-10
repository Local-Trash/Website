use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
            println!("{}", value);
        }
    };

    html! {
        <>
            <div>
                <p>{"Test"}</p>
            </div>

            <div>
                <button {onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
            </div>

        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}