use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <h1>{ "Hello World!" }</h1>
    }
}
