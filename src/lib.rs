use logos::{Lexer, Logos};
use serde_json::{json, Value};

#[derive(Logos, Debug, PartialEq)]
pub enum Vorto {
    #[regex("[a-zĥŝĝĉĵŭ]+", gramatika)]
    GramatikaVorto(Value),

    #[regex("[a-zĥŝĝĉĵŭ]+(ajn|aj|an|a)", adjektivo)]
    Adjektivo(Value),

    #[regex("[a-zĥŝĝĉĵŭ]+(ojn|oj|on|o)", substantivo)]
    Substantivo(Value),

    #[regex("[a-zĥŝĝĉĵŭ]+(en|e)", adverbo)]
    Adverbo(Value),

    #[regex("[a-zĥŝĝĉĵŭ]+(i|us|is|os|as|u)", verbo)]
    Verbo(Value),

    #[error]
    #[regex(" ", logos::skip)]
    Eraro,
}

fn gramatika(lex: &mut Lexer<Vorto>) -> Option<Value> {
    let gramatikaj_vortoj: Value = serde_json::from_str(include_str!("./konstantaj.json")).unwrap();
    let vorto = lex.slice();

    match &gramatikaj_vortoj[vorto] {
        Value::String(stringo) => Some(json!({vorto: stringo})),
        _ => None,
    }
}

fn adjektivo(lex: &mut Lexer<Vorto>) -> Option<Value> {

    let vorto = lex.slice();

    // Speciala kazo.
    if vorto == "kaj" {
        return gramatika(lex);
    }

    // Kontroli ĉu la vorto estas sufiĉe longa.
    if vorto.len() <= 3 {
        return None
    }

    let (akuzativa, plurala) = if vorto.ends_with("ajn") {
        (true, true)
    } else if vorto.ends_with("an") {
        (true, false)
    } else if vorto.ends_with("aj") {
        (false, true)
    } else if vorto.ends_with("a")  {
        (false, false)
    } else{
        return None;
    };

    Some(json!({
        vorto: "z",
        "akuzativa": akuzativa,
        "plurala": plurala,
    }))
}

fn substantivo(lex: &mut Lexer<Vorto>) -> Option<Value> {
    let vorto = lex.slice();

    let (akuzativa, plurala) = if vorto.ends_with("ojn") {
        (true, true)
    } else if vorto.ends_with("on") {
        (true, false)
    } else if vorto.ends_with("oj") {
        (false, true)
    } else if vorto.ends_with("o") || vorto.ends_with("'")  {
        (false, false)
    } else{
        return None;
    };

    Some(json!({
        vorto: "z",
        "akuzativa": akuzativa,
        "plurala": plurala,
    }))
}

fn adverbo(lex: &mut Lexer<Vorto>) -> Option<Value> {
    Some(json!({
        lex.slice(): "adverbo"
    }))
}

fn verbo(lex: &mut Lexer<Vorto>) -> Option<Value> {
    Some(json!({
        lex.slice(): "verbo"
    }))
}
