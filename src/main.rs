use vortilo::parsu_vorton;

fn main() {
    let fdsa = "katojn hundaj trairantan malsanulejo preta";
    let asdf = "kun morgaŭ kaj preter sub katajn irantan preta faritaj";

    let dosiero = "mi estas homo kiuj simple ŝatas la plej bonajn aĵojn en la miajn viv'";

    for vorto in asdf.split_whitespace() {
        println!("{} => {}", vorto, parsu_vorton(vorto).to_string());
    }
    for vorto in fdsa.split_whitespace() {
        println!("{} => {}", vorto, parsu_vorton(vorto).to_string());
    }
    for vorto in dosiero.split_whitespace() {
        println!("{} => {}", vorto, parsu_vorton(vorto).to_string());
    }
    let vorto = "iruntan";
    println!("{} => {}", vorto, parsu_vorton(vorto).to_string());
}
