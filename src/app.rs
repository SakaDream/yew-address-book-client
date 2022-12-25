use yew::prelude::*;
use yew_router::prelude::*;

use crate::{
    modules::{dashboard::*, hello_world::*, login::*, page_not_found::*, signup::*},
    routes::AppRoute,
    shared::layout::{footer::*, header::*},
};

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Index => html! {<HelloWorld />},
        AppRoute::Home => html! {<HelloWorld />},
        AppRoute::HelloWorld => html! {<HelloWorld />},
        AppRoute::Dashboard => html! {<Dashboard />},
        AppRoute::Login => html! {<Login />},
        AppRoute::Signup => html! {<Signup />},
        AppRoute::PageNotFound => html! {<PageNotFound />},
        AppRoute::Logout => html! {<p>{"Logged out!"}</p>},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Header />
            <main class="flex-shrink-0 mt-3">
                <div class="container-fluid">
                    <Switch<AppRoute> render={switch} />
                </div>
            </main>
            <Footer />
        </BrowserRouter>
    }
}
