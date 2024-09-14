use dyn_clone::{DynClone,clone_trait_object};
use nalgebra::{Dyn, SVector};
use rand::{prelude::Distribution, Rng};
use core::num;
use std::{f32::consts::PI, ops};

pub type Vec2 = SVector<f32, 2>;
pub type Vec3 = SVector<f32, 3>;

#[test]
pub fn ree() {
    println!("hello");

    panic!("sandbox function");
}

pub trait IfsFunction: (Fn(Vec2) -> Vec2) + DynClone { 
    
}

impl<Func: Clone + Fn(Vec2) -> Vec2 + ?Sized> IfsFunction for Func {

}
clone_trait_object!(IfsFunction);

pub trait IfsScalarFunction: (Fn(Vec2) -> f32) + DynClone {
    
}

impl<Func: Clone + Fn(Vec2) -> f32 + ?Sized> IfsScalarFunction for Func {

}
clone_trait_object!(IfsScalarFunction);


macro_rules! impl_ifsfunc_op {

    ($op_name:ty,$op_func:ident, $func_name:ident) => {
        impl $op_name for &Box<dyn IfsFunction>{

            type Output =  Box<dyn IfsFunction>;

            fn $op_func(self, rhs: Self) -> Box<dyn IfsFunction> {
                let self_clone = dyn_clone::clone_box(self);
                let rhs_clone = dyn_clone::clone_box(rhs);
                return Box::new(
                    move |x| -> Vec2{
                        return self_clone(x).$func_name(&rhs_clone(x));
                    }
                )
            }
        }
    };

}

impl_ifsfunc_op!(ops::Add,add,add);
impl_ifsfunc_op!(ops::Mul,mul,component_mul);
impl_ifsfunc_op!(ops::Sub,sub,sub);
impl_ifsfunc_op!(ops::Div,div,component_div);





pub struct Ifs {
    functions: Vec<Box<dyn IfsFunction>>,
    weights: Vec<f32>,
}


impl Ifs {
    pub fn new(functions: &Vec<Box<dyn IfsFunction>>, weights:Option<&Vec<f32>>) -> Self {
        let weights = match weights{
            None => (0..functions.len()).map(|x|1.0).collect(),

            Some(x) =>x.clone(),

        };
        let functions = functions.clone();
        Self { functions, weights }
    }

    pub fn build_function(&self, mut rng: impl Rng) -> impl FnMut(Vec2) -> (Vec2,f32) {
        let functions: Vec<_> = self
            .functions.clone();
        let dist = rand::distributions::WeightedIndex::new(self.weights.clone()).unwrap();

        return move |x| {
            let index = dist.sample(&mut rng);
            return (functions[index](x),(index as f32)/((functions.len() - 1 )as f32));
        }
    }

    pub fn extend(&self, other:&Ifs) -> Ifs{
        let mut new_weights = self.weights.clone();
        new_weights.extend(other.weights.clone());
        let mut new_funcs: Vec<Box<dyn IfsFunction>> = self
            .functions.clone();
        new_funcs.extend(other.functions.clone());
        return Ifs{functions: new_funcs ,weights: new_weights};
    }


}



macro_rules! impl_ifs_op {
    ($op_name:ty, $op_func:ident, $func_name:ident) => {
        
    impl $op_name for &Ifs{
        type Output = Ifs;

        fn $op_func(self, rhs: Self) -> Ifs {
            let new_funcs= self.functions.clone().into_iter().zip(rhs.functions.clone()).map(move|(l,r)| l.$func_name(&r)).collect();
            let new_weights = self.weights.clone().into_iter().zip(rhs.weights.clone()).map(move|(l,r)| l).collect();
            return Ifs{functions: new_funcs ,weights: new_weights};
        }
    }
        
    };
}
impl_ifs_op!(ops::Add,add,add);
impl_ifs_op!(ops::Sub,sub,sub);
impl_ifs_op!(ops::Mul,mul,mul);
impl_ifs_op!(ops::Div,div,div);
