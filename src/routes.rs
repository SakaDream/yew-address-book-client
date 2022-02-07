use strum_macros::EnumIter;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable, EnumIter)]
pub enum AppRoute {
    #[at("/")]
    Index,
    #[at("/home")]
    Home,
    #[at("/hello-world")]
    HelloWorld,
    #[at("/dashboard")]
    Dashboard,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
}

impl AppRoute {
    pub fn as_str(&self) -> &str {
        match self {
            AppRoute::Index => "Index",
            AppRoute::Home => "Home",
            AppRoute::HelloWorld => "Hello World",
            AppRoute::Dashboard => "Dashboard",
            AppRoute::PageNotFound => "Page Not Found",
        }
    }
}
