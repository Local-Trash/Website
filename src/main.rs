use yew::prelude::*;
use yew_router::prelude::*;
mod pages;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    Notfound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => pages::home::site(),
        Route::Notfound => pages::notFound::site(),
    }
}

#[function_component(Header)]
fn header() -> Html {
    html! {
        <div class={classes!("PopOut")}>
            {"test"}
        </div>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}