#[macro_use]
extern crate pest_derive;
extern crate pest;

use pest::iterators::Pair;
use serde_json::map::Map;
use serde_json::{json, Value};

macro_rules! mapo {
    ($nomo:expr, $datumo:expr) => {{
        let mut mapo = Map::new();
        mapo.insert($nomo.to_string(), Value::String($datumo.to_string()));
        Value::Object(mapo)
    }};
}

macro_rules! akuzativo {
    () => {
        mapo!("n", "Accusative")
    };
}

macro_rules! plurala {
    () => {
        mapo!("j", "Plural")
    };
}

#[derive(Parser)]
#[grammar = "vort.pest"]
pub struct Vortilo;

pub fn kreu_propraĵoj(paro: Pair<'_, Rule>) -> Value {
    let paro = paro.into_inner().next().unwrap();
    println!("{:?}", paro);
    match paro.as_rule() {
        Rule::tabelvorto => parsu_tabelvorton(paro),
        Rule::pronomo => parsu_pronomon(paro),
        Rule::e_vorteto | Rule::rolvorteto | Rule::gramatika_vorteto | Rule::ekkriita_vorto => {
            parsu_konstantan_vorton(paro)
        }
        Rule::nombro => parsu_nombron(paro),
        Rule::adjektivo | Rule::substantivo | Rule::adverbo => parsu_normalan(paro),
        Rule::verbo => parsu_verbon(paro),
        _ => unreachable!(),
    }
}

fn parsu_tabelvorton(paro: Pair<'_, Rule>) -> Value {
    let mut paro = paro.into_inner();
    // Atingu kaj prefikson kaj postfikson.
    let prefikso = paro.next().unwrap();
    let postfikso = paro.next().unwrap();

    let mut rez = vec![
        mapo!("tb_prefikso", prefikso.as_str()),
        mapo!("tb_postfikso", postfikso.as_str()),
    ];

    // Kontrolu por akuzativo kaj plurala.
    if postfikso.as_rule() == Rule::sxangxebla_postfikso {
        for nov_paro in paro {
            match nov_paro.as_rule() {
                Rule::plural_fino => {
                    rez.push(plurala!());
                }
                Rule::akuzativa_fino => {
                    rez.push(akuzativo!());
                }
                _ => unreachable!(),
            }
        }
    }

    Value::Array(rez)
}

fn parsu_pronomon(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_konstantan_vorton(paro: Pair<'_, Rule>) -> Value {
    let konstanto = paro.as_str();

    let konstantaj_vortoj: Value = serde_json::from_str(include_str!("konstantaj.json")).unwrap();

    match konstantaj_vortoj {
        Value::Object(mapo) => {
            // Kontrolu tra la vortoj.
            for (vorto, defino) in &mapo {
                if vorto == konstanto {
                    return Value::Array(vec![mapo!(vorto, defino)]);
                }
            }
        }
        _ => (),
    };
    json!({})
}

fn parsu_nombron(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_normalan(paro: Pair<'_, Rule>) -> Value {
    let ena = paro.into_inner().next().unwrap();

    let (akuzativo, plurala, radiko) = match ena.as_rule() {
        Rule::ne_o | Rule::ne_a | Rule::ne_e | Rule::ne_apostrof => {
            (false, false, &ena.as_str()[0..ena.as_str().len() - 1])
        }
        Rule::ne_oj | Rule::ne_aj => (false, true, &ena.as_str()[0..ena.as_str().len() - 2]),
        Rule::ne_on | Rule::ne_an | Rule::ne_en => {
            (true, false, &ena.as_str()[0..ena.as_str().len() - 2])
        }
        Rule::ne_ajn | Rule::ne_ojn => (true, false, &ena.as_str()[0..ena.as_str().len() - 3]),
        _ => unreachable!(),
    };

    let radiko_desc = "TODO";

    let mut rez = vec![mapo!(radiko, radiko_desc)];

    if akuzativo {
        rez.push(akuzativo!());
    }

    if plurala {
        rez.push(plurala!());
    }

    Value::Array(rez)
}

fn parsu_verbon(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pest::Parser;

    macro_rules! testu_samspecon {
        ($frazo:expr, $regulo:expr) => {
            let vortilo = Vortilo::parse(Rule::frazo, $frazo)
                .unwrap()
                .next()
                .unwrap()
                .into_inner();
            for (vort_paro, vorto) in vortilo.zip($frazo.split_whitespace()) {
                let substantivo = vort_paro.into_inner().next().unwrap();
                assert_eq!(substantivo.as_str(), vorto);
                assert_eq!(substantivo.as_rule(), $regulo);
            }
        };
    }

    #[test]
    fn test_substantivo() {
        let frazo = "aktivo hundojn estantaĵoj belon katon ĝu'";

        testu_samspecon!(&frazo, Rule::substantivo);
    }
    #[test]
    fn test_adjektivo() {
        let frazo = "aktiva hundajn estantaj belan katan ĝua";
        testu_samspecon!(&frazo, Rule::adjektivo);
    }
    #[test]
    fn test_adverbo() {
        let frazo = "aktive hunde estante belen katen ĝue";
        testu_samspecon!(&frazo, Rule::adverbo);
    }
    #[test]
    fn test_verbo() {
        let frazo = "aktivi hundu estantis belas katos ĝuus";
        testu_samspecon!(&frazo, Rule::verbo);
    }
    #[test]
    fn test_tabelvorto() {
        let frazo = "tiajn neniom iom kiam";
        testu_samspecon!(&frazo, Rule::tabelvorto);
    }
    #[test]
    fn test_frazo() {
        let vorto = "Mi rapide kuiras per la pato";
        let vortilo = Vortilo::parse(Rule::frazo, &vorto);
        assert_eq!(vortilo.unwrap().as_str(), vorto);
    }
}
