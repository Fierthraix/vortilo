extern crate pest;

use vortilo::kreu_propraĵoj;

use crate::pest::Parser;
use vortilo::{Rule, Vortilo};

fn main() {
    // Legu dosieron.
    //let dosiero = "Mi estas homo kiuj simple ŝatas la plej bonajn aĵojn en la viv'".to_ascii_lowercase();

    //let frazo: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // Elektu lingvon.

    let dosiero = "kaj"; //" en preter ne minus plus jes";
    for vorto in dosiero.split_whitespace() {
        let parsita_vorto = Vortilo::parse(Rule::vorto, vorto).unwrap().next().unwrap();

        //println!("{:?}", parsita_vorto);
        println!("{:?}", kreu_propraĵoj(parsita_vorto));
    }

}
