extern crate stdweb;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate lazy_static;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use stdweb::web::{IParentNode, document};

lazy_static! {
    static ref blank_image: &'static str = "blank.png";
}

pub struct Model {
    image: String,
    input: String,
}

pub enum Msg {
    Preview,
    Submit,
    Input(String),
}

impl<CTX> Component<CTX> for Model where CTX: AsMut<ConsoleService> {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model {
            image: blank_image.to_string(),
            input: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
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
            }
        }
        true
    }
}

impl<CTX> Renderable<CTX, Model> for Model where CTX: AsMut<ConsoleService> + 'static, {
    fn view(&self) -> Html<CTX, Self> {
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

pub struct Context {
    console: ConsoleService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<_, Model> = App::new(context);
    let mount_point = document().query_selector("#mount").unwrap().unwrap();
    app.mount(mount_point);
    yew::run_loop();
}
