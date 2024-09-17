
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
    for j in (0..101){
    
        let rng = rand::thread_rng();
        let v = vector_function_to_ifs(|x| x);
        let comp_sin = scalar_function_to_ifs(|x| x.sin());
        let comp_cos = scalar_function_to_ifs(|x| x.cos());
        let mult_components = vector_function_to_ifs(|v:Complex<f32>| Complex::new(v.re*v.im,v.re*v.im));
        let sin_xy = comp_sin.compose(&mult_components);
        let cos_xy = comp_cos.compose(&mult_components);
        let sin = vector_function_to_ifs(|x| x.sin());
        let cos = vector_function_to_ifs(|x| x.cos());
        


        let contractions_1 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(3,0.25), 0.15),None);
        let contractions_2 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(6,0.5), 0.05),None);
        let contractions_3 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(20,7.0), 0.1),None);

        
        let ifs_final:Ifs = (&contractions_1 + &sin.compose(&contractions_1 * 3.0)).extend(&(&contractions_2 + &sin.compose(&contractions_2*2.0))).compose(&v*Complex::cis(2.0*PI*j as f32/100.0));
        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(0.7,0.7,0.7), Vec3::new(0.9,0.9,0.9), Vec3::new(0.1,0.2,0.7), Vec3::new(0.0,0.33,0.2));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(7)), palette,2000,6.0);

    
        img.save(format!("out_{}.png",j))?;
    }

    Ok(())
}


