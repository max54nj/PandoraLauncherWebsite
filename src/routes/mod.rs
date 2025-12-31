use yew::prelude::*;
use yew_router::prelude::*;

mod home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: AppRoute) -> Html {
    match route {
        AppRoute::Home => html! {<home::Home />},
        AppRoute::NotFound => html! { "Page not found" },
    }
}
