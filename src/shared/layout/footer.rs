use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer mt-auto py-3 bg-light">
            <div class="container">
                <span class="text-muted">{ format!("© 2022 Hai Phan") }</span>
            </div>
        </footer>
    }
}
