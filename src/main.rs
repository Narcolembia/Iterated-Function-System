
#![allow(unused, non_snake_case)]
#![allow(non_upper_case_globals)]


mod util;
mod ifs;
use std::{collections::HashMap, f32::consts::PI, vec};
use ifs::{scalar_function_to_ifs, vector_function_to_ifs, Compose, Ifs, Vec3};
use anyhow::Result as AResult;
use image::{RgbaImage, Rgba};
use util::{gen_dilations, gen_points_on_circle, gen_sinxy_functions,i};
use num::Complex;




fn main() -> AResult<()> {
    let j = 1;
 
    //for j in (0..101){
    
        let rng = rand::thread_rng();
        let v = vector_function_to_ifs(|x| x);
        let comp_sin = scalar_function_to_ifs(|x| x.sin());
        let comp_cos = scalar_function_to_ifs(|x| x.cos());
        let mult_components = vector_function_to_ifs(|v:Complex<f32>| Complex::new(v.re*v.im,v.re*v.im));
        let sin_xy = comp_sin.compose(&mult_components);
        let cos_xy = comp_cos.compose(&mult_components);
        let sin = vector_function_to_ifs(|x| x.sin());
        let asin = vector_function_to_ifs(|x| x.asin());
        let cos = vector_function_to_ifs(|x| x.cos());
        let ln = vector_function_to_ifs(|x| x.ln());
        let tan =  vector_function_to_ifs(|x| x.tan());
        let inv = vector_function_to_ifs(|x| 1.0/x);


        let contractions_1 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(4,1.0,0.0), 0.5));
       
        
       
        let ifs_final:Ifs = tan.compose(&contractions_1);
        let weights = Some([4.0,1.0,4.0,1.0].to_vec());


    
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(0.5,0.5,0.5), Vec3::new(0.5,0.5,0.5), Vec3::new(0.25,0.25,0.25), Vec3::new(0.0,0.33,0.67));
        let colors = (0..ifs_final.len).map(|x|palette((x as f32/ifs_final.len as f32))).collect();
       
        let img = util::render_ifs(ifs_final,weights,10u32.pow(7),colors,(2000,2000),1.0,(0.0,0.0),6.0,1,10);

    
        img.save(format!("out_{}.png",j))?;
    //}

    Ok(())
}


