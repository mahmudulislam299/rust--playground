mod components;
mod router;


use crate::router::{switch, Route};

use gloo::console::log;
use std::ops::Deref;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;


#[styled_component(App)]
pub fn app() -> Html {

    html! {
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
    }
}
