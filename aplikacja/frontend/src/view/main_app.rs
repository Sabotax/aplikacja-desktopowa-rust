use gloo_console::log;
// use gloo::console;
// use js_sys::{Date, WebAssembly::Table};
use yew::{html, Component, Context, Html, classes};
// use std::fmt::{self, Debug, Display};
// Define the possible messages which can be sent to the component

#[derive(Eq, PartialEq)]
pub enum AppState {
    Default,
    Dane,
    Led,
    Table,
    Settings
}

impl AppState {
    fn to_string(&self)-> &str {
        return match self {
            AppState::Default => "Default",
            AppState::Dane => "Dane",
            AppState::Led => "Led",
            AppState::Table => "Table",
            AppState::Settings => "Settings"
        }
    }
}

pub enum PanelState {
    Active,
    Inactive
}

pub struct App {
    state: AppState
}

impl Component for App {
    type Message = AppState;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::Default
            }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("ABC");
        if msg == self.state {
            self.state = AppState::Default;
            true
        }
        else {
            match msg {
            AppState::Default => {
                //nigdy się nie wydarzy
                self.state = AppState::Default;
                true
            }
            AppState::Dane => {
                self.state = AppState::Dane;
                true
            }
            AppState::Led => {
                self.state = AppState::Led;
                true
            }
            AppState::Table => {
                self.state = AppState::Table;
                true
            }
            AppState::Settings => {
                self.state = AppState::Settings;
                true
            }
        }}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div id="mainLeft">
                    <div 
                    id="strona1_panel"
                    class={classes!("panel", match self.state {
                        AppState::Default => "panelInactive",
                        AppState::Dane => "panelActive",
                        AppState::Led => "panelInactive",
                        AppState::Table => "panelInactive",
                        AppState::Settings => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| AppState::Dane)}
                    >  
                        <p>{"Panel Danych"}</p>
                    </div>

                    <div 
                    id="strona2_panel"
                    class={classes!("panel", match self.state {
                        AppState::Default => "panelInactive",
                        AppState::Dane => "panelInactive",
                        AppState::Led => "panelActive",
                        AppState::Table => "panelInactive",
                        AppState::Settings => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| AppState::Led)}
                    >  
                        <p>{"Panel Led"}</p>
                    </div>

                    <div 
                    id="strona3_panel"
                    class={classes!("panel", match self.state {
                        AppState::Default => "panelInactive",
                        AppState::Dane => "panelInactive",
                        AppState::Led => "panelInactive",
                        AppState::Table => "panelActive",
                        AppState::Settings => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| AppState::Table)}
                    >  
                        <p>{"Panel Tabeli"}</p>
                    </div>

                    <div 
                    id="strona4_panel"
                    class={classes!("panel", match self.state {
                        AppState::Default => "panelInactive",
                        AppState::Dane => "panelInactive",
                        AppState::Led => "panelInactive",
                        AppState::Table => "panelInactive",
                        AppState::Settings => "panelActive"
                    })}
                    onclick={ctx.link().callback(|_| AppState::Settings)}
                    >  
                        <p>{"Panel Ustawień"}</p>
                    </div>
                </div>
                <div id="mainRight">
                    {match self.state{
                        AppState::Default => "Default",
                        AppState::Dane => "Dane",
                        AppState::Led => "Led",
                        AppState::Table => "Table",
                        AppState::Settings => "Settings"
                    }}
                </div>
            </div>
        }
    }
}