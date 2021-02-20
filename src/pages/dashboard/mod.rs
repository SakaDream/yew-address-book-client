use yew::{ComponentLink, services::fetch::FetchTask};

use crate::shared::model::{Person, ResponseBody};

pub mod dashboard_component;

pub struct Dashboard {
    link: ComponentLink<Self>,
    has_error: bool,
    fetching: bool,
    response: ResponseBody<Vec<Person>>,
    ft: Option<FetchTask>,
}