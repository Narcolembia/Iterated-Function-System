
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
        


        let p5_r1 = Ifs::new(
            gen_dilations(
                &gen_points_on_circle(5,1.0), 0.5),None);
       
        let ifs_final = p5_r1.compose(&sin_xy.compose(&v *&v*2.0)) + &p5_r1;
        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(1.0,1.0,1.0), Vec3::new(1.0,1.0,1.0), Vec3::new(0.2,0.2,0.3), Vec3::new(0.0,0.33,0.67));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(7)), palette,2000,4.5);

    
        img.save(format!("out_{}.png",i))?;
    //}

    Ok(())
}
