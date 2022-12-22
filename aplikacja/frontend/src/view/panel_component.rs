use std::iter::once;

use yew::{Properties,Callback,function_component,Html,html,classes,Component,Context};

#[derive(PartialEq,Copy,Clone)]
pub enum PanelSwitchOutcome {
    Default,
    Dane,
    Led,
    Table,
    Settings
}

impl From<PanelSwitchOutcome> for () {
    fn from(_: PanelSwitchOutcome) -> Self {
        ()
    }
}

// #[derive(Properties, PartialEq)]
// pub struct PanelKomponentProps {
//     pub id: String,
//     pub label: String,
//     pub associated_action: &'static PanelSwitchOutcome,
//     pub active: bool,
//     pub on_click: Callback<PanelSwitchOutcome>
// }

#[derive(Properties,PartialEq)]
pub struct PanelProps<'parent> {
    pub id: String,
    pub label: String,
    pub associated_action: &'parent PanelSwitchOutcome,
    pub active: bool,
    pub callback: Callback<PanelSwitchOutcome>
}

pub struct PanelKomponent<'parent> {
    pub props: PanelProps<'parent>
}

impl Component for PanelKomponent {
    type Message = ();
    type Properties = PanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            props: PanelProps{
                id: "def".to_string(),
                label: "def".to_string(),
                associated_action: PanelSwitchOutcome::Default,
                active: false,
                callback: Callback::noop()
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        //let local_callback = self.callback.clone();
        html! {
            <button onclick={ctx.link().callback(|_| self.props.associated_action.clone())}>{ "Click me" }</button>
        }
    }
}
