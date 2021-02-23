#![recursion_limit = "1000"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use content::Content;
use navigation::Navigation;

mod content;
mod navigation;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Navigation />
                <Content />
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}