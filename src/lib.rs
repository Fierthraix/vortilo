#[macro_use]
extern crate pest_derive;
extern crate pest;

use pest::Span;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "vort.pest"]
pub struct Vortilo;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pest::Parser;
    #[test]
    fn test_substantivo() {
        let frazo = "aktivo hundojn estantaĵoj belon katon";
        let vortilo = Vortilo::parse(Rule::frazo, &frazo).unwrap().next().unwrap().into_inner();

        for (vort_paro, vorto) in vortilo.zip(frazo.split_whitespace()) {
            let substantivo = vort_paro.into_inner().next().unwrap();
            assert_eq!(substantivo.as_str(), vorto);
            assert_eq!(substantivo.as_rule(), Rule::substantivo);
        }
    }
    #[test]
    fn test_adjektivo() {
        let frazo = "aktiva hundajn estantaj belan katan";
        let vortilo = Vortilo::parse(Rule::frazo, &frazo).unwrap().next().unwrap().into_inner();

        for (vort_paro, vorto) in vortilo.zip(frazo.split_whitespace()) {
            let adjektivo = vort_paro.into_inner().next().unwrap();
            assert_eq!(adjektivo.as_str(), vorto);
            assert_eq!(adjektivo.as_rule(), Rule::adjektivo);
        }
    }
    #[test]
    fn test_adverbo() {
        let frazo = "aktive hunde estante belen katen";
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
