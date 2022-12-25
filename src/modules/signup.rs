use yew::prelude::*;

#[function_component(Signup)]
pub fn signup() -> Html {
    html! {
        <div class="container">
            <div class="row d-flex align-items-center justify-content-center">
                <div class="col-4">
                    <form action="">
                        <div class="form-group">
                            <h2>{ "Please Sign Up" }</h2>
                        </div>
                        <div class="form-group">
                            <label for="email" class="col-form-label">{ "Email" }</label>
                            <input type="text" class="form-control" id="email" placeholder="Email"/>
                        </div>
                        <div class="form-group">
                            <label for="username" class="col-form-label">{ "Username" }</label>
                            <input type="text" class="form-control" id="username" placeholder="Username"/>
                        </div>
                        <div class="form-group">
                            <label for="password" class="col-form-label">{ "Password" }</label>
                            <input type="password" class="form-control" id="password" placeholder="Password"/>
                        </div>
                        <div class="form-group">
                            <label for="retype-password" class="col-form-label">{ "Retype Password" }</label>
                            <input type="password" class="form-control" id="retype-password" placeholder="Retype Password"/>
                        </div>
                        <div class="form-group mt-3">
                            <button type="submit" class="btn btn-primary form-control">{ "Sign Up" }</button>
                        </div>
                        <div class="text-center mt-3">
                          <span>{ "Already have an account? " }</span>
                          <a href="/login">{ "Please sign in!" }</a>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
