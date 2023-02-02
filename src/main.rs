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
    #[at("/games")]
    Games,
    #[at("/blog")]
    Blog,
}

// Access to the pages
fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::home::Site(),
        Route::Team => pages::team::Site(),
        Route::Games => pages::games::Site(),
        Route::Blog => pages::blog::Site(),
    }
}

// The header component
#[function_component(Header)]
fn header() -> Html {
    html! {
        <div class={classes!("Header")}>
            <p>{"KeyCap Studios "} <a href="/"><img src="logo.svg" alt="logo"/></a></p>
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
