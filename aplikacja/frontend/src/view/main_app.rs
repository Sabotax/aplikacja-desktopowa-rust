use gloo_console::log;
// use gloo::console;
// use js_sys::{Date, WebAssembly::Table};
use yew::{html, Component, Context, Html, classes};
use std::marker::Copy;
// use std::fmt::{self, Debug, Display};
// Define the possible messages which can be sent to the component

#[derive(Eq, PartialEq,Copy,Clone)]
pub enum LeftState {
    Dane,
    Led,
    Table,
    Settings,
    Default
}
#[derive(Eq, PartialEq)]
pub enum RightState {
    Dane(DaneState),
    Led(LedState),
    Table(TableState),
    Settings(SettingsState),
    Default
}
#[derive(Eq, PartialEq)]
pub enum DaneState {
    GetHumidity
}
#[derive(Eq, PartialEq)]
pub enum LedState {
    SetSingle,
    SetAll,
    GetSingle,
    GetAll
}
#[derive(Eq, PartialEq)]
pub enum TableState {
    GetRow
}
#[derive(Eq, PartialEq)]
pub enum SettingsState {
    SetIP
}
#[derive(Eq, PartialEq)]
pub enum AppMsg {
    Left(LeftState),
    Right(RightState)
}
#[derive(Eq, PartialEq)]
pub enum PanelState {
    Active,
    Inactive
}

pub struct App {
    left_state: LeftState,
    right_state: RightState,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            left_state: LeftState::Default,
            right_state: RightState::Default
            }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log!("ABC");

        if msg == AppMsg::Left(self.left_state) {
            self.left_state = LeftState::Default;
            self.right_state = RightState::Default;
            return true
        }

        match msg {
            AppMsg::Left(LeftState::Default) => {
                //nigdy się nie wydarzy
                false
            }
            AppMsg::Left(LeftState::Dane) => {
                self.left_state = LeftState::Dane;
                true
            }
            AppMsg::Left(LeftState::Led) => {
                self.left_state = LeftState::Led;
                true
            }
            AppMsg::Left(LeftState::Table) => {
                self.left_state = LeftState::Table;
                true
            }
            AppMsg::Left(LeftState::Settings) => {
                self.left_state = LeftState::Settings;
                true
            }
            AppMsg::Right(RightState::Default) => {
                //nie wydarzy sie
                false
            }
            AppMsg::Right(RightState::Dane(DaneState::GetHumidity)) => {
                //TODO
                true
            }
            AppMsg::Right(RightState::Led(LedState::SetAll)) => {
                //TODO
                true
            }
            AppMsg::Right(RightState::Led(LedState::SetSingle)) => {
                //TODO
                true
            }
            AppMsg::Right(RightState::Led(LedState::GetAll)) => {
                //TODO
                true
            }
            AppMsg::Right(RightState::Led(LedState::GetSingle)) => {
                //TODO
                true
            }
            AppMsg::Right(RightState::Table(TableState::GetRow)) => {
                //TODO
                true
            }
            AppMsg::Right(RightState::Settings(SettingsState::SetIP)) => {
                //TODO
                true
            }
        }
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div id="mainLeft">
                    <div 
                    id="strona1_panel"
                    class={classes!("panel", match self.left_state {
                        LeftState::Default => "panelInactive",
                        LeftState::Dane => "panelActive",
                        LeftState::Led => "panelInactive",
                        LeftState::Table => "panelInactive",
                        LeftState::Settings => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| AppMsg::Left(LeftState::Dane))}
                    >  
                        <p>{"Panel Danych"}</p>
                    </div>

                    <div 
                    id="strona2_panel"
                    class={classes!("panel", match self.left_state {
                        LeftState::Default => "panelInactive",
                        LeftState::Dane => "panelInactive",
                        LeftState::Led => "panelActive",
                        LeftState::Table => "panelInactive",
                        LeftState::Settings => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| AppMsg::Left(LeftState::Led))}
                    >  
                        <p>{"Panel Led"}</p>
                    </div>

                    <div 
                    id="strona3_panel"
                    class={classes!("panel", match self.left_state {
                        LeftState::Default => "panelInactive",
                        LeftState::Dane => "panelInactive",
                        LeftState::Led => "panelInactive",
                        LeftState::Table => "panelActive",
                        LeftState::Settings => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| AppMsg::Left(LeftState::Table))}
                    >  
                        <p>{"Panel Tabeli"}</p>
                    </div>

                    <div 
                    id="strona4_panel"
                    class={classes!("panel", match self.left_state {
                        LeftState::Default => "panelInactive",
                        LeftState::Dane => "panelInactive",
                        LeftState::Led => "panelInactive",
                        LeftState::Table => "panelInactive",
                        LeftState::Settings => "panelActive"
                    })}
                    onclick={ctx.link().callback(|_| AppMsg::Left(LeftState::Settings))}
                    >  
                        <p>{"Panel Ustawień"}</p>
                    </div>
                </div>
                <div id="mainRight">
                    {match self.left_state{
                        LeftState::Default => "Default",
                        LeftState::Dane => "Dane",
                        LeftState::Led => "Led",
                        LeftState::Table => "Table",
                        LeftState::Settings => "Settings"
                    }}
                </div>
            </div>
        }
    }
}