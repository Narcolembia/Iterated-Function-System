
#![allow(unused, non_snake_case)]


mod util;
mod ifs;

use ifs::{scalar_function_to_ifs, vector_function_to_ifs, Compose, Ifs, Vec2, Vec3};
use anyhow::Result as AResult;
use image::{RgbaImage, Rgba};
use util::{gen_dilations, gen_points_on_circle, gen_sinxy_functions};



fn main() -> AResult<()> {
    let i = 1;
    
    //for i in (0..100){
        let rng = rand::thread_rng();
        let v = vector_function_to_ifs(|x| x);
        let comp_sin = scalar_function_to_ifs(|x| x.sin());
        let comp_cos = scalar_function_to_ifs(|x| x.cos());
        let mult_components = vector_function_to_ifs(|v:Vec2| Vec2::new(v[0]*v[1],v[0]*v[1]));
        let sin_xy = comp_sin.compose(&mult_components);
        let cos_xy = comp_cos.compose(&mult_components);
        


        let contractions_1 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(10,1.0), 0.4),None);
        let contractions_2 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(10,5.0), 0.4),None);
        let contractions_3 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(20,7.0), 0.25),None);
        
        
        
        
       
        let ifs_final = (contractions_1.compose(&sin_xy.compose(&v*0.1) + &contractions_2) + &contractions_1.compose(&cos_xy.compose(&v*0.2) + &contractions_2)).extend(&(&contractions_3 * &sin_xy.compose(&v*0.1) + &contractions_3));

        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(0.8,0.8,0.8), Vec3::new(0.7,0.7,0.7), Vec3::new(0.1,0.4,0.4), Vec3::new(0.0,0.33,0.2));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(8)), palette,2000,8.0,1.8);

    
        img.save(format!("out_{}.png",i))?;
    //}

    Ok(())
}


#![allow(unused, non_snake_case)]


mod util;
mod ifs;

use ifs::{scalar_function_to_ifs, vector_function_to_ifs, Compose, Ifs, Vec2, Vec3};
use anyhow::Result as AResult;
use image::{RgbaImage, Rgba};
use util::{gen_dilations, gen_points_on_circle, gen_sinxy_functions};



fn main() -> AResult<()> {
    let i = 1;
    
    //for i in (0..100){
        let rng = rand::thread_rng();
        let v = vector_function_to_ifs(|x| x);
        let comp_sin = scalar_function_to_ifs(|x| x.sin());
        let mult_components = vector_function_to_ifs(|v:Vec2| Vec2::new(v[0]*v[1],v[0]*v[1]));
        let sin_xy = comp_sin.compose(&mult_components);
        


        let contractions = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(5,1.0), 0.5),None);
       
        let ifs_final = contractions.compose(&sin_xy.compose(&v*2.0)) + &contractions;

        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(1.0,1.0,1.0), Vec3::new(1.0,1.0,1.0), Vec3::new(0.2,0.2,0.3), Vec3::new(0.0,0.33,0.67));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(7)), palette,2000,4.5);

    
        img.save(format!("out_{}.png",i))?;
    //}

    Ok(())
}
