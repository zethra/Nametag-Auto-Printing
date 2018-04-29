extern crate stdweb;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate serde_json;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, Request, Response};
use yew::format::{Json, Restorable};
use stdweb::web::{IParentNode, document};

pub struct Model {
    image: String,
    input: String,
}

pub enum Msg {
    Preview,
    Submit,
    Input(String),
    Error,
    Noop
}

pub struct Context {
    console: ConsoleService,
    fetch: FetchService
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn print_err<OUT: 'static>(resp: Response<OUT>) where OUT: From<Restorable> {

}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model {
            image: "blank.png".to_string(),
            input: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Preview => {
                self.image = format!("preview?name={}", self.input);
                context.as_mut().log(format!("Preview: {}", self.input).as_str());
                self.input = "".to_string();
            },
            Msg::Input(data) => {
                context.as_mut().log(&self.image);
                self.input = data;
                context.as_mut().log(format!("Format: {}", self.input).as_str());
            },
            Msg::Submit => {
                context.as_mut().log("Submit");
                let req = Request::post("submit")
                                    .header("Content-Type", "application/json")
                                    .body(Json(&json!({"name": "Ben"})))
                                    .expect("Failed to build request");
                context.fetch.fetch(req, print_err.into());
            }
        }
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <img id="img", src=&self.image,/>
            </div>
            <div>
                <p class="title",>{ "Name:" }</p>
                <input id="input", autofocus="true", maxLength="10", type="text", value=&self.input, oninput=|e: InputData| Msg::Input(e.value),/>
                <p class="subtitle",>{ r#"(10 characters or less, no \, #, or %)"# }</p>
                <div class="btns",>
                    <button id="preview", onclick=|_| Msg::Preview,>{ "Preview" }</button>
                    <button id="submit", onclick=|_| Msg::Submit,>{ "Submit" }</button>
                </div>
            </div>
        }
    }
}


fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
        fetch: FetchService::new()
    };
    let app: App<_, Model> = App::new(context);
    let mount_point = document().query_selector("#mount").unwrap().unwrap();
    app.mount(mount_point);
    yew::run_loop();
}
