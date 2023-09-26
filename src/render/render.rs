use std::io::Write;
use std::process::{Command as ExecCommand, Stdio};
use std::path::Path;
use std::time::Instant;

use image;
use image::DynamicImage;
use image::Pixel;

use crate::quell::cells::Grid;
use crate::quell::direction::Direction;
use crate::quell::update::update;

fn render_grid(grid: Grid, textures: Vec<DynamicImage>) -> Vec<u32> {
	let width = grid.width * 16;
	let height = grid.height * 16;
	let mut pixels: Vec<u32> = Vec::with_capacity(width*height);
	for _i in 0..width*height {
		pixels.push(0);
	}
	
	grid.for_each(|x, y, cell| {
		let mut image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
		let mut direction = Direction::Right;
		if let Some(cell) = cell {
			image = textures[cell.id() as usize].to_rgba8();
			direction = cell.direction();
		} else {
			image = textures[0].to_rgba8();
		}
		let px = x as usize * 16;
		let py = (grid.height-1-y as usize) * 16;

		/*let cell_pixels1: &[u32] = unsafe {
			std::slice::from_raw_parts(image.as_ptr() as *const u32, image.len() / 4)
		};
		let mut cell_pixels = unsafe {
			let const_ptr = cell_pixels1 as *const [u32];
			let mut_ptr = const_ptr as *mut [u32];
			&mut *mut_ptr
		};*/
		
		match direction {
			Direction::Right => {}
			Direction::Down => {
				rotate_ccw(&mut image);
			}
			Direction::Left => {
				rotate_ccw(&mut image);
				rotate_ccw(&mut image);
			}
			Direction::Up => {
				rotate_ccw(&mut image);
				rotate_ccw(&mut image);
				rotate_ccw(&mut image);
			}
		}

		
		for cy in 0..16 {
			for cx in 0..16 {
				let p = image.get_pixel(cx, cy).channels();
				let mut r = p[0] as u32;
				let mut g = p[1] as u32;
				let mut b = p[2] as u32;
				let a = p[3] as u32;
				/*let r = image[cy*4*16+cx*4] as u32;
				let g = image[cy*4*16+cx*4+1] as u32;
				let b = image[cy*4*16+cx*4+2] as u32;
				let a = image[cy*4*16+cx*4+3] as u32;*/
				if (a == 0) {
					r = 0x2A;
					g = 0x2A;
					b = 0x2A;
				}
				let pixel = (r<<24)|(g<<16)|(b<<8)|255;
				pixels[(py+cy as usize) * width as usize + (px+cx as usize)] = pixel;
			}
		}
	});
	pixels
}
// https://stackoverflow.com/questions/49690459/converting-a-vecu32-to-vecu8-in-place-and-with-minimal-overhead
pub fn vec_u32_to_u8(data: &Vec<u32>) -> Vec<u8> {
    // TODO: https://stackoverflow.com/questions/72631065/how-to-convert-a-u32-array-to-a-u8-array-in-place
    // TODO: https://stackoverflow.com/questions/29037033/how-to-slice-a-large-veci32-as-u8
    let capacity = 32/8 * data.len() as usize;  // 32/8 == 4
    let mut output = Vec::<u8>::with_capacity(capacity);
    for &value in data {
        output.push((value >> 24) as u8);  // r
        output.push((value >> 16) as u8);  // g
        output.push((value >>  8) as u8);  // b
        output.push((value >>  0) as u8);  // a
    }
    output
}

fn rotate_ccw(img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
	/*let mut newpixels: Vec<Vec<u32>> = vec![vec![0; 16]; 16];
	for i in 0..16 {
		for j in 0..16 {
			newpixels[i][j] = pixels[i*16+j];
		}
	}*/
	for i in 0..16/2 {
		for j in i..16-i-1 {
			let temp  = img.get_pixel(i,      j).clone();
			let temp2 = img.get_pixel(j,      16-i-1).clone();
			let temp3 = img.get_pixel(16-i-1, 16-j-1).clone();
			let temp4 = img.get_pixel(16-j-1, i).clone();

			img.put_pixel(i,      j,      temp2);
			img.put_pixel(j,      16-i-1, temp3);
			img.put_pixel(16-i-1, 16-j-1, temp4);
			img.put_pixel(16-j-1, i,      temp);
		}
	}
	/*for i in 0..16 {
		for j in 0..16 {
			pixels[i*16+j] = newpixels[i][j];
		}
	}*/
}

pub fn render(grid: &mut Grid, ticks: usize, output_file: &str) {
	let mut textures: Vec<DynamicImage> = Vec::new();
	textures.push(image::open(&Path::new("textures/BGDefault.png")).unwrap());
	textures.push(image::open(&Path::new("textures/wall.png")).unwrap());
	textures.push(image::open(&Path::new("textures/mover.png")).unwrap());
	textures.push(image::open(&Path::new("textures/generator.png")).unwrap());
	textures.push(image::open(&Path::new("textures/CWspinner_alt.png")).unwrap());
	textures.push(image::open(&Path::new("textures/CCWspinner_alt.png")).unwrap());
	textures.push(image::open(&Path::new("textures/block.png")).unwrap());
	textures.push(image::open(&Path::new("textures/slide.png")).unwrap());
	textures.push(image::open(&Path::new("textures/trash.png")).unwrap());
	textures.push(image::open(&Path::new("textures/enemy.png")).unwrap());
	textures.push(image::open(&Path::new("textures/BGPlaceable0.png")).unwrap());


	let mut child = ExecCommand::new("ffmpeg")
		.stdin(Stdio::piped())
		.arg("-loglevel").arg("verbose")
		.arg("-y")
		.arg("-f").arg("rawvideo")
		.arg("-pix_fmt").arg("rgba")
		.arg("-s").arg(format!("{}x{}", grid.width*16, grid.height*16))
		.arg("-r").arg("5")
		.arg("-i").arg("-")

		.arg("-c:v").arg("libx264")
		.arg("-vb").arg("2500k")
		.arg("-pix_fmt").arg("yuv420p")
		.arg(output_file)

		.spawn().expect("Failed to start ffmpeg");

	let mut child_stdin = child.stdin.as_ref().unwrap();
	for _t in 0..ticks {
		let mut now = Instant::now();
		let pixels = render_grid(grid.clone(), textures.clone());
		let end1 = now.elapsed().as_millis();
		
		now = Instant::now();
		update(grid);
		let end2 = now.elapsed().as_micros();
		
		println!("update(): {}micros | render_grid(): {}millis", end2, end1);
		child_stdin.write(&vec_u32_to_u8(&pixels)).unwrap();
	}
	child.wait().unwrap();
}
