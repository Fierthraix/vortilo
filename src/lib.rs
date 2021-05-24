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
    fn test_adjektivo() {
        let vorto = "aĥtiva hundo estas bela kato";
        let vortilo = Vortilo::parse(Rule::frazo, &vorto);

        println!("{:?}", vortilo);
        assert_eq!(vortilo.unwrap().as_str(), vorto);
        assert!(false);
    }
    #[test]
    fn test_frazo() {
        let vorto = "Mi rapide kuiras per la paton";
        let vortilo = Vortilo::parse(Rule::adverbo, &vorto);
        println!("YYY\n{:?}", vortilo);
        assert!(vortilo.is_ok());
    }
    /*
    #[test]
    fn test_vorto() {
        let vorto = "eĥoŝanĝoĉiuĵaŭde";
        let vortilo = Vortilo::parse(Rule::vorto, &vorto);
        let parsita_vort = vortilo.unwrap().as_str();
        assert_eq!(parsita_vort, vorto);
    }
    #[test]
    fn test_frazo() {
        let my_str = "vorto salution estas mia nomo Adamo";
        let succ2 = Vortilo::parse(Rule::frazo, my_str).unwrap().next().unwrap();

        for (parsita_vort, vorto) in succ2.into_inner().zip(my_str.split_whitespace()) {
            assert_eq!(parsita_vort.as_str(), vorto)
        }
    }
    */
}
