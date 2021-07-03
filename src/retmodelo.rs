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
        let nula_vektoro = Vec::with_capacity(0);
        let traduko = if let Some(traduko) = &self.traduko {
            match traduko {
                Value::Array(frazo) => frazo,
                _ => unreachable!(),
            }
        } else {
            &nula_vektoro
        };

        html! {
            <div>
                <textarea class="tekstejo" rows="7" cols="50"
                    oninput=self.ligilo.callback(|enigo: InputData| Ago::Parsu(enigo.value))>
                </textarea>
                <br />
                <textarea readonly=true class="tekstejo" rows="7" cols="50"
                    value=Value::Array(traduko.clone()).to_string()>
                </textarea>
                <p>{for traduko.iter()
                    .zip(self.enigo.split_whitespace())
                    .map(|(traduko, enigo)| RetPaĝo::bildigi_vorton(enigo, traduko))}
                </p>
            </div>
        }
    }
}

impl RetPaĝo {
    fn bildigi_vorton(vorto: &str, traduko: &Value) -> Html {
        let listo = if let Value::Array(listo) = traduko {
            listo
        } else {
            return html! { <h1>{format!("LISTO NE ESTIS LISTO {:?}", traduko)}</h1> };
        };

        let tabel_vico = |mapo: &Value| {
            let mapo = match mapo {
                Value::Object(mapo) => mapo,
                _ => return html! { <h1>{format!("MAPO NE EKZISTAS {:?}", mapo)}</h1> },
            };
            let (radiko, traduko) = mapo.iter().next().unwrap();
            let traduko = if let Value::String(traduko) = traduko {
                traduko
            } else {
                unreachable!()
            };
            html! {
                <tr>
                    <td>{radiko}</td>
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
