use std::{collections::HashMap, f32::consts::PI, mem::offset_of, thread::current,thread,sync::mpsc};
use image::{imageops::{self, FilterType::{CatmullRom, Gaussian, Triangle}}, Frame, Pixel, Rgba, RgbaImage,GrayAlphaImage,LumaA};
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

pub fn render_ifs(ifs:&Ifs, weights:Option<Vec<f32>>, colors:&Vec<Vec3>, background_color:Option<&Vec<u8>>, num_iters:u32, frame:(u32,u32), scale_factor:f32,offset:(f32,f32),rotation:f32,gamma:f32,supersample:u32,thread_count:u32) ->RgbaImage {
	let rendering_resolution = (frame.0*supersample,frame.1*supersample);
	
	let resolution_min = rendering_resolution.0.min(rendering_resolution.1) as f32;
	
	let offset = (rendering_resolution.0 as f32/resolution_min + (rendering_resolution.1 as f32/resolution_min)*i + offset.0 + offset.1*i);
	let rotation = Complex::<f32>::cis(rotation);
	let iters_per_thread = num_iters/thread_count;
	let (tx,rx) = mpsc::channel();
	let background_color = match background_color{
		Some(x) => x,
		None => &[0,0,0,0].to_vec(),
	};

	for _ in 0..thread_count{
		let tx_clone = tx.clone();
		let ifs_clone = ifs.clone();
		let weights_clone = weights.clone();
		let colors_clone = colors.clone();
		thread::spawn(move|| {
			let mut point_counts: HashMap<(u32,u32), (HashMap<usize,u32>,u32)> = HashMap::new();
			let rng = rand::thread_rng();
			let mut func = ifs_clone.build_function(weights_clone, rng);
			let mut current_value = Complex::<f32>::new(rand::random(),rand::random());
			let mut current_index = 0;
			for _ in 0..iters_per_thread{
				(current_value,current_index) = func(current_value);
				if current_value.norm().is_nan(){
					current_value = Complex::<f32>::new(rand::random(),rand::random());
					current_index = 0;	
				}
				
				let pixel = (current_value*scale_factor*rotation + offset) * resolution_min/2.0;
				
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
			tx_clone.send(imageops::resize(&img,frame.0,frame.1, Triangle));
			println!("thread finished generating image");
			drop(tx_clone);
		});
	}
	drop(tx);
	let mut final_image = RgbaImage::new(frame.0, frame.1);
	final_image.pixels_mut().for_each(|p| *p = Rgba([background_color[0]as u8, background_color[1]as u8, background_color[2] as u8, background_color[3]]));
	
	for img in rx{
		imageops::overlay(&mut final_image, &img, 0, 0);
	}

	return final_image

}
pub fn render_ifs_greyscale(ifs:&Ifs, weights:Option<Vec<f32>>, transparent_background:bool, num_iters:u32, frame:(u32,u32), scale_factor:f32,offset:(f32,f32),rotation:f32,gamma:f32,supersample:u32,thread_count:u32) ->GrayAlphaImage {
	let rendering_resolution = (frame.0*supersample,frame.1*supersample);
	
	let resolution_min = rendering_resolution.0.min(rendering_resolution.1) as f32;
	
	let offset = (rendering_resolution.0 as f32/resolution_min + (rendering_resolution.1 as f32/resolution_min)*i + offset.0 + offset.1*i);
	let rotation = Complex::<f32>::cis(rotation);
	let iters_per_thread = num_iters/thread_count;
	let (tx,rx) = mpsc::channel();

	for _ in 0..thread_count{
		let tx_clone = tx.clone();
		let ifs_clone = ifs.clone();
		let weights_clone = weights.clone();
		let num_points = rendering_resolution.0*rendering_resolution.1;
		thread::spawn(move|| {
			let mut point_counts: nalgebra::DMatrix<f32> = nalgebra::DMatrix::from_vec(rendering_resolution.0 as usize,rendering_resolution.1 as usize,(0..num_points).map(|x| 0.0).collect());
			let rng = rand::thread_rng();
			let mut func = ifs_clone.build_function(weights_clone, rng);
			let mut current_value = Complex::<f32>::new(rand::random(),rand::random());
			for _ in 0..iters_per_thread{
				(current_value,_) = func(current_value);
				if current_value.norm().is_nan(){
					current_value = Complex::<f32>::new(rand::random(),rand::random());

				}
				
				let pixel = (current_value*scale_factor*rotation + offset) * resolution_min/2.0;
				let pixel = (pixel.re as u32,pixel.im as u32);
				if pixel.0 < rendering_resolution.0 && pixel.1 < rendering_resolution.1{
					point_counts[(pixel.0 as usize,pixel.1 as usize)] += 1.0;
				}
			}
			println!("thread finished generating points");
			println!("{}",point_counts.max());
			let max = point_counts.max();
			point_counts = point_counts/max;

			let mut img = GrayAlphaImage::new(rendering_resolution.0, rendering_resolution.1);

			img.pixels_mut().for_each(|p| *p = LumaA([255, 0]));

			for x in 0..rendering_resolution.0{
				for y in 0..rendering_resolution.1{
					//println!("{}",point_counts[(x as usize,y as usize)] *255.0);
					let alpha = ((point_counts[(x as usize,y as usize)]).powf(1.0/gamma)*255.0) as u8;
					img.put_pixel(x, y,LumaA([0,alpha]));
				}
			}
			tx_clone.send(imageops::resize(&img,frame.0,frame.1, Triangle));
			println!("thread finished generating image");
			drop(tx_clone);
		});
	}
	drop(tx);
	let mut final_image = GrayAlphaImage::new(frame.0, frame.1);
	if transparent_background{ 
		final_image.pixels_mut().for_each(|p| *p = LumaA([0,0]));
	}
	else {
		final_image.pixels_mut().for_each(|p| *p = LumaA([255,255]));
	}
	
	for img in rx{
		imageops::overlay(&mut final_image, &img, 0, 0);
	}

	return final_image

}
pub fn generate_color_pallete(a:Vec3, b:Vec3, c:Vec3, d:Vec3) -> impl Fn(f32) -> Vec3{
    return move |x| a + b.component_mul(&(2.0 * PI * (c*x + d)).map(f32::cos)) 
}
