use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello world"}</h1>
    }
}

pub fn serve() {
    yew::Renderer::<App>::new().render();
}
