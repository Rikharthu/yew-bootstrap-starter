use yew::prelude::*;

pub struct Navigation {}

impl Component for Navigation {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Navigation {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar navbar-expand-md navbar-dark bg-dark fixed-top">
                <a class="navbar-brand" href="#">{"Navbar"}</a>
                <button class="navbar-toggler">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarsExampleDefault">
                    <ul class="navbar-nav mr-auto">
                        <li class="nav-item active">
                            <a class="nav-link" href="#">
                                {"Home "}<span class="sr-only">{"(current)"}</span>
                            </a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="https://example.com">{"Link"}</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link disabled" href="#">{"Disabled"}</a>
                        </li>
                    </ul>
                    <form class="form-inline my-2 my-lg-0">
                        <input class="form-control mr-sm-2" type="text" placeholder="Search" />
                        <button class="btn btn-outline-success my-2 my-sm-0" type="submit">{"Search"}</button>
                    </form>
                </div>
            </nav>
        }
    }
}