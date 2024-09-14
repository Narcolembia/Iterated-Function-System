use std::{collections::HashMap, f32::consts::PI};
use image::{RgbImage, Rgb};
use nalgebra::{partial_le, partial_max};
use crate::ifs::*;


pub fn gen_functions<T:Clone+ Copy +'static>(func: impl Fn(Vec2,T) -> Vec2 + Clone + 'static ,params:Vec<T>) -> Vec<Box<dyn IfsFunction>> {
	params.into_iter().map(move|param| {
		let func_clone = func.clone();
		let ifs_function:Box<dyn IfsFunction>  = Box::new(move |x|{ func_clone(x,param) } );
			return ifs_function
	}
).collect()	
}

pub fn move_twords_point(input:Vec2, params:(Vec2,f32)) -> Vec2{
	let target = params.0;
	let ratio = params.1;

	return input * ratio + (1.0-ratio)*target
}

pub fn sinxy_mult(input:Vec2,params:f32)->Vec2{
	let freq = params;
	return Vec2::new(1.0,1.0)*(input[0]*input[1]*freq).sin()
}

pub fn gen_sinxy_mult_functions(freqs:Vec<f32>) ->Vec<Box<dyn IfsFunction>> {
	return gen_functions(sinxy_mult,freqs);
}

pub fn gen_chaos_game_functions( points: &Vec<Vec2>, ratio: f32) -> Vec<Box<dyn IfsFunction>> {
	let points = points.clone();
	let params = points.into_iter().map(|x| (x,ratio)).collect();
    return gen_functions(move_twords_point, params)
}

pub fn gen_points_on_unit_circle(num_points:usize) -> Vec<Vec2>{
	return (0..num_points)
        .map(|x| {
            let theta = ((x as f32) / (num_points as f32)) * 2.0 * PI;
            return Vec2::new(theta.cos(), theta.sin())
        }).collect()
}

pub fn gen_points_on_spiral(num_points:usize,increment:f32) -> Vec<Vec2>{
	return (0..num_points)
        .map(|x| {
            let theta = ((x as f32) / (num_points as f32)) * 2.0 * PI;
            return increment* x as f32* Vec2::new(theta.cos(), theta.sin())
        }).collect()
}



pub fn iterate_function(mut func:impl FnMut(Vec2) -> (Vec2,f32),num_iters:u32) -> Vec<(Vec2,f32)>{
    let mut ret_list:Vec<(Vec2,f32)> = Vec::new();
    let mut current_value = (Vec2::new(0.0,0.0),0.0);
    for _ in (0..num_iters){
        current_value = func(current_value.0);
        ret_list.push((current_value));
    }

    return ret_list;
}


pub fn points_to_image(points:Vec<(Vec2,f32)>, palette: impl Fn(f32)->Vec3,resolution:u32, gamma:f32)->RgbImage{
	let max = points
		.iter()
		.map(|&(pt, _)| pt.magnitude())
		.reduce(|l, r| if l > r { l } else { r })
		.unwrap();


    let pixels: Vec<_> = points
		.into_iter()
		.map(|pt| ( (((pt.0/max) + Vec2::new(1.0,1.0))*(resolution - 1) as f32/2.0), pt.1))
		.map(|(pos, color)| {
			let pos = (pos.x as u32, pos.y as u32);
			(pos, palette(color))
		})
		.collect();
    
	let mut pointCounts: HashMap<(u32, u32), (u32, Vec3)> = HashMap::new();
    for &(pos, color) in &pixels {
		pointCounts
			.entry(pos)
			.and_modify(|(count, avgColor)| {
				*count += 1;
				*avgColor = (*avgColor*0.9 + color*0.1);
                
			})
			.or_insert((1, color));
	}
	let maxCount = pointCounts
		.values()
		.map(|&(count, _)| count)
		.max()
		.unwrap();

    let mut img = RgbImage::new(resolution, resolution);
	img.pixels_mut().for_each(|p| *p = Rgb([0, 0, 0]));
    for ((x, y), (count, color)) in pointCounts {
		let rgbcolor = color * 255.0 * (count as f32 / maxCount as f32).powf(1.0/gamma);
        img.put_pixel(x, y, Rgb([rgbcolor.x as u8,rgbcolor.y as u8,rgbcolor.z as u8]));
    }
	return img;
}

pub fn generate_color_pallete(a:Vec3, b:Vec3, c:Vec3, d:Vec3) -> impl Fn(f32) -> Vec3{
    return move |x| a + b.component_mul(&(2.0 * PI * (c*x + d)).map(f32::cos)) 
}
