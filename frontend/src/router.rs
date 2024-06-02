use crate::pages::{Home, Settings};

use yew::{function_component, html, Html};
use yew_router::prelude::*;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/settings")]
    Settings,
}

fn route(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::Settings => {
            html! { <Settings /> }
        }
    }
}

#[function_component(Main)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={route} />
        </BrowserRouter>
    }
}
