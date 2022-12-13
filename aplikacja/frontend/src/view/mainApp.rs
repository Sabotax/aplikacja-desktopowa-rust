use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html, classes};
// Define the possible messages which can be sent to the component
pub enum SwitchAction {
    Panel1,
    Panel2,
}

pub enum PanelState {
    Active,
    Inactive
}

pub struct Panele {
    panel1: PanelState,
    panel2: PanelState
}

pub struct App {
    value: i64, // This will store the counter value
    panelRight: Html,
    panele: Panele
}

impl Component for App {
    type Message = SwitchAction;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { 
            value: 0 ,
             panelRight: html!{},
             panele: Panele{
                panel1:PanelState::Inactive,
                panel2:PanelState::Inactive
            }
            }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SwitchAction::Panel1 => {
                // self.value += 1;
                // console::log!("plus one"); // Will output a string to the browser console
                // true // Return true to cause the displayed change to update
                self.panelRight = html! {
                    "Twoja stara"
                };
                self.panele.panel1 = PanelState::Active;
                self.panele.panel2 = PanelState::Inactive;
                true
            }
            SwitchAction::Panel2 => {
                self.panelRight = html! {
                    "Twoja stara2"
                };
                self.panele.panel2 = PanelState::Active;
                self.panele.panel1 = PanelState::Inactive;
                true
            }
        }
    }

    // TODO zrobić zamiast if żeby było PANEL STATE i mu dać match

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div id="mainLeft">
                    <div 
                    id="strona1_panel"
                    class={classes!("panel", match self.panele.panel1 {
                            PanelState::Active => "panelActive",
                            PanelState::Inactive => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| SwitchAction::Panel1)}
                    >  
                        <p>{"Panel 1"}</p>
                    </div>

                    <div 
                    id="strona2_panel"
                    class={classes!("panel", match self.panele.panel2 {
                            PanelState::Active => "panelActive",
                            PanelState::Inactive => "panelInactive"
                    })}
                    onclick={ctx.link().callback(|_| SwitchAction::Panel2)}
                    >  
                        <p>{"Panel 2"}</p>
                    </div>
                </div>
                <div id="mainRight">
                    {self.panelRight.clone()}
                </div>
            </div>
            // <div>
            //     <div class="panel">
            //         // A button to send the Increment message
            //         <button class="button" onclick={ctx.link().callback(|_| Msg::Increment)}>
            //             { "+1" }
            //         </button>

            //         // A button to send the Decrement message
            //         <button onclick={ctx.link().callback(|_| Msg::Decrement)}>
            //             { "-1" }
            //         </button>

            //         // A button to send two Increment messages
            //         <button onclick={ctx.link().batch_callback(|_| vec![Msg::Increment, Msg::Increment])}>
            //             { "+1, +1" }
            //         </button>

            //     </div>

            //     // Display the current value of the counter
            //     <p class="counter">
            //         { self.value }
            //     </p>

            //     // Display the current date and time the page was rendered
            //     <p class="footer">
            //         { "Rendered: " }
            //         { String::from(Date::new_0().to_string()) }
            //     </p>
            // </div>
        }
    }
}