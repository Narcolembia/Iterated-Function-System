
#![allow(unused, non_snake_case)]
#![allow(non_upper_case_globals)]


mod util;
mod ifs;
mod complexFunction;

use crate::complexFunction::ComplexFunction;
use std::{collections::HashMap, f32::consts::PI, vec};
use complexFunction::{Compose, Pow};
use ifs::{Ifs, Vec3};
use anyhow::Result as AResult;
use image::{buffer::ConvertBuffer, imageops, RgbImage, Rgba, RgbaImage,GrayAlphaImage};
use util::{dilate_around_point, gen_points_on_circle, i};
use num::{complex::ComplexFloat, Complex};




fn main() -> AResult<()> {
    
 
    //for j in (0..101){
    
        let rng = rand::thread_rng();

        let re_static:Box<dyn ComplexFunction> = Box::new(|x|x.re().into());
        let re = &re_static;

        let im_static:Box<dyn ComplexFunction> = Box::new(|x|x.im().into());
        let im = &im_static;
        
        let sin_static:Box<dyn ComplexFunction> = Box::new(|x|x.sin());
        let sin = &sin_static;

        let cos_static:Box<dyn ComplexFunction> = Box::new(|x|x.cos());
        let cos = &re_static;

        let tan_static:Box<dyn ComplexFunction> = Box::new(|x|x.cos());
        let tan = &tan_static;

        let ln_static:Box<dyn ComplexFunction> = Box::new(|x|x.ln());
        let ln = &ln_static;

        let z_static:Box<dyn ComplexFunction> = Box::new(|z|z);
        let z = &z_static;
        
        let num_iters = 10u32.pow(9);
        let resolution = (2000,2000);
        //let resolution = (resolution.1,resolution.0);
        let scale = 0.08;
        let offset = (0.0,0.0);
        let rotation = PI/2.0;
        let gamma_correction = 5.0;
        let supersample = 1;
        let num_threads = 10;
        let transparent_background = false;

        let mut functions:Vec<Box<dyn ComplexFunction>> = Vec::new();
        let points = gen_points_on_circle(6, 1.5, 0.0);
        for j in 0..6{
            functions.push({ 
                let point = points[j].clone();
                tan.compose(ln.compose(re.compose( ((0.09*z.pow(3.0) + 0.95*z + 0.5 * point)/(0.09*z.pow(5.0) + 0.95*z - 0.5 * point)).compose(sin) ) + i* im.compose(((0.09*z.pow(3.0) + 0.95*z + 0.5 * point)/(0.09*z.pow(5.0) + 0.95*z - 0.5 * point)).compose(cos))))
            } ) ;
        }
     

        let weights = None;
        let ifs = Ifs::new(functions);

        
        
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette_1 = util::generate_color_pallete(Vec3::new(0.5,0.5,0.5), Vec3::new(0.8,0.8,0.8), Vec3::new(0.0,0.0,0.2), Vec3::new(0.5,0.0,0.33));
        //let colors = (0..ifs.len).map(|x|Vec3::new(0.5,0.2,0.0)).collect();

     
        let background_color = Some([0,0,0,255].to_vec());
        //let background_color = None;
        
       
   
        let mut img_1 = util::render_ifs_greyscale(&ifs,weights,false,num_iters,resolution,scale,offset,rotation,gamma_correction,supersample,num_threads);
   
        //let img_1:RgbImage = img_1.convert();
        
        //let img_1:RgbImage = img_1.convert();
        img_1.save(format!("out_{}.png",1))?;
    //}

    Ok(())
}


