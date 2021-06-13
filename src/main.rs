use logos::Logos;
use vortilo::parsu_vorton;

fn main() {
    // Legu dosieron.

    // Elektu lingvon.

    let dosiero = "katojn hundaj trairantan preta";
    let asdf = "kun morgaŭ kaj preter sub katajn irantan preta faritaj";

    let dosiero = "mi estas homo kiuj simple ŝatas la plej bonajn aĵojn en la viv'";

    for vorto in dosiero.split_whitespace() {
        println!("{} => {:?}", vorto, parsu_vorton(vorto));
    }
}
