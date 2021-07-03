use crate::parsu_frazon;
use serde_json::Value;
use yew::prelude::*;

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
        //let traduko = if let Some(traduko) = &self.traduko { to_string_pretty(traduko).unwrap() } else { String::new() };
        let traduko = if let Some(traduko) = &self.traduko {
            match traduko {
                Value::Array(frazo) => frazo,
                _ => return html! { <h1>{"TRADUKO WASN'T AN ARRAY"}</h1> },
            }
        } else {
            return html! {
                <div>
                    <textarea
                        oninput=self.ligilo.callback(|enigo: InputData| Ago::Parsu(enigo.value))>
                    </textarea>
                    <textarea readonly=true></textarea>
                </div>
            };
        };

        html! {
            <div>
                <textarea
                    oninput=self.ligilo.callback(|enigo: InputData| Ago::Parsu(enigo.value))>
                </textarea>
                <textarea readonly=true
                    value=Value::Array(traduko.to_vec()).to_string()>
                </textarea>
                <p>{for traduko.iter().zip(self.enigo.split_whitespace()).map(|(traduko, enigo)| RetPaĝo::bildigi_vorton(enigo, traduko))}</p>
            </div>
        }
    }
}

impl RetPaĝo {
    fn bildigi_vorton(vorto: &str, traduko: &Value) -> Html {
        let listo = if let Value::Array(listo) = traduko {
            listo
        } else {
            return html! { <h1>{"LISTO WASNT A LIST"}</h1> };
        };

        let tabel_vico = |mapo: &Value| {
            let mapo = match mapo {
                Value::Object(mapo) => mapo,
                _ => return html! { <h1>{"MAPO DOESN'T EXIST"}</h1> },
            };
            let (radiko, traduko) = mapo.iter().next().unwrap();
            html! {
                <tr>
                    <td><em>{radiko}</em></td>
                    <td><em>{traduko}</em></td>
                </tr>
            }
        };

        html! {
            <div class="pepo_ujo">
            <p class="pepo_titolo">{vorto}{'\u{00a0}'}</p>
              <div class="pepo_enhavo">
              <table>
                { for listo.iter().map(|mapo| tabel_vico(mapo)) }
              </table>
              </div>
            </div>
        }
    }
}
