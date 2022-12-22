use std::iter::once;

use yew::{Properties,Callback,function_component,Html,html,classes};

#[derive(PartialEq,Copy,Clone)]
pub enum PanelSwitchOutcome {
    Default,
    Dane,
    Led,
    Table,
    Settings
}

#[derive(Properties, PartialEq)]
pub struct PanelKomponentProps {
    pub id: String,
    pub label: String,
    pub associated_action: &'static PanelSwitchOutcome,
    pub active: bool,
    pub on_click: Callback<PanelSwitchOutcome>
}

#[function_component(PanelKomponent)]
pub fn panel_komponent(PanelKomponentProps { id, label, associated_action, active, on_click }: &PanelKomponentProps) -> Html {
    
    let call = Callback::from( move |_| {
        on_click.emit(*associated_action)
    });

    return html!(
        <div 
            id={id.clone()}
            class={classes!("panel", match active {
                true => "panelActive",
                false => "panelInactive"
            })}
            onclick={call}>  
                <p>{label}</p>
        </div>
    )
}

