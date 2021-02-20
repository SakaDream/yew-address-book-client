use yew_router::prelude::*;

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/"]
    Index,
    #[to = "#/home"]
    Home,
    #[to = "#/hello-world"]
    HelloWorld,
    #[to = "#/dashboard"]
    Dashboard,
    #[to = "#/page-not-found"]
    PageNotFound,
}

/// Fix fragment handling problem for yew_router
/// Reference: https://github.com/jetli/rust-yew-realworld-example-app/blob/45667b4cd10bd4195b5253fcd9488c493af5964e/crates/conduit-wasm/src/routes/mod.rs#L37
pub fn fix_fragment_routes(route: &mut Route) {
    let r = route.route.as_str();
    if let Some(index) = r.find('#') {
        route.route = r[index..].to_string();
    } else {
        route.route = "#/home".to_string();
    }
}
