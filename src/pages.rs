#![allow(non_snake_case)]
pub mod home {
    use yew::{Html, html};

    pub fn site() -> Html {
        html! {
            <p>{"help"}</p>
        }
    }
}

pub mod notFound {
    use yew::{Html, html, classes};

    pub fn site() -> Html {
        html! {
            <div class={classes!("notFound")}>
                <p>{"Sorry that page was not found?"}</p>
            </div>
        }
    }
}