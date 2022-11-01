mod components;
mod router;

// use crate::components::molecules::custom_form::Data;
use crate::router::{switch, Route};

use stylist::yew::styled_component;
use yew::prelude::*;
// use yew::ContextProvider;
use yew_router::prelude::*;



#[styled_component(App)]
pub fn app() -> Html {
    html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
    }
}
