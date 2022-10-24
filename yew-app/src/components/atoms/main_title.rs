use yew::prelude::*;
// use gloo::console::log;


use crate::components::atoms::text_input::TextInput;
use crate::components::atoms::custom_button::CustomButton;

// #[derive(Properties, ParatilEq)]

#[derive(Properties)] 
#[derive(PartialEq)]
pub struct Props
{
	pub title: String,
	pub name:String,
	pub age:u32,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html
{
	// let newonchange: Callback<Event> = Callback::from(|event:Event| {
	// 			let value = event.target()
	// 						.unwrap();
	// 						// .unchecked_into::<HtmlInputElement>()
	// 						// .value();
	// 						log!(value);
	// 		});
	// let str = props.name.clone();
	html!
	{
		<>
		<h1> {&props.title} </h1>
		<h2> {&props.name}</h2>
		<h3> {&props.age} </h3>

		<form >
			<label for="fname">{"First name:"}</label> <br/>
			<input type="text" id="fname" name="fname" value= {props.name.clone()} /><br/>
			<label for="lname">{"Last name:"}</label><br/>
			<input type="text" id="lname" name="lname" value="Doe"/><br/><br/>
			<input type="submit" value="Submit" /><br/>
			<TextInput name = "myname"/>
			<CustomButton label = "oksubmit"/>
	
		</form> 
		</>
	}

}

