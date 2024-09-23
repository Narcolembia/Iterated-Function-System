
#![allow(unused, non_snake_case)]


mod util;
mod ifs;
use std::{collections::HashMap, f32::consts::PI};
use ifs::{scalar_function_to_ifs, vector_function_to_ifs, Compose, Ifs, Vec3};
use anyhow::Result as AResult;
use image::{RgbaImage, Rgba};
use util::{gen_dilations, gen_points_on_circle, gen_sinxy_functions};
use num::Complex;




fn main() -> AResult<()> {
    let j = 1;
    let i = Complex::<f32>::new(0.0,1.0);
    //for j in (0..101){
    
        let rng = rand::thread_rng();
        let v = vector_function_to_ifs(|x| x);
        let comp_sin = scalar_function_to_ifs(|x| x.sin());
        let comp_cos = scalar_function_to_ifs(|x| x.cos());
        let mult_components = vector_function_to_ifs(|v:Complex<f32>| Complex::new(v.re*v.im,v.re*v.im));
        let sin_xy = comp_sin.compose(&mult_components);
        let cos_xy = comp_cos.compose(&mult_components);
        let sin = vector_function_to_ifs(|x| x.sin());
        let cos = vector_function_to_ifs(|x| x.cos());
        let ln = vector_function_to_ifs(|x| x.ln());
        let tan =  vector_function_to_ifs(|x| x.tan());
        let inv = vector_function_to_ifs(|x| 1.0/x);


        let contractions_1 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(5,1.0), 0.5),None);
        let contractions_2 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(10,2.0), 0.2),None);


        
        let ifs_final:Ifs = tan.compose(&contractions_1);
        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(0.7,0.7,0.7), Vec3::new(0.9,0.9,0.9), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(7)), palette,2000,1.0,Complex::<f32>::new(0.0,0.0),5.0,);

    
        img.save(format!("out_{}.png",j))?;
    ///}

    Ok(())
}


