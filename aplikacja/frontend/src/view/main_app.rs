use gloo_console::log;
use web_sys::console;
// use gloo::console;
// use js_sys::{Date, WebAssembly::Table};
use yew::{html, Component, Context, Html, classes, UseStateHandle, use_state, use_effect_with_deps};
use std::borrow::Borrow;
use std::error::Error;
use std::marker::Copy;
use reqwasm::http::{Request, Headers,ReferrerPolicy,RequestMode};
use js_sys::JsString;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use crate::hello;
use yew::prelude::*;
use crate::network::network_service;
use yew::Hook;
use crate::view::data_component::TestKomponent;
use crate::view::panel_component::PanelKomponent;
use crate::view::panel_component::PanelSwitchOutcome;
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

        let on_panel_select = Callback::from(move |action: &'static PanelSwitchOutcome| {
            //TODO
        });

        html! {
            <div>
                <div id="mainLeft">
                    <PanelKomponent
                        id= {"strona1_panel".to_string()}
                        label= {"Panel Danych"}
                        associated_action= {&PanelSwitchOutcome::Dane}
                        active= {false}
                        on_click= {on_panel_select}
                    />

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
                    <TestKomponent/>
                </div>
                <div id="mainRight">
                    {match self.left_state{
                        LeftState::Default => "Def",
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