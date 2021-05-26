#[macro_use]
extern crate pest_derive;
extern crate pest;

use std::collections::HashMap;

use pest::Span;
use pest::RuleType;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "vort.pest"]
pub struct Vortilo;


pub fn kreu_propraĵoj<'a>(paro: Pair<'a, Rule>) -> Vec<(String, String)>{
    let paro = paro.into_inner().next().unwrap();
    match paro.as_rule() {
        Rule::tabelvorto => parsu_tabelvorton(&paro),
        Rule::pronomo => parsu_pronomon(&paro),
        Rule::e_vorteto => parsu_e_vorteton(&paro),
        Rule::rolvorteto => parsu_rolvorteton(&paro),
        Rule::gramatika_vorteto => parsu_gramatikan_vorteton(&paro),
        Rule:: ekkriita_vorto => parsu_ekkriitan(&paro),
        Rule::nombro => parsu_nombron(&paro),
        Rule::adjektivo => parsu_adjektivon(&paro),
        Rule::substantivo => parsu_substantivon(&paro),
        Rule::adverbo => parsu_adverbon(&paro),
        Rule::verbo => parsu_verbon(&paro),
        _ => unreachable!()
    }
}

fn parsu_tabelvorton<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    let mut paro = paro.into_inner();
    let prefikso = paro.next().unwrap();
    let postfikso = paro.next().unwrap();
    vec![]
}

fn parsu_pronomon<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_e_vorteton<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_rolvorteton<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_gramatikan_vorteton<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_ekkriitan<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_nombron<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_adjektivon<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_substantivon<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_adverbon<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn parsu_verbon<'a>(paro: &Pair<'a, Rule>) -> Vec<(String, String)> {
    vec![]
}

fn akzukativo() -> (String, String) {
    ("n".to_string(), "Accusative".to_string())
}

fn plurala() -> (String, String) {
    ("j".to_string(), "Plural".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pest::Parser;
    #[test]
    fn test_substantivo() {
        let frazo = "aktivo hundojn estantaĵoj belon katon ĝu'";
        let vortilo = Vortilo::parse(Rule::frazo, &frazo).unwrap().next().unwrap().into_inner();

        for (vort_paro, vorto) in vortilo.zip(frazo.split_whitespace()) {
            let substantivo = vort_paro.into_inner().next().unwrap();
            assert_eq!(substantivo.as_str(), vorto);
            assert_eq!(substantivo.as_rule(), Rule::substantivo);
        }
    }
    #[test]
    fn test_adjektivo() {
        let frazo = "aktiva hundajn estantaj belan katan ĝua";
        let vortilo = Vortilo::parse(Rule::frazo, &frazo).unwrap().next().unwrap().into_inner();

        for (vort_paro, vorto) in vortilo.zip(frazo.split_whitespace()) {
            let adjektivo = vort_paro.into_inner().next().unwrap();
            assert_eq!(adjektivo.as_str(), vorto);
            assert_eq!(adjektivo.as_rule(), Rule::adjektivo);
        }
    }
    #[test]
    fn test_adverbo() {
        let frazo = "aktive hunde estante belen katen ĝue";
        let vortilo = Vortilo::parse(Rule::frazo, &frazo).unwrap().next().unwrap().into_inner();

        for (vort_paro, vorto) in vortilo.zip(frazo.split_whitespace()) {
            let adverbo = vort_paro.into_inner().next().unwrap();
            assert_eq!(adverbo.as_str(), vorto);
            assert_eq!(adverbo.as_rule(), Rule::adverbo);
        }
    }
    #[test]
    fn test_verbo() {
        let frazo = "aktivi hundu estantis belas katos ĝuus";
        let vortilo = Vortilo::parse(Rule::frazo, &frazo).unwrap().next().unwrap().into_inner();

        println!("{:?}", vortilo);

        for (vort_paro, vorto) in vortilo.zip(frazo.split_whitespace()) {
            let verbo = vort_paro.into_inner().next().unwrap();
            assert_eq!(verbo.as_str(), vorto);
            assert_eq!(verbo.as_rule(), Rule::verbo);
        }
    }
    #[test]
    fn test_frazo() {
        let vorto = "Mi rapide kuiras per la pato";
        let vortilo = Vortilo::parse(Rule::frazo, &vorto);
        assert_eq!(vortilo.unwrap().as_str(), vorto);
    }
}
