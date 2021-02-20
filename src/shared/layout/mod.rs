use yew::{ComponentLink, Properties};

use crate::routes::AppRoute;

pub mod header;
pub mod footer;

pub struct Header {
    _link: ComponentLink<Self>,
    props: HeaderProps,
}

#[derive(Properties, Clone)]
pub struct HeaderProps {
    pub current_route: Option<AppRoute>,
}

pub struct Footer {
    _link: ComponentLink<Self>,
}
