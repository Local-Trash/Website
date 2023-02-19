use yew::prelude::*;
use yew_router::prelude::*;
mod pages;

// The pages
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[not_found]
    #[at("/")]
    Home,
    #[at("/team")]
    Team,
    #[at("/software")]
    Software,
    #[at("/blog")]
    Blog,
}

// Access to the pages
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::home::Site(),
        Route::Team => pages::team::Site(),
        Route::Software => pages::software::Site(),
        Route::Blog => pages::blog::Site(),
    }
}

// The header component
#[function_component(Header)]
fn header() -> Html {
    html! {
        <div class={classes!("Header")}>
            <a href="/" id="Home">{"KeyCap Studios "}</a> <a href="/software" id="Software">{"Software "}</a> <a href="/team" id="devs">{"Devs "}</a> <a href="/blog" id="blog">{"Blog "}</a>
        </div>
    }
}

// The main app
#[function_component]
fn App() -> Html {
    html! {
        <>
            <Header/>

            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </>
    }
}

// The render of the main app
fn main() {
    yew::Renderer::<App>::new().render();
}