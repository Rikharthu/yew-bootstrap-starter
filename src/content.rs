use yew::prelude::*;

pub struct Content {}

impl Component for Content {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Content {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="container">
                <div class="starter-template">
                    <h1>{"Bootstrap starter template"}</h1>
                    <p class="lead">
                        {"Use this document as a way to quickly start any new project."}
                        <br/>
                        {"All you get is this text and a mostly barebones HTML document."}
                    </p>
                </div>
            </main>
        }
    }
}