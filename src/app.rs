use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement, Window};
use yew::prelude::*;

pub struct App {
    data: String,
}

pub enum Msg {
    InputTitle(String),
    Hover,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputTitle(arg) => {
                self.data = arg;
                true
            }
            Msg::Hover => {
                log::info!("{:?}", self.data.clone());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let input = link.batch_callback(|e: InputEvent| {
            let target = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            target.map(|input| Msg::InputTitle(input.value()))
        });
        let textarea = link.batch_callback(|e: InputEvent| {
            let target = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            target.map(|input| Msg::InputTitle(input.value()))
        });
        let hover = link.batch_callback(|e: MouseEvent| {
            let target: Option<EventTarget> = e.target();
            let hover = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            hover.map(|_| Msg::Hover)
        });
        let hover_textarea = link.batch_callback(|e: MouseEvent| {
            let target = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            target.map(|_| Msg::Hover)
        });

        html! {
            <>
                <input type="text" oninput={ input } onmouseover={ hover } />
                <textarea oninput={ textarea } onmouseover={ hover_textarea } />
                <p>{ self.data.clone() }</p>
            </>
        }
    }
}
