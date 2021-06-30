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
            _ => unreachable!(),
        }
    };
}

macro_rules! traduko_el_mapo {
    ($vorto:expr, $mapo:expr) => {
        if $mapo.contains_key($vorto) {
            match &$mapo[$vorto] {
                Value::String(traduko) => Some(json!({ $vorto: traduko })),
                _ => unreachable!(),
            }
        } else {
            None
        }
    };
}

fn vektoro_al_mapo(vektoro: Vec<(String, Value)>) -> Value {
    let mut vek = Vec::with_capacity(vektoro.len());
    for (indekso, value) in vektoro.iter() {
        let mut mapo = Map::with_capacity(1);
        mapo.insert(indekso.clone(), value.clone());
        vek.push(Value::Object(mapo))
    }
    Value::Array(vek)
}

lazy_static! {
    static ref AFIKSOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/afiksoj.json");
    static ref ADJEKTIVOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/adjektivoj.json");
    static ref KONSTANTAJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/konstantaj.json");
    static ref PRONOMOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/pronomoj.json");
    static ref SUBSTANTIVOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/substantivoj.json");
    static ref TABEL_VORTOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/tabelvortoj.json");
    static ref VERBOJ: Map<String, Value> = alsxutu_dosieron!("../vortoj/verboj.json");
    static ref RADIKOJ: Vec<(String, Value)> = AFIKSOJ
        .iter()
        .chain(ADJEKTIVOJ.iter())
        .chain(KONSTANTAJ.iter())
        .chain(PRONOMOJ.iter())
        .chain(SUBSTANTIVOJ.iter())
        .chain(TABEL_VORTOJ.iter())
        .chain(VERBOJ.iter())
        .map(|(radiko, traduko)| (radiko.clone(), traduko.clone()))
        .collect();
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
    Adjektivo,
    Substantivo,
    Adverbo,
    Verbo,
}

pub fn parsu_vorton(vorto: &str) -> Value {
    // Kontrolu ĉu la vorto estas gramatika.
    if let Some(valuo) = gramatika(vorto) {
        return valuo;
    }

    // Trovu tabelvortojn.
    if let Some(valuo) = tabel_vorto(vorto) {
        return valuo;
    }

    // Trovu pronomojn.
    if let Some(valuo) = pronomo(vorto) {
        return valuo;
    }

    // Sekvonta kontrolo bezonas almenaŭ 3 literojn.
    if vorto.len() < 3 {
        return json!({});
    }

    let (akuzativa, plurala, _speco, rez) = if vorto.ends_with('o') || vorto.ends_with('\'') {
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
    } else if vorto.ends_with('a') {
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
    } else if vorto.ends_with('e') {
        (false, false, VortSpeco::Adverbo, radiko(trancxi!(vorto, 1)))
    } else if vorto.ends_with('s') || vorto.ends_with('i') {
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
    let (_akuzativa, _plurala, fino) = if vorto.ends_with("jn") {
        (true, true, 2)
    } else if vorto.ends_with('j') {
        (false, true, 1)
    } else if vorto.ends_with('n') {
        (true, false, 1)
    } else {
        (false, false, 0)
    };

    let vorto = trancxi!(vorto, fino);

    traduko_el_mapo!(vorto, TABEL_VORTOJ)
}

fn pronomo(vorto: &str) -> Option<Value> {
    let (_poseda, _akuzativa, _plurala, fino) = if vorto.ends_with("ajn") {
        (true, true, true, 3)
    } else if vorto.ends_with("an") {
        (true, true, false, 2)
    } else if vorto.ends_with('a') {
        (true, false, false, 1)
    } else if vorto.ends_with('n') {
        (false, true, false, 1)
    } else {
        (false, false, false, 0)
    };

    let vorto = trancxi!(vorto, fino);

    traduko_el_mapo!(vorto, PRONOMOJ)
}

fn radiko(vorto: &str) -> Value {
    match rikuro(vorto) {
        Some(vektoro) => vektoro_al_mapo(vektoro),
        None => json!({}),
    }
}

fn rikuro(vorto: &str) -> Option<Vec<(String, Value)>> {
    let mut indeksoj = vec![];
    let mut valuoj = vec![];

    let mut nuna_indekso = 0;
    let mut vorto_indesko = 0;

    while nuna_indekso < vorto.len() {
        let ebla_vorto = &RADIKOJ[vorto_indesko].0; // Radiko ni volas provi.
        if vorto[nuna_indekso..].starts_with(ebla_vorto) {
            // Sukceso -- vorto komencas kun radiko.
            indeksoj.push((nuna_indekso, vorto_indesko)); // Aldonu l'informon al nia listo.
            valuoj.push(RADIKOJ[vorto_indesko].clone()); // Ankaux konservi tradukon.

            nuna_indekso += ebla_vorto.len();
            vorto_indesko = 0;
        } else {
            // Alie, gxi ne gxustis, do ni pliigu l'indekson.

            if vorto_indesko < RADIKOJ.len() - 1 {
                // Kontrolu cxu ni penis cxiun vorton.
                vorto_indesko += 1; // Se ne, pliigu l'indekson.
            } else {
                // Ne penis cxiun vorton, do forjxeti lastan valuon, kaj reprovi.
                if valuoj.is_empty() {
                    return None;
                }
                valuoj.pop(); // Forjxeti valuon.
                let nov_indekstoj = indeksoj.pop().unwrap(); // Restarigi al lastaj indeksoj.
                nuna_indekso = nov_indekstoj.0;
                vorto_indesko = nov_indekstoj.1 + 1;

                if vorto_indesko >= RADIKOJ.len() {
                    // Ni ne havas pliajn vortojn provi.
                    return None;
                }
            }
        }
    }
    Some(valuoj)
}

fn verbo(vorto: &str) -> Value {
    let (_rezulto, radik) = if vorto.ends_with('i') {
        ("infinitive tense", trancxi!(vorto, 1))
    } else if vorto.ends_with('u') {
        ("imperative tense", trancxi!(vorto, 2))
    } else if vorto.ends_with("us") {
        ("conditional tense", trancxi!(vorto, 2))
    } else if vorto.ends_with("is") {
        ("past tense", trancxi!(vorto, 2))
    } else if vorto.ends_with("as") {
        ("present tense", trancxi!(vorto, 2))
    } else if vorto.ends_with("os") {
        ("future tense", trancxi!(vorto, 2))
    } else {
        return json!({});
    };
    json!({ vorto: radiko(radik) })
}
