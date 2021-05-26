#[macro_use]
extern crate pest_derive;
extern crate pest;

//use std::collections::HashMap;

//use pest::Span;
//use pest::RuleType;
use pest::iterators::Pair;
use serde_json::{json, Value};


fn akuzativo(json: &mut Value, akuzativ: bool) {
    if akuzativ {
        match json {
            Value::Object(map) => {
                map.insert("n".to_string(), Value::String("Accusative".to_string()));
            },
            _ => (),
        }
    }
}

#[derive(Parser)]
#[grammar = "vort.pest"]
pub struct Vortilo;

pub fn kreu_propraĵoj<'a>(paro: Pair<'a, Rule>) -> Value {
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

fn parsu_tabelvorton<'a>(paro: Pair<'a, Rule>) -> Value {
    let mut paro = paro.into_inner();
    let prefikso = paro.next().unwrap();
    let postfikso = paro.next().unwrap();

    let mut rez = json!({
        prefikso.as_str(): prefikso.as_str(),
        postfikso.as_str(): postfikso.as_str()
    });

    akuzativo(&mut rez, postfikso.as_str().contains('n'));
    //akuzativo!(rez, postfikso.as_str().contains('j'));

    rez
}

fn parsu_pronomon<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_e_vorteton<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_rolvorteton<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_gramatikan_vorteton<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_ekkriitan<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_nombron<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_adjektivon<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_substantivon<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_adverbon<'a>(paro: Pair<'a, Rule>) -> Value {
    json!({})
}

fn parsu_verbon<'a>(paro: Pair<'a, Rule>) -> Value {
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
        }
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
