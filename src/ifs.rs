


use dyn_clone::{DynClone,clone_trait_object};
use nalgebra::{Dyn, SVector};
use rand::{prelude::Distribution, Rng};
use core::num;
use std::{f32::consts::PI, ops, process::Output};

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


macro_rules! impl_ifsfunc_op {

    ($op_name:ty,$op_func:ident, $func_name:ident) => {
        impl $op_name for Box<dyn IfsFunction>{

            type Output =  Box<dyn IfsFunction>;

            fn $op_func(self, rhs: Self) -> Box<dyn IfsFunction> {
                
                return Box::new(
                    move |x| -> Vec2{
                        return self(x).$func_name(&rhs(x));
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

fn compose(lhs: Box<dyn IfsFunction>,rhs: Box<dyn IfsFunction>) ->Box<dyn IfsFunction>{
    return Box::new(move|x| lhs(rhs(x)));
}



#[derive(Clone)]
pub struct Ifs {
    functions: Vec<Box<dyn IfsFunction>>,
    weights: Vec<f32>,
    len: usize,
}

pub trait Compose<T,O>{
 
    fn compose(self,other:T) -> O;
}

impl Ifs {
    pub fn new(functions: Vec<Box<dyn IfsFunction>>, weights:Option<Vec<f32>>) -> Self {
        let weights = match weights{
            None => (0..functions.len()).map(|x|1.0).collect(),

            Some(x) =>x.clone(),

        };
        let functions = functions.clone();
        let len = functions.len();
        Self { functions, weights, len }
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
        return Ifs::new(new_funcs, Some(new_weights));
    }


}
pub fn scalar_function_to_ifs(func: impl Fn(f32)->f32 + Clone + 'static)->Ifs{
    let func = func.clone();
    let func = move|v:Vec2| Vec2::new(func(v[0]),func(v[1]));    
    let func:Box<dyn IfsFunction> = Box::new(func);
    let func = [func].to_vec();
    return Ifs::new(func,None);
}
pub fn vector_function_to_ifs(func: impl Fn(Vec2)->Vec2 + Clone + 'static)->Ifs{
    let func = func.clone();
    let func = move|v:Vec2| func(v);    
    let func:Box<dyn IfsFunction> = Box::new(func);
    let func = [func].to_vec();
    return Ifs::new(func,None);
}


fn match_sizes_and_clone(left:&Ifs,  right:&Ifs) -> (Ifs,Ifs){

    let mut left_clone = left.clone();
    let mut right_clone = right.clone();
    if left_clone.len == right_clone.len {return (left_clone,right_clone)};

    let mut short: &mut Ifs = if left_clone.len > right_clone.len { &mut right_clone } else {&mut left_clone};

    let diff = (left.len as i32 - right.len as i32).abs() as usize;
    let quotient = diff/short.len;
    let remainder = diff % short.len;
    for _ in 0..quotient{
        short.functions.extend_from_within((0..short.len));
        short.weights.extend_from_within((0..short.len));
    }
    short.functions.extend_from_within((0..remainder));
    short.weights.extend_from_within((0..remainder));

    return (left_clone,right_clone)
}




macro_rules! impl_ifs_op {
    ($op_name:ty, $rhs_type:ty, $op_func:ident, $func_name:ident) => {
        
    impl $op_name for &Ifs{
        type Output = Ifs;

        fn $op_func(self, rhs: $rhs_type) -> Ifs {
            let (lhs,rhs) = match_sizes_and_clone(&self, &rhs);
            let new_funcs= lhs.functions.into_iter().zip(rhs.functions).map(move|(l,r)| l.$func_name(r)).collect();
            let new_weights = lhs.weights.into_iter().zip(rhs.weights).map(move|(l,r)| l).collect();
            return Ifs::new(new_funcs ,Some(new_weights));
        }
    }
    impl $op_name for Ifs{
        type Output = Ifs;
        fn $op_func(self, rhs: $rhs_type) -> Ifs {
            let (lhs,rhs) = match_sizes_and_clone(&self, &rhs);
            let new_funcs= lhs.functions.into_iter().zip(rhs.functions).map(move|(l,r)| l.$func_name(r)).collect();
            let new_weights = lhs.weights.into_iter().zip(rhs.weights).map(move|(l,r)| l).collect();
            return Ifs::new(new_funcs ,Some(new_weights));
        }
    }


    };
}
impl_ifs_op!(ops::Add<&Ifs>,&Ifs,add,add);
impl_ifs_op!(ops::Sub<&Ifs>,&Ifs,sub,sub);
impl_ifs_op!(ops::Mul<&Ifs>,&Ifs,mul,mul);
impl_ifs_op!(ops::Div<&Ifs>,&Ifs,div,div);
impl_ifs_op!(ops::Add<Ifs>,Ifs,add,add);
impl_ifs_op!(ops::Sub<Ifs>,Ifs,sub,sub);
impl_ifs_op!(ops::Mul<Ifs>,Ifs,mul,mul);
impl_ifs_op!(ops::Div<Ifs>,Ifs,div,div);

macro_rules! impl_ifs_scalar_op {
    ($op_name:ty, $op_func:ident, $op:tt) => {
        impl $op_name for &Ifs{
            type Output = Ifs;
    
            fn $op_func(self, rhs: f32) -> Ifs {
                let self_copy = self.clone();
                let rhs:Box<dyn IfsFunction> = Box::new(move|x:Vec2| x $op rhs);
                let rhs = [rhs].to_vec();
                let rhs = Ifs::new(rhs,None);
                return self_copy $op rhs
            }
            
        }
        impl $op_name for Ifs{
            type Output = Ifs;
    
            fn $op_func(self, rhs: f32) -> Ifs {
                
                let rhs:Box<dyn IfsFunction> = Box::new(move|x:Vec2| x $op rhs);
                let rhs = [rhs].to_vec();
                let rhs = Ifs::new(rhs,None);
                return self $op rhs
            }
            
        }
    };
}

impl_ifs_scalar_op!(ops::Mul<f32>, mul,*);
impl_ifs_scalar_op!(ops::Div<f32>, div,/);
impl Compose<&Ifs,Ifs> for &Ifs{
    fn compose(self, other:&Ifs) ->Ifs{
        let (self_clone,other_clone) = match_sizes_and_clone(self, other);
        let new_funcs= self_clone.functions.into_iter().zip(other_clone.functions).map(move|(l,r)| compose(l,r)).collect();
        let new_weights = self_clone.weights.into_iter().zip(other_clone.weights).map(move|(l,r)| l).collect();
        return Ifs::new(new_funcs,Some(new_weights))
    }
}

impl Compose<Ifs,Ifs> for &Ifs{
    fn compose(self, other:Ifs) ->Ifs{
        let (self_clone,other_clone) = match_sizes_and_clone(&self, &other);
        let new_funcs= self_clone.functions.into_iter().zip(other_clone.functions).map(move|(l,r)| compose(l,r)).collect();
        let new_weights = self_clone.weights.into_iter().zip(other_clone.weights).map(move|(l,r)| l).collect();
        return Ifs::new(new_funcs,Some(new_weights))
    }
}


