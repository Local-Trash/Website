#![allow(non_snake_case)]
pub mod home {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("Home")}>
                <p class={classes!(vec!["Card"])}>
                    {"
                        KeyCap Studios is a group of developers that program and publish digital applications. Our founder and lead developer is @LocalTrash_.
                    "}
                </p>
            </div>
        }
    }
}

pub mod team {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("Team")}>

            </div>
        }
    }
}

pub mod games {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("Blog")}>
                <title>{"Test"}</title>
            </div>
        }
    }
}

pub mod blog {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("blog")}>

            </div>
        }
    }
}

pub mod notFound {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("notFound")}>
                <p>{"Sorry that page was not found?"}</p>
            </div>
        }
    }
}