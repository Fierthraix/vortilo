use logos::Logos;
use vortilo::Vorto;

fn main() {
    // Legu dosieron.

    // Elektu lingvon.

    let dosiero = "katojn hundaj trairantan preta";
    let asdf = "kun morgaŭ kaj preter sub katajn irantan preta faritaj";

    let dosiero = "mi estas homo kiuj simple ŝatas la plej bonajn aĵojn en la viv'";
    let mut lex = Vorto::lexer(dosiero);

    for fdsa in lex { 
        println!("{:?}", fdsa);
    }
}
