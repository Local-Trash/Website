#![allow(non_snake_case)]
pub mod home {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        print!("Main");
        html! {
            <>
                <div class={classes!("home")}>
                    <p>
                        {"KeyCap Studios is a game development studio that produces and publishes games and/or game software. Our current goal is to make games that help the cheaper gamer. We are currently working on two different pieces of software. A game engine, called Fish, and a 2D fighter game called Immortality Arena. "}
                    </p>
                </div>
                <img src="/logo.png" alt="logo" class={classes!("logo")}/>
            </>
        }
    }
}

pub mod team {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("team")}>
                <div id="localtrash">
                    <header>{"LocalTrash"}</header>
                    <p> 
                        {"I'm one of the Co Founders of the studio and am the main developer of the game engine Fish. I am a rustacaen. My social media is @LocalTrashyt for Youtube and Loc@lTrash for Github"}
                    </p>
                </div> 
                <div id="superdoge">
                    <header>{"SuperDoge"}</header>
                    <p>
                        {"Hi, I'm one of the devs at keycap studios, I'm really creative and enjoy programming. I play a lot of overwatch 2 and take some inspiration from other games I play"}
                    </p>
                </div> 
            </div>
        }
    }
}

pub mod software {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("software")}>
                <iframe src="https://itch.io/embed/1661877?border_width=0&amp;bg_color=d1b187&amp;fg_color=4b3d44&amp;border_color=d2c9a5" width="206" height="165" frameborder="0"><a href="https://locltrash.itch.io/ghostin-ai">{"Ghostin AI by Loc@lTrash"}</a></iframe>
            </div>
        }
    }
}

pub mod blog {
    use yew::{Html, html, classes};

    pub fn Site() -> Html {
        html! {
            <div class={classes!("blog")}>
                <div class={classes!("card")}>
                    <header>{"Website is online"}</header>
                    <p>
                        {"We have finished and launched the website. The one you are right now. We can post games places where we weren't before. Welcome."}
                    </p>
                </div>
            </div>
        }
    }
}