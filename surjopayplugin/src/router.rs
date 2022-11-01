use crate::components::pages::hello::Hello;
use crate::components::pages::home::Home;
use crate::components::pages::formpage::Formpage;
use crate::components::pages::showformInfopage::Showforminfopage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
    #[at("/formpage")]
    Formpage,
    #[at("/showforminfopage")]
    Showforminfopage,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Hello => html! { <Hello /> },
        Route::Formpage => html! { <Formpage /> },
        Route::Showforminfopage => html! { <Showforminfopage /> },
    }
}
