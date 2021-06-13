#[macro_use]
extern crate lazy_static;

use serde_json::{json, Value};

lazy_static! {
    static ref KONSTANTAJ: Value = serde_json::from_str(include_str!("./konstantaj.json")).unwrap();
    static ref TABEL_VORTOJ: Value = serde_json::from_str(include_str!("./tabelvortoj.json")).unwrap();
    static ref PRONOMOJ: Value = serde_json::from_str(include_str!("./pronomoj.json")).unwrap();
}

macro_rules! trancxi {
    ($vorto:expr, $nombro:expr) => {
        &$vorto[..$vorto.as_bytes().iter().count() - $nombro]
    }
}

#[derive(Debug, PartialEq)]
pub enum Vorto {
    GramatikaVorto(Value),
    Adjektivo(Value),
    Substantivo(Value),
    Adverbo(Value),
    Verbo(Value),
    Eraro,
}

enum VortSpeco {
    GramatikaVorto,
    Adjektivo,
    Substantivo,
    Adverbo,
    Verbo,
}

pub fn parsu_vorton(vorto: &str) -> Value {
    // Kontrolu ĉu la vorto estas gramatika.
    match gramatika(vorto) {
        Some(valuo) => return valuo,
        None => (),
    }

    match tabel_vorto(vorto) {
        Some(valuo) => return valuo,
        None => (),
    }

    match pronomo(vorto) {
        Some(valuo) => return valuo,
        None => (),
    }

    if vorto.len() < 3 {
        return json!({});
    }

    let (akuzativa, plurala, speco, rez) = if vorto.ends_with("o") || vorto.ends_with("'") {
        (false, false, VortSpeco::Substantivo, radiko(trancxi!(vorto, 1)))
    } else if vorto.ends_with("oj") {
        (false, true, VortSpeco::Substantivo, radiko(trancxi!(vorto, 2)))
    } else if vorto.ends_with("on") {
        (true, false, VortSpeco::Substantivo, radiko(trancxi!(vorto, 2)))
    } else if vorto.ends_with("ojn") {
        (true, true, VortSpeco::Substantivo, radiko(trancxi!(vorto, 3)))
    } else if vorto.ends_with("a") {
        (false, false, VortSpeco::Adjektivo, radiko(trancxi!(vorto, 1)))
    } else if vorto.ends_with("aj") {
        (false, true, VortSpeco::Adjektivo, radiko(trancxi!(vorto, 2)))
    } else if vorto.ends_with("an") {
        (true, false, VortSpeco::Adjektivo, radiko(trancxi!(vorto, 2)))
    } else if vorto.ends_with("ajn") {
        (true, true, VortSpeco::Adjektivo, radiko(trancxi!(vorto, 3)))
    } else if vorto.ends_with("en") {
        (false, true, VortSpeco::Adverbo, radiko(trancxi!(vorto, 2)))
    } else if vorto.ends_with("e") {
        (false, false, VortSpeco::Adverbo, radiko(trancxi!(vorto, 1)))
    } else if vorto.ends_with("s") {
        (false, false, VortSpeco::Verbo, verbo(vorto))
    } else {
        return json!({});
    };

    /*
    return match speco {
        VortSpeco::Adjektivo => adjektivo(vorto),
        VortSpeco::Substantivo => substantivo(vorto),
        VortSpeco::Verbo => verbo(vorto),
        VortSpeco::Adverbo => adverbo(vorto),
        VortSpeco::GramatikaVorto => unreachable!(),
    };
    */
    rez
}

fn gramatika(vorto: &str) -> Option<Value> {
    match &KONSTANTAJ[vorto] {
        Value::String(traduko) => Some(json!({vorto: traduko})),
        _ => None,
    }
}

fn tabel_vorto(vorto: &str) -> Option<Value> {
    let (akuzativa, plurala, fino) = if vorto.ends_with("jn") {
        (true, true, 2)
    } else if vorto.ends_with("j") {
        (false, true, 1)
    } else if vorto.ends_with("n") {
        (true, false, 1)
    } else {
        (false, false, 0)
    };

    let vorto = trancxi!(vorto, fino);

    match &TABEL_VORTOJ[vorto] {
        Value::String(traduko) => Some(json!({vorto: traduko})),
        _ => None,
    }
}

fn pronomo(vorto: &str) -> Option<Value> {
    let (poseda, akuzativa, plurala, fino) = if vorto.ends_with("ajn") {
        (true, true, true, 3)
    } else if vorto.ends_with("an") {
        (true, true, false, 2)
    } else if vorto.ends_with("a") {
        (true, false, false, 1)
    } else if vorto.ends_with("n") {
        (false, true, false, 1)
    } else {
        (false, false, false, 0)
    };

    let vorto = trancxi!(vorto, fino);

    match &PRONOMOJ[vorto] {
        Value::String(traduko) => Some(json!({ vorto: traduko })),
        _ => None,
    }
}

fn radiko(vorto: &str) -> Value {
    json!({
        vorto: "z"
    })
}

fn verbo(vorto: &str) -> Value {
    json!({
        vorto: "verbo"
    })
}
