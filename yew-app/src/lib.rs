use gloo::console::log; // for console log
use stylist::{/*style, yew::styled_component,*/Style};
use yew::prelude::*;


pub mod components;

use components::atoms::main_title::MainTitle;

const STYLE_FILE: &str = include_str!("main.css");


#[function_component(App)]
pub fn app() -> Html
{
    let name = "mahmuds";

    log!{name};

    let stylesheet = Style::new(STYLE_FILE).unwrap();

    // let name = String::from("mahmud");
    // let title = String::from("hello world");

    // let s = MainTitle {name:name, title:title, age:100};

    html! {
        <>
        <h1 class = {name}> { "hello rusty world yo yo !!!"} </h1>
        
        <div class={stylesheet}>
            <h1>{"Hello World!!!"}</h1>
            <p>{"more text"}</p>
            <MainTitle title = "title test" name = "mahmud" age = 100 />
        </div>

        // <h2>{"HTML Forms"}</h2>

        // <form >
        // <label for="fname">{"First name:"}</label> <br/>
        // <input type="text" id="fname" name="fname" value="John"/><br/>
        // <label for="lname">{"Last name:"}</label><br/>
        // <input type="text" id="lname" name="lname" value="Doe"/><br/><br/>
        // <input type="submit" value="Submit"/>
        // </form> 

        // <p>{"If you click the Submit button, the form-data will be sent"}</p>


        </>
    }
}