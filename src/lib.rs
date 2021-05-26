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

#[derive(Parser)]
#[grammar = "vort.pest"]
pub struct Vortilo;

pub fn kreu_propraĵoj(paro: Pair<'_, Rule>) -> Value {
    let paro = paro.into_inner().next().unwrap();
    match paro.as_rule() {
        Rule::tabelvorto => parsu_tabelvorton(paro),
        Rule::pronomo => parsu_pronomon(paro),
        Rule::e_vorteto => parsu_e_vorteton(paro),
        Rule::rolvorteto => parsu_rolvorteton(paro),
        Rule::gramatika_vorteto => parsu_gramatikan_vorteton(paro),
        Rule::ekkriita_vorto => parsu_ekkriitan(paro),
        Rule::nombro => parsu_nombron(paro),
        Rule::adjektivo => parsu_adjektivon(paro),
        Rule::substantivo => parsu_substantivon(paro),
        Rule::adverbo => parsu_adverbon(paro),
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
                    rez.push(mapo!("j", "Plural"));
                }
                Rule::akuzativa_fino => {
                    rez.push(mapo!("n", "Accusative"));
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

fn parsu_e_vorteton(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_rolvorteton(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_gramatikan_vorteton(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_ekkriitan(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_nombron(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_adjektivon(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_substantivon(_paro: Pair<'_, Rule>) -> Value {
    json!({})
}

fn parsu_adverbon(_paro: Pair<'_, Rule>) -> Value {
    json!({})
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
