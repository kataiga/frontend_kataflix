use yew::prelude::*;

use crate::components::greating::Greating;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div>
            <Greating/>
            <h2 class={"heading"}>{"Kataiga !"}</h2>
        </div>
    }
}