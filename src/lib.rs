#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use web_sys::window;

mod parsilo;
mod retmodelo;

pub use parsilo::parsu_frazon;
pub use parsilo::parsu_vorton;
use retmodelo::RetPaĝo;

#[wasm_bindgen(start)]
pub fn eku() {
    let document = window()
        .and_then(|window| window.document())
        .expect("ne povis akiri document");
    let root = document
        .get_element_by_id("vortilo-root")
        .expect("ne povis trovi #vortilo-root");
    yew::Renderer::<RetPaĝo>::with_root(root).render();
}
