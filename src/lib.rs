mod utils;
extern crate rustoku;
use wasm_bindgen::prelude::*;
use std::error::Error;
use std::convert::TryInto;
use rustoku::solve::move_change::{ChangeType, IndexValuePair};
use std::fmt::{Display, Formatter};

extern crate js_sys;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/*
    The Rustoku library does not implement wasm_bindgen itself, so we create wrappers for the needed
    data structures
 */

#[wasm_bindgen]
pub struct Sudoku {
    puz: rustoku::Sudoku,
    solver: rustoku::HumanSolve,
}

#[wasm_bindgen]
pub struct Square(rustoku::Square);


#[wasm_bindgen]
pub struct Move(rustoku::Move);



#[wasm_bindgen]
impl Sudoku {
    pub fn new(input_str: &str) -> Sudoku {
        let puz = rustoku::Sudoku::new(input_str).unwrap();
        let solver = rustoku::HumanSolve::new();
        Sudoku {
            puz,
            solver
        }
    }

    pub fn values(&self) -> Vec<u8> {
        self.puz.iter().map(|sq| sq.num()).collect()
    }

    pub fn square(&self, ind: usize) -> Square {
        Square(self.puz[ind])
    }

    pub fn value(&self, ind: u32) -> u8 {
        self.puz[ind as usize].num()
    }

    pub fn poss(&self, ind: u32) -> Vec<u8>{
        self.puz[ind as usize].possibilities()
    }

    pub fn board_size() -> u32 {
        rustoku::Sudoku::board_size() as u32
    }
    
    pub fn hint(&self) -> Move {
        Move(self.solver.next(&self.puz).unwrap())
    }

    pub fn is_fixed(&self, ind: u32) -> bool {
        self.puz[ind as usize].is_fixed()
    }
}

#[wasm_bindgen]
impl Square {
    pub fn value(&self) -> u8 {
        self.0.num()
    }

    pub fn possibilities (&self) -> Vec<u8> {
        self.0.possibilities()
    }
}


#[wasm_bindgen]
impl Move {

    pub fn update_board(&self) {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");


        for pair in self.0.involved_vec() {
            Self::update_candidates(&document, pair, "candidate--highlight")
        }

        for pair in self.0.changes_vec() {
            // Does not matter right now if the change type removes potentials or sets a value
            let v = match pair {
                ChangeType::RemovedPot(v) => v,
                ChangeType::SetValue(v) => v,
            };
            Self::update_candidates(&document, v, "candidate--to-remove");
        }

        let message = document.get_element_by_id("message-p").unwrap();
        message.set_inner_html(&format!("{}", self.0.method()));

    }

    // This function interacts with the JS window document directly, making it easier to deal
    // with the more complex data type over the wasm boundary.
    fn update_candidates(document: &web_sys::Document, pair: &IndexValuePair, class_to_add: &str) {
        let ind = pair.index();
        let values = pair.value_vec();
        let mut sq = document.get_element_by_id(&format!("sq{}", ind)).unwrap();
        let mut sqp = document.get_element_by_id(&format!("sq{}-cand", ind)).unwrap();
        web_sys::console::log_1(&JsValue::from_str(&format!("{:?}", values)));
        for value in values {
            sqp.children().get_with_index(value as u32 - 1).unwrap().class_list().add_1(class_to_add);
        }
    }

    pub fn apply(&self, puz: &mut Sudoku) {
        let amove = self.0.clone();
        amove.apply(&mut puz.puz);

    }

}

