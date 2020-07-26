use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "#/"]
    Index,
    #[to = "#/hello-world"]
    HelloWorld,
    #[to = "#/page-not-found"]
    PageNotFound,
}

/// Fix url handling problem for yew_router
pub fn switch(route: &mut Route) -> AppRoute {
    let r = route.route.as_str();
    if let Some(index) = r.find('#') {
        route.route = r[index..].to_string();
    } else {
        route.route = "#/".to_string();
    }

    let current_route = AppRoute::switch(route.clone());
    if current_route == Some(AppRoute::Index) && route.route.as_str() != "#/" {
        AppRoute::PageNotFound
    } else {
        current_route.unwrap()
    }
}