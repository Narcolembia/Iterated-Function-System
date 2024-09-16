#![allow(unused, non_snake_case)]


mod util;
mod ifs;

use ifs::{Ifs, Vec2, Vec3};
use anyhow::Result as AResult;
use image::{RgbaImage, Rgba};
use util::{gen_dilations, gen_points_on_circle, gen_sinxy_functions};



fn main() -> AResult<()> {
    let i = 1;
    
    //for i in (0..100){
        let rng = rand::thread_rng();
    

        let p5_r1 = Ifs::new(
            &gen_dilations(
                &gen_points_on_circle(5,1.0), 0.1),None);
        let p10_r2 = Ifs::new(
            &gen_dilations(
                &gen_points_on_circle(10,3.0), 0.15),None);
        let sin_xy_1 = Ifs::new(
        &gen_sinxy_functions((0..5).map(|x| 1.0).collect()),None);
        let sin_xy_2 = Ifs::new(
            &gen_sinxy_functions((0..5).map(|x| 2.0).collect()),None);
       
        let ifs_final = (&p5_r1 + &(&p5_r1 + &sin_xy_1)).extend(&(&(&sin_xy_2 + &p10_r2) + &p10_r2)) ;
        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(1.0,1.0,1.0), Vec3::new(1.0,1.0,1.0), Vec3::new(0.2,0.2,0.3), Vec3::new(0.0,0.33,0.67));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(7)), palette,2000,4.5);

    
        img.save(format!("out_{}.png",i))?;
    //}

    Ok(())
}
