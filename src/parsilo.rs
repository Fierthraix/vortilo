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

macro_rules! nur_traduko_el_mapo {
    ($vorto:expr, $mapo:expr) => {
        if $mapo.contains_key($vorto) {
            match &$mapo[$vorto] {
                Value::String(traduko) => Some(traduko),
                _ => unreachable!(),
            }
        } else {
            None
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

fn vektoro_al_mapo(vektoro: Vec<(String, Value)>) -> Vec<Value> {
    let mut vek = Vec::with_capacity(vektoro.len());
    for (indekso, value) in vektoro.iter() {
        let mut mapo = Map::with_capacity(1);
        mapo.insert(indekso.clone(), value.clone());
        vek.push(Value::Object(mapo))
    }
    vek
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
    static ref AKUZATIVA: Value = serde_json::from_str(r#"{ "n": "accusative" }"#).unwrap();
    static ref PLURALA: Value = serde_json::from_str(r#"{ "j": "plural" }"#).unwrap();
    static ref ADJEKTIVA: Value = serde_json::from_str(r#"{ "a": "adjective" }"#).unwrap();
    static ref ADVERBA: Value = serde_json::from_str(r#"{ "e": "adverb" }"#).unwrap();
    static ref SUBSTANTIVA: Value = serde_json::from_str(r#"{ "o": "noun" }"#).unwrap();
    static ref POEM_SUBSTANTIVA: Value =
        serde_json::from_str(r#"{ "'": "poetry noun ending" }"#).unwrap();
}

pub fn parsu_frazon(frazo: &str) -> Value {
    let mut rezultoj = vec![];
    for vortaĵo in frazo.split_whitespace() {
        let vorto = vortaĵo.trim_end_matches(&[',', ';', '.', '-'][..]);
        rezultoj.push(parsu_vorton(vorto));
    }

    Value::Array(rezultoj)
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

    // Kontrolu ĉu vorto verbas.
    if vorto.ends_with('s') || vorto.ends_with('i') {
        return verbo(vorto);
    }

    // Pritrakti 'e', 'a', kaj 'o' vortojn.
    let (akuzativa, plurala, speco, radik) = if vorto.ends_with('o') {
        (false, false, SUBSTANTIVA.clone(), trancxi!(vorto, 1))
    } else if vorto.ends_with('\'') {
        (false, false, POEM_SUBSTANTIVA.clone(), trancxi!(vorto, 1))
    } else if vorto.ends_with("oj") {
        (false, true, SUBSTANTIVA.clone(), trancxi!(vorto, 2))
    } else if vorto.ends_with("on") {
        (true, false, SUBSTANTIVA.clone(), trancxi!(vorto, 2))
    } else if vorto.ends_with("ojn") {
        (true, true, SUBSTANTIVA.clone(), trancxi!(vorto, 3))
    } else if vorto.ends_with('a') {
        (false, false, ADJEKTIVA.clone(), trancxi!(vorto, 1))
    } else if vorto.ends_with("aj") {
        (false, true, ADJEKTIVA.clone(), trancxi!(vorto, 2))
    } else if vorto.ends_with("an") {
        (true, false, ADJEKTIVA.clone(), trancxi!(vorto, 2))
    } else if vorto.ends_with("ajn") {
        (true, true, ADJEKTIVA.clone(), trancxi!(vorto, 3))
    } else if vorto.ends_with("en") {
        (true, false, ADVERBA.clone(), trancxi!(vorto, 2))
    } else if vorto.ends_with('e') {
        (false, false, ADVERBA.clone(), trancxi!(vorto, 1))
    } else {
        return json!({});
    };

    //let mut rezulto = radiko, speco];
    let mut rezulto = radiko(radik);
    rezulto.push(speco);

    if plurala {
        rezulto.push(PLURALA.clone());
    }
    if akuzativa {
        rezulto.push(AKUZATIVA.clone());
    }

    Value::Array(rezulto)
}

fn gramatika(vorto: &str) -> Option<Value> {
    if let Some(gramatika_vorto) = traduko_el_mapo!(vorto, KONSTANTAJ) {
        Some(Value::Array(vec![gramatika_vorto]))
    } else {
        None
    }
}

fn tabel_vorto(vorto: &str) -> Option<Value> {
    let (akuzativa, plurala, fino) = if vorto.ends_with("jn") {
        (true, true, 2)
    } else if vorto.ends_with('j') {
        (false, true, 1)
    } else if vorto.ends_with('n') {
        (true, false, 1)
    } else {
        (false, false, 0)
    };

    let vorto = trancxi!(vorto, fino);

    let traduko = nur_traduko_el_mapo!(vorto, TABEL_VORTOJ)?;
    let mut rezulto = vec![json!({ vorto: traduko })];

    if plurala {
        rezulto.push(PLURALA.clone());
    }
    if akuzativa {
        rezulto.push(AKUZATIVA.clone());
    }

    Some(Value::Array(rezulto))
}

fn pronomo(vorto: &str) -> Option<Value> {
    let (poseda, akuzativa, plurala, fino) = if vorto.ends_with("ajn") {
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

    let mut rezulto = vec![];
    let traduko = nur_traduko_el_mapo!(vorto, PRONOMOJ)?;
    rezulto.push(json!({ vorto: traduko }));

    if poseda {
        rezulto.push(serde_json::from_str(r#"{"a": "possesive"}"#).unwrap());
    }
    if plurala {
        rezulto.push(PLURALA.clone());
    }
    if akuzativa {
        rezulto.push(AKUZATIVA.clone());
    }

    Some(Value::Array(rezulto))
}

fn radiko(vorto: &str) -> Vec<Value> {
    match kunmetita(vorto) {
        Some(vektoro) => vektoro_al_mapo(vektoro),
        None => vec![],
    }
}

fn kunmetita(vorto: &str) -> Option<Vec<(String, Value)>> {
    let mut indeksoj = vec![];
    let mut valuoj = vec![];

    let mut nuna_indekso = 0;
    let mut vorto_indesko = 0;

    while nuna_indekso < vorto.len() {
        let ebla_vorto = &RADIKOJ[vorto_indesko].0; // Radiko ni volas provi.
        if vorto[nuna_indekso..].starts_with(ebla_vorto) {
            // Vorto komencas kun la radiko; aldonu ĝin al nia list'.
            indeksoj.push((nuna_indekso, vorto_indesko));
            valuoj.push(RADIKOJ[vorto_indesko].clone());

            nuna_indekso += ebla_vorto.len(); // Movu al nova indekso.
            vorto_indesko = 0; // Ni volas reprovi ĉiun radikon.
        } else {
            // Alie, vorto ne ĝustis, do ni pliigu tiun indekson.
            if vorto_indesko < RADIKOJ.len() - 1 {
                vorto_indesko += 1; // Se ne, pliigu l'indekson.
            } else {
                if valuoj.is_empty() {
                    return None;
                }
                valuoj.pop(); // Forĵeti valuon.
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
    let (tenso, radik) = if vorto.ends_with('i') {
        (
            serde_json::from_str(r#"{"i": "infinitive tense"}"#).unwrap(),
            trancxi!(vorto, 1),
        )
    } else if vorto.ends_with('u') {
        (
            serde_json::from_str(r#"{"u": "imperative tense"}"#).unwrap(),
            trancxi!(vorto, 2),
        )
    } else if vorto.ends_with("us") {
        (
            serde_json::from_str(r#"{"us": "conditional tense"}"#).unwrap(),
            trancxi!(vorto, 2),
        )
    } else if vorto.ends_with("is") {
        (
            serde_json::from_str(r#"{"is": "past tense"}"#).unwrap(),
            trancxi!(vorto, 2),
        )
    } else if vorto.ends_with("as") {
        (
            serde_json::from_str(r#"{"as": "present tense"}"#).unwrap(),
            trancxi!(vorto, 2),
        )
    } else if vorto.ends_with("os") {
        (
            serde_json::from_str(r#"{"os": "future tense"}"#).unwrap(),
            trancxi!(vorto, 2),
        )
    } else {
        return json!({});
    };

    let mut rezulto = radiko(radik);
    rezulto.push(tenso);

    Value::Array(rezulto)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testu_vortilon() {
        let frazo = "mi estas simpla homo kiu ŝatas la plej bonajn aĵojn en la viv' morgaŭ";

        let vortoj = frazo
            .split_whitespace()
            .map(|vorto| parsu_vorton(vorto))
            .collect::<Vec<Value>>();

        println!("{}", Value::Array(vortoj.clone()).to_string());

        let atendita: Value = serde_json::from_str(r#"[
            [{"mi":{"mi":"I/me"}}],
            [{"estas":[{"est":"you are"}]},{"as":"present tense"}],
            [[{"simpl":"simple"}],{"a":"adjective"}],
            [[{"hom":"man"}],{"o":"noun"}],
            [{"kiu":{"kiu":"who/which"}}],
            [{"ŝatas":[{"ŝat":"to like"}]},{"as":"present tense"}],
            [{"la":"the"}],
            [{"plej":"most"}],
            [[{"bon":"good"}],{"a":"adjective"},{"j":"plural"},{"n":"accusative"}],
            [[{"aĵ":"thing, concrete manifestation"}],{"o":"noun"},{"j":"plural"},{"n":"accusative"}],
            [{"en":"in"}],
            [{"la":"the"}],
            [[{"viv":"live"}],{"'":"noun"}],
            [{"morgaŭ":"tomorrow"}]
        ]"#).unwrap();

        assert_eq!(Value::Array(vortoj), atendita);
    }
}
