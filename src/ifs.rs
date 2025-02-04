
use crate::complexFunction::ComplexFunction;
use dyn_clone::{DynClone,clone_trait_object};
use nalgebra::SVector;
use rand::{prelude::Distribution, Rng};
use core::num;
use std::{f32::consts::PI, ops, process::Output};
use ::num::Complex;

pub type Vec3 = SVector<f32, 3>;



#[derive(Clone)]
pub struct Ifs {
    functions: Vec<Box<dyn ComplexFunction>>,
    pub len: usize,
}


impl Ifs {
    pub fn new(functions: Vec<Box<dyn ComplexFunction>>) -> Self {

        let functions = functions.clone();
        let len = functions.len();
        Self { functions, len }
    }

    pub fn build_function(&self,weights:Option<Vec<f32>>, mut rng: impl Rng) -> impl FnMut(Complex<f32>) -> (Complex<f32>,usize) {
        let functions: Vec<_> = self
            .functions.clone();
        let weights = match weights{
            None => (0..functions.len()).map(|x|1.0).collect(),

            Some(x) =>x,

        };

        let dist = rand::distributions::WeightedIndex::new(weights).unwrap();

        return move |x| {
            let index = dist.sample(&mut rng);
            return (functions[index](x),index);
        }
    }


    pub fn extend(&self, other:&Ifs) -> Ifs{
       
        let mut new_funcs: Vec<Box<dyn ComplexFunction>> = self
            .functions.clone();
        new_funcs.extend(other.functions.clone());
        return Ifs::new(new_funcs);
    }


}
