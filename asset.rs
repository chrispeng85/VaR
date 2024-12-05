use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]

pub struct Asset {
    name: String,
    returns: Vec<f64>,
    current_value: f64,


}   

impl Asset {

    pub fn new(name: String, current_value: f64) ->Self {
        Asset {
            name,
            returns: Vec::new(),
            current_value,
        }

    }

    pub fn add_return(&mut self, ret: f64) {
        self.returns.push(ret);

    }

    pub fn name(&self)->str {

        &self.name

    }

    pub fn return(&self) -> &[f64] {

        &self.returns
    }

    pub fn current_value(&self) -> f64 {
        self.current_value

    }
    






}