mod utils;
extern crate rustoku;
use wasm_bindgen::prelude::*;
use std::error::Error;
use std::convert::TryInto;
use rustoku::solve::move_change::{ChangeType, IndexValuePair};
use std::fmt::{Display, Formatter};

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Sudoku {
    puz: rustoku::Sudoku,
    solver: rustoku::HumanSolve,
}

#[wasm_bindgen]
pub struct Square(rustoku::Square);

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChangeT {
    SetValue = 1,
    RemovedPot = 0,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ChangeS (Vec<u32>);

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
impl ChangeS {

    pub fn new() -> ChangeS{
        ChangeS(Vec::new())
    }

    pub fn insert(&mut self, index: u8, set: bool, value:u8){
        let mut v: u32 = index as u32;
        v <<= 8;
        if set {
            v += 1;
        }
        v <<= 8;
        v += value as u32;
        self.0.push(v);
    }

    pub fn involved_index(&self, move_index: u32) -> u8 {
        let mut v = self.0.get(move_index as usize).expect(&format!("Value of out range: {}", move_index)).clone();
        v >>= 16;
        v as u8
    }

    pub fn did_val_change (&self, move_index: u32) -> bool {
        let mut v = self.0.get(move_index as usize).expect(&format!("Value of out range: {}", move_index)).clone();
        v >>= 8;
        v & 1 > 0
    }

    pub fn affected_value(&self, move_index: u32) -> u8 {
        let v = self.0.get(move_index as usize).expect(&format!("Value of out range: {}", move_index)).clone();
        let res = v & 255;
        assert!(res > 0 && res <= 9);
        res as u8

    }

    pub fn as_string (&self) -> String {
        let str = format!("{}", self);
        println!("{}",str);
        str
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
    // pub fn involved(&self) -> Vec<u8>{
    //     self.0.get_involved_vec().iter().map(|v| *v as u8).collect()
    // }
    //
    // pub fn value_change(&self) -> i32 {
    //     for change in self.0.changes_iter() {
    //         match change.change() {
    //             ChangeType::SetValue(v) => return change.index() as i32,
    //             _ => ()
    //         }
    //     }
    //     -1i32
    // }
    //
    // pub fn value(&self) -> u8 {
    //     match self.0.changes_iter().next().unwrap().change() {
    //         ChangeType::SetValue(v) => *v,
    //         ChangeType::RemovedPot(v) => *v
    //     }
    // }
    //
    // pub fn removed_pot_ind (&self) -> Vec<u8> {
    //     self.0.changes_vec().iter().map(|sq| sq.index() as u8).collect()
    // }
    //
    // pub fn technique(&self) -> String {
    //     self.0.method().to_string()
    // }
}


impl Display for ChangeS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.0.len() {
            let ind = self.affected_value(i as u32);
            let change = self.did_val_change(i as u32);
            let value = self.affected_value(i as u32);
            write!(f, "Did it change: {}\nIndex: {}\nValue: {}", change, ind, value)?;
        }
        Ok(())
    }
}

