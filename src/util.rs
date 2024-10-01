use std::{collections::HashMap, f32::consts::PI, mem::offset_of, thread::current,thread,sync::mpsc};
use image::{imageops::{self, FilterType::{CatmullRom, Gaussian, Triangle}}, Frame, Pixel, Rgba, RgbaImage};
use nalgebra::{partial_le, partial_max};
use crate::ifs::*;
use num::Complex;

pub const i:Complex<f32> = Complex::I;



pub fn gen_functions<T:Clone+ Copy + Send +'static>(func: impl Fn(Complex<f32>,T) -> Complex<f32> + Clone+ Send + 'static ,params:Vec<T>) -> Vec<Box<dyn IfsFunction>> {
	params.into_iter().map(move|param| {
		let func_clone = func.clone();
		let ifs_function:Box<dyn IfsFunction>  = Box::new(move |x|{ func_clone(x,param) } );
			return ifs_function
	}
).collect()	
}

pub fn dilate_around_point(input:Complex<f32>, params:(Complex<f32>,f32)) -> Complex<f32>{
	let target = params.0;
	let ratio = params.1;

	return input * ratio + (1.0-ratio)*target
}

pub fn sinxy(input:Complex<f32>,params:f32)->Complex<f32>{
	let freq = params;
	return Complex::<f32>::new(1.0,1.0)*(input.re*input.im*freq).sin()
}

pub fn gen_sinxy_functions(freqs:Vec<f32>) ->Vec<Box<dyn IfsFunction>> {
	return gen_functions(sinxy,freqs);
}

pub fn gen_dilations( points: &Vec<Complex<f32>>, ratio: f32) -> Vec<Box<dyn IfsFunction>> {
	let points = points.clone();
	let params = points.into_iter().map(|x| (x,ratio)).collect();
    return gen_functions(dilate_around_point, params)
}

pub fn gen_points_on_circle(num_points:usize, radius:f32,offset:f32) -> Vec<Complex<f32>>{
	return (0..num_points)
        .map(|x| {
            let theta = (((x as f32) / (num_points as f32)) * 2.0 * PI) + offset;
            return radius * Complex::<f32>::new(theta.cos(), theta.sin())
        }).collect()
}

pub fn gen_points_on_spiral(num_points:usize,increment:f32) -> Vec<Complex<f32>>{
	return (0..num_points)
        .map(|x| {
            let theta = ((x as f32) / (num_points as f32)) * 2.0 * PI;
            return increment* x as f32* Complex::<f32>::new(theta.cos(), theta.sin())
        }).collect()
}



pub fn iterate_function(mut func:impl FnMut(Complex<f32>) -> (Complex<f32>,f32),num_iters:u32) -> Vec<(Complex<f32>,f32)>{
    let mut ret_list:Vec<(Complex<f32>,f32)> = Vec::new();
    let mut current_value = (Complex::<f32>::new(1.0,1.0),0.0);
    for _ in (0..num_iters){
        current_value = func(current_value.0);
		if current_value.0.norm().is_nan(){
			current_value = (Complex::<f32>::new(1.0,1.0),0.0);
			
		}
        ret_list.push((current_value));
    }

    return ret_list;
}

pub fn render_ifs(ifs:Ifs, weights:Option<Vec<f32>>, num_iters:u32, colors:Vec<Vec3>, frame:(u32,u32), scale_factor:f32,offset:(f32,f32),gamma:f32,supersample:u32,thread_count:u32) ->RgbaImage {
	let rendering_resolution = (frame.0*supersample,frame.1*supersample);
	let resolution_min = rendering_resolution.0.min(rendering_resolution.1) as f32/2.0;
	let offset = (1.0 + 1.0*i + offset.0 + offset.1*i);
	let iters_per_thread = num_iters/thread_count;
	let (tx,rx) = mpsc::channel();

	for _ in 0..thread_count{
		let tx_clone = tx.clone();
		let ifs_clone = ifs.clone();
		let weights_clone = weights.clone();
		let colors_clone = colors.clone();
		thread::spawn(move|| {
			let mut point_counts: HashMap<(u32,u32), (HashMap<usize,u32>,u32)> = HashMap::new();
			let rng = rand::thread_rng();
			let mut func = ifs_clone.build_function(weights_clone, rng);
			let mut current_value = Complex::<f32>::new(0.0,0.0);
			let mut current_index = 0;
			for _ in 0..iters_per_thread{
				(current_value,current_index) = func(current_value);
				if current_value.norm().is_nan(){
					current_value = Complex::<f32>::new(1.0,1.0);
					current_index = 0;
					
				}
				let pixel = (current_value*scale_factor + offset) * resolution_min;
				let pixel = (pixel.re as u32,pixel.im as u32);
				point_counts.entry(pixel).and_modify(|counts| {
					counts.0.entry(current_index).and_modify(|count| *count += 1).or_insert(1);
					counts.1 += 1
				}
				).or_insert(( HashMap::from([(current_index,1)]),1 ));
			}
			println!("thread finished generating points");
			let mut max_count = 0;
			for (_,counts) in &point_counts{
				max_count = max_count.max( counts.1) ;
			}
			let mut img = RgbaImage::new(rendering_resolution.0, rendering_resolution.1);
			img.pixels_mut().for_each(|p| *p = Rgba([255, 255, 255, 0]));
			for ((x, y), counts) in point_counts {
				let total_count = counts.1;
				let alpha = (total_count as f32/max_count as f32).powf(1.0/gamma) * 255.0;
				let rgbcolor = counts.0.iter().fold(Vec3::new(0.0,0.0,0.0),|acc,x| acc + (colors_clone[*x.0] * *x.1 as f32) / total_count as f32) * 255.0;
				let rgbacolor = Rgba([rgbcolor.x as u8,rgbcolor.y as u8,rgbcolor.z as u8, alpha as u8]);
			
				if x<rendering_resolution.0 && y < rendering_resolution.1{
					
					img.put_pixel(x, y,rgbacolor);
				}
			}
			tx_clone.send(imageops::resize(&img,frame.0,frame.1, CatmullRom));
			println!("thread finished generating image");
			drop(tx_clone);
		});
	}
	drop(tx);
	let mut final_image = RgbaImage::new(frame.0, frame.1);
	final_image.pixels_mut().for_each(|p| *p = Rgba([255, 255, 255, 255]));
	
	for img in rx{
		imageops::overlay(&mut final_image, &img, 0, 0);
	}

	return final_image

}
/*
pub fn points_to_image(points:Vec<(Complex<f32>,f32)>, palette: impl Fn(f32)->Vec3,resolution:u32,scale_factor:f32, offset:Complex<f32>, gamma:f32)->RgbImage{
	

    let pixels: Vec<_> = points
		.into_iter()
		.map(|pt| ( (((pt.0*scale_factor) + Complex::<f32>::new(1.0,1.0) + offset)*(resolution - 1) as f32/2.0), pt.1))
		.map(|(pos, color)| {
			let pos = (pos.re as u32, pos.im as u32);
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
		if x<resolution && y < resolution{
        	img.put_pixel(x, y, Rgb([rgbcolor.x as u8,rgbcolor.y as u8,rgbcolor.z as u8]));
		}
    }
	return img;
}
*/
pub fn generate_color_pallete(a:Vec3, b:Vec3, c:Vec3, d:Vec3) -> impl Fn(f32) -> Vec3{
    return move |x| a + b.component_mul(&(2.0 * PI * (c*x + d)).map(f32::cos)) 
}
