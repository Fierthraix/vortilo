extern crate pest;

use std::collections::HashMap;

use vortilo::kreu_propraĵoj;

use crate::pest::Parser;
use crate::pest::Span;
use vortilo::{Rule, Vortilo};

fn main() {
    // Legu dosieron.
    let dosiero = "Mi estas homo kiuj simple ŝatas la plej bonajn aĵojn en la viv'".to_ascii_lowercase();

    let frazo: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // Elektu lingvon.

    for vorto in dosiero.split_whitespace() {
        let parsita_vorto = Vortilo::parse(Rule::vorto, vorto);

        println!("{:?}", parsita_vorto);
    }

    kreu_propraĵoj();

}
