#[macro_use]
extern crate lazy_static;

use serde_json::{json, Value};

lazy_static! {
    static ref AFIKSOJ: Value = serde_json::from_str(include_str!("../vortoj/afiksoj.json")).unwrap();
    static ref ADJEKTIVOJ: Value = serde_json::from_str(include_str!("../vortoj/adjektivoj.json")).unwrap();
    static ref KONSTANTAJ: Value = serde_json::from_str(include_str!("../vortoj/konstantaj.json")).unwrap();
    static ref PRONOMOJ: Value = serde_json::from_str(include_str!("../vortoj/pronomoj.json")).unwrap();
    static ref SUBSTANTIVOJ: Value = serde_json::from_str(include_str!("../vortoj/substantivoj.json")).unwrap();
    static ref TABEL_VORTOJ: Value =
        serde_json::from_str(include_str!("../vortoj/tabelvortoj.json")).unwrap();
    static ref VERBOJ: Value = serde_json::from_str(include_str!("../vortoj/verboj.json")).unwrap();
}

macro_rules! trancxi {
    ($vorto:expr, $nombro:expr) => {
        &$vorto[..$vorto.as_bytes().iter().count() - $nombro]
    };
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

    // Trovu tabelvortojn.
    match tabel_vorto(vorto) {
        Some(valuo) => return valuo,
        None => (),
    }

    // Trovu pronomojn.
    match pronomo(vorto) {
        Some(valuo) => return valuo,
        None => (),
    }

    // Sekvonta kontrolo bezonas almenaŭ 3 literojn.
    if vorto.len() < 3 {
        return json!({});
    }

    let (akuzativa, plurala, speco, rez) = if vorto.ends_with("o") || vorto.ends_with("'") {
        (
            false,
            false,
            VortSpeco::Substantivo,
            radiko(trancxi!(vorto, 1)),
        )
    } else if vorto.ends_with("oj") {
        (
            false,
            true,
            VortSpeco::Substantivo,
            radiko(trancxi!(vorto, 2)),
        )
    } else if vorto.ends_with("on") {
        (
            true,
            false,
            VortSpeco::Substantivo,
            radiko(trancxi!(vorto, 2)),
        )
    } else if vorto.ends_with("ojn") {
        (
            true,
            true,
            VortSpeco::Substantivo,
            radiko(trancxi!(vorto, 3)),
        )
    } else if vorto.ends_with("a") {
        (
            false,
            false,
            VortSpeco::Adjektivo,
            radiko(trancxi!(vorto, 1)),
        )
    } else if vorto.ends_with("aj") {
        (
            false,
            true,
            VortSpeco::Adjektivo,
            radiko(trancxi!(vorto, 2)),
        )
    } else if vorto.ends_with("an") {
        (
            true,
            false,
            VortSpeco::Adjektivo,
            radiko(trancxi!(vorto, 2)),
        )
    } else if vorto.ends_with("ajn") {
        (true, true, VortSpeco::Adjektivo, radiko(trancxi!(vorto, 3)))
    } else if vorto.ends_with("en") {
        (false, true, VortSpeco::Adverbo, radiko(trancxi!(vorto, 2)))
    } else if vorto.ends_with("e") {
        (false, false, VortSpeco::Adverbo, radiko(trancxi!(vorto, 1)))
    } else if vorto.ends_with("s") || vorto.ends_with("i") {
        (false, false, VortSpeco::Verbo, verbo(vorto))
    } else {
        return json!({});
    };

    if plurala {
        //rez.aldonu(pluralo)
    }
    if akuzativa {
        //rez.aldonu(akuzativo)
    }

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
        Value::String(traduko) => Some(json!({ vorto: traduko })),
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
        Value::String(traduko) => Some(json!({ vorto: traduko })),
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
    simpla_radiko(vorto)
}

fn simpla_radiko(radiko: &str) -> Value {
    // Kontrolu ĉu vorto estas en normala listo.
    match &ADJEKTIVOJ[radiko] {
        Value::String(traduko) => return json!({ radiko: traduko }),
        _ => (),
    }
    match &SUBSTANTIVOJ[radiko] {
        Value::String(traduko) => return json!({ radiko: traduko }),
        _ => (),
    }
    match &VERBOJ[radiko] {
        Value::String(traduko) => return json!({ radiko: traduko }),
        _ => (),
    }

    json!({ radiko: "NE TROVITA" })
}

fn verbo(vorto: &str) -> Value {
    let (rezulto, radik) = if vorto.ends_with("i") {
        ("A", trancxi!(vorto, 1))
    } else if vorto.ends_with("u") {
        ("A", trancxi!(vorto, 2))
    } else if vorto.ends_with("us") {
        ("A", trancxi!(vorto, 2))
    } else if vorto.ends_with("is") {
        ("A", trancxi!(vorto, 2))
    } else if vorto.ends_with("as") {
        ("A", trancxi!(vorto, 2))
    } else if vorto.ends_with("os") {
        ("A", trancxi!(vorto, 2))
    } else {
        return json!({});
    };
    json!({
        vorto: radiko(radik)
    })
}
