#[macro_use]
extern crate lazy_static;

use serde_json::{json, map::Map, Value};

macro_rules! trancxi {
    ($vorto:expr, $nombro:expr) => {
        &$vorto[..$vorto.as_bytes().iter().count() - $nombro]
    };
}

macro_rules! alsxutu_dosieron {
    ($dosiero:expr) => {
        match serde_json::from_str(include_str!($dosiero)).unwrap() {
            Value::Object(mapo) => mapo,
            _ => unreachable!()
        }
    }
}

macro_rules! traduko_el_mapo {
    ($vorto:expr, $mapo:expr)  => {
        if $mapo.contains_key($vorto) {
            match &$mapo[$vorto] {
                Value::String(traduko) => Some(json!({ $vorto: traduko })),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}


lazy_static! {
    static ref AFIKSOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/afiksoj.json");
    static ref ADJEKTIVOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/adjektivoj.json");
    static ref KONSTANTAJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/konstantaj.json");
    static ref PRONOMOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/pronomoj.json");
    static ref SUBSTANTIVOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/substantivoj.json");
    static ref TABEL_VORTOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/tabelvortoj.json");
    static ref VERBOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/verboj.json");
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
    traduko_el_mapo!(vorto, KONSTANTAJ)
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

    traduko_el_mapo!(vorto, TABEL_VORTOJ)
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

    traduko_el_mapo!(vorto, PRONOMOJ)
}

fn radiko(vorto: &str) -> Value {
    simpla_radiko(vorto)
}

fn subradikoj(vorto: &str) -> Option<Value> {

    //for vorto in ADJEKTIVOJ + SUBSTANTIVOJ + ... {
    //    rikure alvoki la progragon por atingi ilin.
    //
    //}
    None
}

fn simpla_radiko(radiko: &str) -> Value {
    // Kontrolu ĉu vorto estas en normala listo.
    match traduko_el_mapo!(radiko, ADJEKTIVOJ) {
        Some(json) => return json,
        _ => ()
    }
    match traduko_el_mapo!(radiko, ADJEKTIVOJ) {
        Some(json) => return json,
        _ => ()
    }
    match traduko_el_mapo!(radiko, SUBSTANTIVOJ) {
        Some(json) => return json,
        _ => ()
    }
    match traduko_el_mapo!(radiko, VERBOJ) {
        Some(json) => return json,
        _ => ()
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
