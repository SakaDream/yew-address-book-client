use yew_router::prelude::*;
use yew::prelude::*;

use crate::{
    components::{ hello_world::*, dashboard::*, page_not_found::*, },
    shared::layout::{ header::*, footer::*, },
    routes::AppRoute,
};

 
fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Index => html!{<HelloWorld />},
        AppRoute::Home => html!{<HelloWorld />},
        AppRoute::HelloWorld => html!{<HelloWorld />},
        AppRoute::Dashboard => html!{<Dashboard />},
        AppRoute::PageNotFound => html!{<PageNotFound />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    Switch::render(switch);

    html! {
        <BrowserRouter>
            <Header />
            <main class="flex-shrink-0 mt-3">
                <div class="container-fluid">
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </div>
            </main>
            <Footer />
        </BrowserRouter>
    }
}
