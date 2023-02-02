#![allow(non_snake_case)]
pub mod home {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        print!("Main");
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
                <iframe frameborder="0" src="https://itch.io/embed/1661877?border_width=0&amp;dark=true" width="206" height="165"><a href="https://locltrash.itch.io/ghostin-ai">{"Ghostin AI by Loc@lTrash"}</a></iframe>
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
