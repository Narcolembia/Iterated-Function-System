use dyn_clone::{clone_box, clone_trait_object, DynClone,clone};
use ::num::Complex;
use std::{f32::consts::PI, ops, process::Output};
use crate::util::{i};
use auto_ops::impl_op_ex;
use lazy_static::{lazy_static};



pub trait ComplexFunction: (Fn(Complex<f32>) -> Complex<f32>) + DynClone + Send { 

    
}

impl<Func: Clone + Fn(Complex<f32>) -> Complex<f32> + ?Sized + Send> ComplexFunction for Func {

}

clone_trait_object!(ComplexFunction);

pub trait Compose<Other> {
    fn compose(&self, other:Other) -> Self;
    
}

impl Compose<&Box<dyn ComplexFunction>> for Box<dyn ComplexFunction>{
    fn compose(&self, other:&Box<dyn ComplexFunction>) -> Self{
        let self_clone = clone_box(self);
        let other_clone = clone_box(other);
        return Box::new(move |z| self_clone(other_clone(z)));
    }
}
impl Compose<Box<dyn ComplexFunction>> for Box<dyn ComplexFunction>{
    fn compose(&self, other:Box<dyn ComplexFunction>) -> Self{
        let self_clone = clone_box(self);
        return Box::new(move |z| self_clone(other(z)));
    }
}

pub trait Pow<Other> {
    fn pow(&self, other:Other) -> Self;
    
}

impl Pow<&Box<dyn ComplexFunction>> for Box<dyn ComplexFunction>{
    fn pow(&self, other:&Box<dyn ComplexFunction>) -> Self{
        let self_clone = clone_box(self);
        let other_clone = clone_box(other);
        
        return Box::new(move |z| self_clone(z).powc(other_clone(z)));
    }
}
impl Pow<Box<dyn ComplexFunction>> for Box<dyn ComplexFunction>{
    fn pow(&self, other:Box<dyn ComplexFunction>) -> Self{
        let self_clone = clone_box(self);

        return Box::new(move |z| self_clone(z).powc(other(z)));
    }
}
impl Pow<Complex<f32>> for Box<dyn ComplexFunction>{
    fn pow(&self, other:Complex<f32>) -> Self{
        let self_clone = clone_box(self);
        return Box::new(move |z| self_clone(z).powc(other));
    }
}
impl Pow<f32> for Box<dyn ComplexFunction>{
    fn pow(&self, other:f32) -> Self{
        let self_clone = clone_box(self);
        return Box::new(move |z| self_clone(z).powc(other + 0.0*i));
    }
}


macro_rules!  ComplexFunction_ops{
    ($op:tt) => {
        impl_op_ex!($op |lhs:&Box<dyn ComplexFunction>, rhs:&Box<dyn ComplexFunction>| -> Box<dyn ComplexFunction>{ 
            let lhs_clone = clone_box(lhs);
            let rhs_clone =  clone_box(rhs);
            Box::new(move |z| lhs_clone(z) $op rhs_clone(z))}
        );
        impl_op_ex!($op |lhs:&Box<dyn ComplexFunction>, rhs:&Complex<f32>| -> Box<dyn ComplexFunction>{ 
            let lhs_clone = clone_box(lhs);
            let rhs_clone = clone(rhs);
            Box::new(move |z| lhs_clone(z) $op rhs_clone)}
        );
        impl_op_ex!($op |lhs:&Complex<f32>, rhs:&Box<dyn ComplexFunction>| -> Box<dyn ComplexFunction>{ 
            let rhs_clone = clone_box(rhs);
            let lhs_clone = clone(lhs);
            Box::new(move |z| lhs_clone $op rhs_clone(z))}
        );
        impl_op_ex!($op |lhs:&Box<dyn ComplexFunction>, rhs:f32| -> Box<dyn ComplexFunction>{ 
            let lhs_clone = clone_box(lhs);
           
            Box::new(move |z| lhs_clone(z) $op rhs)}
        );
        impl_op_ex!($op |lhs:f32, rhs:&Box<dyn ComplexFunction>| -> Box<dyn ComplexFunction>{ 
            let rhs_clone = clone_box(rhs);
          
            Box::new(move |z| lhs $op rhs_clone(z))}
        );

    };
}

ComplexFunction_ops!(+);
ComplexFunction_ops!(-);
ComplexFunction_ops!(*);
ComplexFunction_ops!(/);

fn compose(lhs: Box<dyn ComplexFunction>,rhs: Box<dyn ComplexFunction>) ->Box<dyn ComplexFunction>{
    return Box::new(move|x| lhs(rhs(x)));
}
