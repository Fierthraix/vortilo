use serde_json::{Value, json, map::Map};

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
                Value::String(traduko) => Some(traduko),
                _ => unreachable!(),
            }
        } else {
            None
        }
    };
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
        let vorto = vortaĵo.trim_end_matches(&[',', ';', '.', '-', '?', '!'][..]);
        rezultoj.push(parsu_vorton(&vorto.to_lowercase()));
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
        return Value::Array(vec![json!({vorto: ""})]);
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
        return Value::Array(vec![json!({vorto: ""})]);
    };

    let mut interpretoj = radiko(radik);
    if interpretoj.is_empty() {
        interpretoj.push(vec![json!({radik: ""})]);
    }

    for interpreto in interpretoj.iter_mut() {
        interpreto.push(speco.clone());
        if plurala {
            interpreto.push(PLURALA.clone());
        }
        if akuzativa {
            interpreto.push(AKUZATIVA.clone());
        }
    }

    if interpretoj.len() == 1 {
        return Value::Array(interpretoj.pop().unwrap());
    }

    Value::Array(interpretoj.into_iter().map(Value::Array).collect())
}

fn gramatika(vorto: &str) -> Option<Value> {
    let traduko = traduko_el_mapo!(vorto, KONSTANTAJ)?;
    Some(Value::Array(vec![json!({ vorto: traduko })]))
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

    let traduko = traduko_el_mapo!(vorto, TABEL_VORTOJ)?;
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
    let traduko = traduko_el_mapo!(vorto, PRONOMOJ)?;
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

fn radiko(vorto: &str) -> Vec<Vec<Value>> {
    kunmetita(vorto)
        .into_iter()
        .map(|vektoro| {
            vektoro
                .into_iter()
                .map(|(indekso, valuo)| {
                    let mut mapo = Map::with_capacity(1);
                    mapo.insert(indekso, valuo);
                    Value::Object(mapo)
                })
                .collect::<Vec<Value>>()
        })
        .collect::<Vec<Vec<Value>>>()
}

fn kunmetita(vorto: &str) -> Vec<Vec<(String, Value)>> {
    let mut interpretoj = vec![];
    let mut nuna_vojo = vec![];
    sercxu_kunmetojn(vorto, 0, &mut nuna_vojo, &mut interpretoj);
    interpretoj.sort_by_key(|i| i.len());
    interpretoj
}

fn sercxu_kunmetojn(
    vorto: &str,
    nuna_indekso: usize,
    nuna_vojo: &mut Vec<(String, Value)>,
    interpretoj: &mut Vec<Vec<(String, Value)>>,
) {
    if nuna_indekso == vorto.len() {
        interpretoj.push(nuna_vojo.clone());
        return;
    }

    for (radiko, traduko) in RADIKOJ.iter() {
        if vorto[nuna_indekso..].starts_with(radiko) {
            nuna_vojo.push((radiko.clone(), traduko.clone()));
            sercxu_kunmetojn(vorto, nuna_indekso + radiko.len(), nuna_vojo, interpretoj);
            nuna_vojo.pop();
        }
    }
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

    let mut interpretoj = radiko(radik);

    if interpretoj.is_empty() {
        return Value::Array(vec![tenso]);
    }

    for interpreto in interpretoj.iter_mut() {
        interpreto.push(tenso.clone());
    }

    if interpretoj.len() == 1 {
        return Value::Array(interpretoj.pop().unwrap());
    }

    Value::Array(interpretoj.into_iter().map(Value::Array).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testu_bazan_frazon() {
        let frazo = "mi estas simpla homo kiu ŝatas la plej bonajn aĵojn en la viv' morgaŭ";

        let vortoj = frazo
            .split_whitespace()
            .map(|vorto| parsu_vorton(vorto))
            .collect::<Vec<Value>>();

        let atendita = serde_json::json!([
            [{"mi":"I/me"}],
            [{"est":"to be"},{"as":"present tense"}],
            [{"simpl":"simple"},{"a":"adjective"}],
            [{"hom":"man"},{"o":"noun"}],
            [{"kiu":"who/which"}],
            [{"ŝat":"to like"},{"as":"present tense"}],
            [{"la":"the"}],
            [{"plej":"most"}],
            [{"bon":"good"},{"a":"adjective"},{"j":"plural"},{"n":"accusative"}],
            [{"aĵ":"thing, concrete manifestation"},{"o":"noun"},{"j":"plural"},{"n":"accusative"}],
            [{"en":"in"}],
            [{"la":"the"}],
            [{"viv":"live"},{"'":"poetry noun ending"}],
            [{"morgaŭ":"tomorrow"}]
        ]);

        assert_eq!(Value::Array(vortoj), atendita);
    }

    #[test]
    fn testu_plurajn_interpretojn_por_kunmetajxo() {
        let rezulto = parsu_vorton("dolĉamaro");
        let atendita = serde_json::json!([
            [{"dolĉ":"sweet"},{"amar":"bitter"},{"o":"noun"}],
            [{"dolĉ":"sweet"},{"am":"love"},{"ar":"group, collection"},{"o":"noun"}]
        ]);
        assert_eq!(rezulto, atendita);
    }

    #[test]
    fn testu_ordigon_de_interpretoj_laŭ_komponoj() {
        let rezulto = parsu_vorton("eraro");
        // "erar-o" (2 komponoj) devas aperi antaŭ "er-ar-o" (3 komponoj)
        let atendita = serde_json::json!([
            [{"erar":"to err"},{"o":"noun"}],
            [{"er":"fragment, small piece, particle"},{"ar":"group, collection"},{"o":"noun"}]
        ]);
        assert_eq!(rezulto, atendita);
    }

    // --- Verbaj tensoj ---

    #[test]
    fn testu_pasintajn_verbojn() {
        assert_eq!(
            parsu_vorton("kuris"),
            serde_json::json!([{"kur":"run"},{"is":"past tense"}])
        );
    }

    #[test]
    fn testu_estontajn_verbojn() {
        assert_eq!(
            parsu_vorton("kuros"),
            serde_json::json!([{"kur":"run"},{"os":"future tense"}])
        );
    }

    #[test]
    fn testu_kondiĉajn_verbojn() {
        assert_eq!(
            parsu_vorton("kurus"),
            serde_json::json!([{"kur":"run"},{"us":"conditional tense"}])
        );
    }

    #[test]
    fn testu_infinitivajn_verbojn() {
        assert_eq!(
            parsu_vorton("kuri"),
            serde_json::json!([{"kur":"run"},{"i":"infinitive tense"}])
        );
    }

    // --- Adverboj ---

    #[test]
    fn testu_adverbon() {
        assert_eq!(
            parsu_vorton("rapide"),
            serde_json::json!([{"rapid":"fast"},{"e":"adverb"}])
        );
    }

    #[test]
    fn testu_adverbon_akuzativan() {
        assert_eq!(
            parsu_vorton("rapiden"),
            serde_json::json!([{"rapid":"fast"},{"e":"adverb"},{"n":"accusative"}])
        );
    }

    // --- Substantivaj finaĵoj ---

    #[test]
    fn testu_substantivon_pluralon() {
        assert_eq!(
            parsu_vorton("hundoj"),
            serde_json::json!([{"hund":"dog"},{"o":"noun"},{"j":"plural"}])
        );
    }

    #[test]
    fn testu_substantivon_akuzativan() {
        assert_eq!(
            parsu_vorton("hundon"),
            serde_json::json!([{"hund":"dog"},{"o":"noun"},{"n":"accusative"}])
        );
    }

    #[test]
    fn testu_substantivon_pluralan_akuzativan() {
        assert_eq!(
            parsu_vorton("hundojn"),
            serde_json::json!([{"hund":"dog"},{"o":"noun"},{"j":"plural"},{"n":"accusative"}])
        );
    }

    // --- Adjektivaj finaĵoj ---

    #[test]
    fn testu_adjektivon_pluralon() {
        assert_eq!(
            parsu_vorton("rapidaj"),
            serde_json::json!([{"rapid":"fast"},{"a":"adjective"},{"j":"plural"}])
        );
    }

    #[test]
    fn testu_adjektivon_pluralan_akuzativan() {
        assert_eq!(
            parsu_vorton("rapidajn"),
            serde_json::json!([{"rapid":"fast"},{"a":"adjective"},{"j":"plural"},{"n":"accusative"}])
        );
    }

    // --- Tabelvorto kun finaĵoj ---

    #[test]
    fn testu_tabelvortojn_bazajn() {
        assert_eq!(
            parsu_vorton("tio"),
            serde_json::json!([{"tio":"that"}])
        );
    }

    #[test]
    fn testu_tabelvortojn_pluralon() {
        assert_eq!(
            parsu_vorton("tioj"),
            serde_json::json!([{"tio":"that"},{"j":"plural"}])
        );
    }

    #[test]
    fn testu_tabelvortojn_akuzativan() {
        assert_eq!(
            parsu_vorton("tion"),
            serde_json::json!([{"tio":"that"},{"n":"accusative"}])
        );
    }

    #[test]
    fn testu_tabelvortojn_pluralan_akuzativan() {
        assert_eq!(
            parsu_vorton("tiojn"),
            serde_json::json!([{"tio":"that"},{"j":"plural"},{"n":"accusative"}])
        );
    }

    // --- Pronomaj finaĵoj ---

    #[test]
    fn testu_pronomajn_akuzativan() {
        assert_eq!(
            parsu_vorton("min"),
            serde_json::json!([{"mi":"I/me"},{"n":"accusative"}])
        );
    }

    #[test]
    fn testu_pronomajn_posedan() {
        assert_eq!(
            parsu_vorton("mia"),
            serde_json::json!([{"mi":"I/me"},{"a":"possesive"}])
        );
    }

    #[test]
    fn testu_pronomajn_posedan_akuzativan() {
        assert_eq!(
            parsu_vorton("mian"),
            serde_json::json!([{"mi":"I/me"},{"a":"possesive"},{"n":"accusative"}])
        );
    }

    #[test]
    fn testu_pronomajn_posedan_pluralon_akuzativan() {
        assert_eq!(
            parsu_vorton("miajn"),
            serde_json::json!([{"mi":"I/me"},{"a":"possesive"},{"j":"plural"},{"n":"accusative"}])
        );
    }

    // --- Nekonataj vortoj ---

    #[test]
    fn testu_nekonatan_vorton() {
        // vorto sen rekonigebla finaĵo
        assert_eq!(
            parsu_vorton("xyz"),
            serde_json::json!([{"xyz":""}])
        );
    }

    #[test]
    fn testu_mallongan_vorton() {
        // malpli ol 3 bajtoj
        assert_eq!(
            parsu_vorton("ab"),
            serde_json::json!([{"ab":""}])
        );
    }

    // --- Interpunkcio en parsu_frazon ---

    #[test]
    fn testu_interpunkcion_en_frazo() {
        let rezulto = parsu_frazon("vi kuras!");
        let atendita = serde_json::json!([
            [{"vi":"you"}],
            [{"kur":"run"},{"as":"present tense"}]
        ]);
        assert_eq!(rezulto, atendita);
    }

    #[test]
    fn testu_komon_en_frazo() {
        let rezulto = parsu_frazon("jes, mi");
        let atendita = serde_json::json!([
            [{"jes":"yes"}],
            [{"mi":"I/me"}]
        ]);
        assert_eq!(rezulto, atendita);
    }
}
