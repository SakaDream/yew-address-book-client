use yew::prelude::*;
use yew_router::components::Link;
use crate::routes::AppRoute;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    html! {
        <>
            <h1>{ "Page not found" }</h1>
            <Link<AppRoute> to={AppRoute::Home}>{ "Go to Home" }</Link<AppRoute>>
        </>
    }
}
