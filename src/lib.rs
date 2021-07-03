#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod parsilo;
mod retmodelo;

pub use parsilo::parsu_frazon;
pub use parsilo::parsu_vorton;
use retmodelo::RetPaĝo;

#[wasm_bindgen(start)]
pub fn eku() {
    App::<RetPaĝo>::new().mount_to_body();
}
