use crate::parsu_frazon;
use serde_json::Value;
use web_sys::HtmlTextAreaElement;
use yew::events::InputEvent;
use yew::prelude::*;

pub struct RetPaĝo {
    enigo: String,
    traduko: Value,
}

pub enum Ago {
    Parsu(String),
}

const DEFAŬLTA: &str = "Saluton, al ĉiuj!";

impl Component for RetPaĝo {
    type Message = Ago;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            enigo: String::from(DEFAŬLTA),
            traduko: parsu_frazon(DEFAŬLTA),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, mesaĝo: Self::Message) -> bool {
        match mesaĝo {
            Ago::Parsu(enigo) => {
                self.enigo = enigo.clone();
                self.traduko = parsu_frazon(&enigo);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let traduko = match &self.traduko {
            Value::Array(frazo) => frazo,
            _ => unreachable!(),
        };

        html! {
            <div>
                <textarea class="tekstejo" rows="7" cols="50" placeholder={DEFAŬLTA}
                    oninput={ctx.link().callback(|evento: InputEvent| {
                        Ago::Parsu(evento.target_unchecked_into::<HtmlTextAreaElement>().value())
                   }
               )}>
                </textarea>
                <br />
                <textarea readonly=true class="tekstejo" rows="7" cols="50"
                    value={self.traduko.to_string()}>
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
                { for listo.iter().map(tabel_vico) }
              </table>
              </div>
            </div>
        }
    }
}
