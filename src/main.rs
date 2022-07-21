#![allow(unused_doc_comments)]

use yew::prelude::*; // bring in all the newcessary modules to build a web front end with rust

struct Counter {
    value: i64
}

/*
*  Much like React.js, 
*  the Yew framework supports functional programming
* through the use of appropriate macros.
*/
#[function_component(App)]
fn app() -> Html {
    /// app react function component that returns
    /// an Html component
    /// 
    /// # Args:
    /// *   none
    /// # Returns:
    ///     Html component
    
    // create the state hooks (much like the state of React.js components)
    let state = use_state(|| Counter{
        value : 0
    });
    // create an onClick method to handle the clicking of
    // a button
    let on_click_increment = {
        let state = state.clone();  // create a clone, like we're using const state = state.slice
        // create a callback function for the closure attributed to on_counter_click function
        Callback::from(move |_| {   // the '_' means that we don't care about the parameters passed to the created closure
            state.set(Counter {     // the 'move' keyword allows the Callback closure to take ownership of all variables referenced inside our closure
                value: state.value + 1 // because Rust borrows references and not the variable itself by default
            })
        }) // there is no semicolon because the on_counter_click has to return the callback method
    };

    let on_click_decrement = {
        let state = state.clone();
        Callback::from(move |_| {
            state.set(Counter {
                value: state.value - 1
            })
        })
    };

    // now, return the HTML render, much like in any other App
    html! {
        <div>
            <button onclick={ on_click_increment }> {"+1"} </button> // increment the counter
            <button onclick={ on_click_decrement }> {"-1"} </button> // decrement the counter
            <p>{ format!( "Current counter state: {}", state.value) }</p> // print the counter state
        </div>
    } // no semicolon so that app returns this Html component
}

// create a main function to tell Yew framework what the root of our front end application is
fn main(){
    yew::start_app::<App>();
}