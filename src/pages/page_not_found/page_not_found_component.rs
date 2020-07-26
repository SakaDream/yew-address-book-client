use yew::{Component, ComponentLink, Html, html};
use yew_router::prelude::*;
use crate::{
    pages::page_not_found::PageNotFound,
    routes::AppRoute,
};

impl Component for PageNotFound {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PageNotFound {}
    }

    fn update(&mut self, _: Self::Message) -> bool {
        true
    }

    fn change(&mut self, _: Self::Properties) -> yew::ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Page not found" }</h1>
                <RouterAnchor<AppRoute> route=AppRoute::Index> { "Go to Home" } </RouterAnchor<AppRoute>>
            </>
        }
    }
}