
#![allow(unused, non_snake_case)]
#![allow(non_upper_case_globals)]


mod util;
mod ifs;
use std::{collections::HashMap, f32::consts::PI, vec};
use ifs::{scalar_function_to_ifs, vector_function_to_ifs, Compose, Ifs, Vec3};
use anyhow::Result as AResult;
use image::{buffer::ConvertBuffer, imageops, RgbImage, Rgba, RgbaImage,GrayAlphaImage};
use nalgebra::ComplexField;
use util::{gen_dilations, gen_points_on_circle,i};
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
        let atan =  vector_function_to_ifs(|x| x.atan());
        let inv = vector_function_to_ifs(|x| 1.0/x);
        let real = vector_function_to_ifs(|x| (x.re + 0.0*i));

        let num_iters = 10u32.pow(9);
        let resolution = (4000,4000);
        //let resolution = (resolution.1,resolution.0);
        let scale = 0.43;
        let offset = (0.0,0.0);
        let rotation = 0.0;
        let gamma_correction = 4.0;
        let supersample = 2;
        let num_threads = 10;
        let transparent_background = false;

        let points_1 = gen_points_on_circle(4, 1.0, 0.0);
        let contractions_1 = Ifs::new(
        gen_dilations(
                &points_1, 0.5));
 
   

        let contractions = contractions_1;
        let ifs_1 = tan.compose(&contractions);
     
        let ifs_final = ifs_1;
        let weights_1:Option<Vec<f32>> = Some([6.0,1.0,6.0,1.0].to_vec());
        
        //let weights_1 = None;
        
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette_1 = util::generate_color_pallete(Vec3::new(0.5,0.5,0.5), Vec3::new(0.8,0.8,0.8), Vec3::new(0.0,0.0,0.2), Vec3::new(0.5,0.0,0.33));
        //let colors_1 = (0..ifs_final.len).map(|x|Vec3::new(0.0,0.0,0.0)).collect();

     
        //let background_color = Some([255,255,255,255].to_vec());
        //let background_color = None;
        
       
        //let mut img_1 = util::render_ifs(&ifs_final,weights_1,&colors_1,Some(&background_color),10u32.pow(7),(2000,2000),2.0,(-0.0,0.3),-PI/2.0,1.5,1,10);
       
        ///let mut img_1 = util::render_ifs(&ifs_final,weights_1,&colors_1,background_color.as_ref(),num_iters,resolution,scale,offset,rotation,gamma_correction,supersample,num_threads);
        let mut img_1 = util::render_ifs_greyscale(&ifs_final,weights_1,transparent_background,num_iters,resolution,scale,offset,rotation,gamma_correction,supersample,num_threads);
        //let img_1:RgbImage = img_1.convert();
        
        //let img_1:RgbImage = img_1.convert();
        img_1.save(format!("out_{}.png",j))?;
    //}

    Ok(())
}


