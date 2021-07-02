use crate::parsu_frazon;
use yew::prelude::*;
use serde_json::Value;

pub struct RetPaĝo {
    enigo: String,
    traduko: Option<Value>,
    ligilo: ComponentLink<Self>,
}

pub enum Ago {
    Parsu(String),
}

impl Component for RetPaĝo {
    type Message = Ago;
    type Properties = ();

    fn create(_: Self::Properties, ligilo: ComponentLink<Self>) -> Self {
        Self {
            enigo: String::new(),
            traduko: None,
            ligilo,
        }
    }

    fn update(&mut self, mesaĝo: Self::Message) -> ShouldRender {
        match mesaĝo {
            Ago::Parsu(enigo) => {
                self.enigo = enigo.clone();
                self.traduko = Some(parsu_frazon(&enigo));
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let traduko = if let Some(traduko) = &self.traduko { traduko.to_string() } else { String::new() };
        html! {
            <div>
                <textarea
                    oninput=self.ligilo.callback(|enigo: InputData| Ago::Parsu(enigo.value))>
                </textarea>
                <textarea readonly=true
                    value=traduko>
                </textarea>
            </div>
        }
    }
}
