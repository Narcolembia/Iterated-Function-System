#![allow(unused, non_snake_case)]


mod util;
mod ifs;

use ifs::{Ifs, Vec2, Vec3};
use anyhow::Result as AResult;
use image::{RgbaImage, Rgba};
use util::{gen_chaos_game_functions, gen_points_on_unit_circle, gen_sinxy_mult_functions};



fn main() -> AResult<()> {
    let i = 1;
    
    //for i in (0..100){
        let rng = rand::thread_rng();
        let num_points_1 = 10;

        
        let ratio_1 = 0.3;
        let frequencys_1 = (0..num_points_1).map(|x| 1.5).collect(); 
        let ratio_2 = 0.25;


        let points_1 = util::gen_points_on_spiral(num_points_1,1.0/num_points_1 as f32);
        let points_2 = util::gen_points_on_unit_circle(num_points_1);

        let funcs_1 = gen_chaos_game_functions(&points_1, ratio_1);
        let funcs_2 = gen_sinxy_mult_functions(frequencys_1);
        let funcs_3 = gen_chaos_game_functions(&points_2, ratio_1);
        let funcs_4 = gen_chaos_game_functions(&points_2, ratio_2);
        
        let ifs1 = Ifs::new(&funcs_1, None);
        let ifs2 = Ifs::new(&funcs_2,None);
        let ifs3 = Ifs::new(&funcs_3,None);
        let ifs4 = Ifs::new(&funcs_4,None);
        let ifs_final = (&(&ifs2 * &ifs3) + &ifs3).extend(&(&ifs4 * &ifs2));
        let mut func = ifs_final.build_function(rng);
        //dbg!(util::iterate_function(func, 10u32.pow(1)));
        let palette = util::generate_color_pallete(Vec3::new(1.0,1.0,1.0), Vec3::new(1.0,1.0,1.0), Vec3::new(0.5,0.5,0.5), Vec3::new(0.0,0.33,0.67));
        let img = util::points_to_image(util::iterate_function(func, 10u32.pow(8)), palette,2000,6.0);

    
        img.save(format!("out_{}.png",i))?;
    //}

    Ok(())
}
