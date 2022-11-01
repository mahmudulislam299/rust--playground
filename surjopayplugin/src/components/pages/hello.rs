// use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Hello)]
pub fn hello() -> Html {
    let history1 = use_history().unwrap();

    let onclick1 = Callback::from(move |_| {
        history1.push(Route::Home);
    });

    let history2 = use_history().unwrap();
    let onclick2 = Callback::from(move |_| {
      history2.push(Route::Formpage);
    });

    let history3 = use_history().unwrap();
    let onclick3 = Callback::from(move |_| {
      history3.push(Route::Showforminfopage);
    });

    html! {
      <div>
        <h1>{"Hello"}</h1>
        <button onclick ={onclick1}>{"Go Home"}</button>
        <button onclick ={onclick2}>{"Go Formpage"}</button>
        <button onclick ={onclick3}>{"Go Showforminfo page"}</button>
      </div>
    }
}
