use strum::IntoEnumIterator;
use yew::prelude::*;
use yew_router::components::Link;
use yew_router::hooks::use_route;

use crate::routes::AppRoute;
use crate::shared::constraint;

#[function_component(Header)]
pub fn header() -> Html {
    let route: AppRoute = use_route().unwrap_or_default();

    let mut links = vec![];
    for app_route in AppRoute::iter() {
        if (app_route.as_str() != AppRoute::Login.as_str())
            & (app_route.as_str() != AppRoute::Signup.as_str())
        {
            let mut classes = String::from("nav-link");

            if route.as_str() == app_route.as_str() {
                classes.push_str(" active");
            }
            let app_route_clone = app_route.clone();

            links.push(html! {
                <li class="nav-item">
                    <Link<AppRoute> to={app_route} classes={classes!(classes)} >{ app_route_clone.as_str() }</Link<AppRoute>>
                </li>
            });
        }
    }

    html! {
        <header>
            <nav class="navbar navbar-expand navbar-dark bg-dark">
                <div class="container-fluid">
                    <a class="navbar-brand" href="#">{ constraint::APP_NAME }</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse"
                        data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false"
                        aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                            { for links }
                        </ul>
                    </div>
                </div>
            </nav>
        </header>
    }
}
