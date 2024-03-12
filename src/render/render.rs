use std::io::Write;
use std::process::{Command as ExecCommand, Stdio};
use std::path::Path;
use std::time::Instant;

use image::{self, ImageBuffer, Rgba};
use image::DynamicImage;

use crate::quell::cells::Grid;
use crate::quell::update::update;

fn render_grid(grid: Grid, textures: Vec<[ImageBuffer<Rgba<u8>, Vec<u8>>; 4]>) -> Vec<u32> {
    let width = grid.width * 16;
    let height = grid.height * 16;
    let mut pixels: Vec<u32> = vec![0; width*height];
    
    grid.for_each(|x, y, cell| {
        let image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
        if let Some(cell) = cell {
            image = textures[cell.id() as usize][cell.direction() as usize].clone();
        } else {
            image = textures[0][0].clone();
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

        // let mut pix = image.into_vec();
        // for cy in 0..16 {
        //     let i = (py+cy as usize) * width;
        //     let i2 = cy*width;
        //     unsafe {
        //         std::ptr::copy_nonoverlapping(&pix[i2..i2+16].as_ptr(), std::mem::transmute(pixels[i..i+16].as_mut_ptr()), 16);
        //     }
        // }

        for cy in 0..16 {
            for cx in 0..16 {
                unsafe {
                    let p = image.get_pixel(cx, cy).0;
                    let pixel: u32 = std::mem::transmute(p);
                    pixels[(py+cy as usize) * width as usize + (px+cx as usize)] = pixel;
                }
            }
        }
    });
    pixels
}

// https://stackoverflow.com/questions/49690459/converting-a-vecu32-to-vecu8-in-place-and-with-minimal-overhead
// pub fn vec_u32_to_u8(data: &Vec<u32>) -> Vec<u8> {
//     // TODO: https://stackoverflow.com/questions/72631065/how-to-convert-a-u32-array-to-a-u8-array-in-place
//     // TODO: https://stackoverflow.com/questions/29037033/how-to-slice-a-large-veci32-as-u8
//     let capacity = 32/8 * data.len() as usize;  // 32/8 == 4
//     let mut output = Vec::<u8>::with_capacity(capacity);
//     for &value in data {
//         output.push((value >> 24) as u8);  // r
//         output.push((value >> 16) as u8);  // g
//         output.push((value >>  8) as u8);  // b
//         output.push((value >>  0) as u8);  // a
//     }
//     output
// }

fn rotate_ccw(img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
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
}

pub fn render(grid: &mut Grid, ticks: u64, tps: u64, output_file: &str) -> (Vec<u128>, Vec<u128>, Vec<u128>) {
    let mut fake_textures: Vec<DynamicImage> = Vec::new();
    fake_textures.push(image::open(&Path::new("textures/BGDefault.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/wall.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/mover.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/generator.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/CWspinner_alt.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/CCWspinner_alt.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/block.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/slide.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/trash.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/enemy.png")).unwrap());
    fake_textures.push(image::open(&Path::new("textures/BGPlaceable0.png")).unwrap());

    let mut textures: Vec<[ImageBuffer<Rgba<u8>, Vec<u8>>; 4]> = Vec::with_capacity(11);

    for texture in fake_textures {
        let mut real_texture = texture.to_rgba8().clone();
        for cy in 0..16 {
            for cx in 0..16 {
                let p = real_texture.get_pixel(cx, cy).0;
                let mut r = p[0];
                let mut g = p[1];
                let mut b = p[2];
                let a = p[3];
                if a == 0 {
                    r = 0x2A;
                    g = 0x2A;
                    b = 0x2A;
                }
                real_texture.put_pixel(cx, cy, [r, g, b, a].into());
            }
        }

        let mut tex: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> = Vec::new();
        tex.push(real_texture.clone());

        rotate_ccw(&mut real_texture);
        tex.push(real_texture.clone());
        
        rotate_ccw(&mut real_texture);
        tex.push(real_texture.clone());
        
        rotate_ccw(&mut real_texture);
        tex.push(real_texture.clone());

        textures.push(tex.try_into().unwrap());
    }

    let mut child = ExecCommand::new("ffmpeg")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .arg("-loglevel").arg("warning")
        .arg("-y")
        .arg("-f").arg("rawvideo")
        .arg("-pix_fmt").arg("rgba")
        .arg("-s").arg(format!("{}x{}", grid.width*16, grid.height*16))
        .arg("-r").arg(format!("{}", tps))
        .arg("-i").arg("-")
        
        .arg("-frames:v").arg(format!("{}", ticks))
        .arg("-c:v").arg("libx264")
        .arg("-crf").arg("27")
        .arg("-preset").arg("ultrafast")
        .arg("-vb").arg("2500k")
        .arg("-pix_fmt").arg("yuv420p")
        .arg(output_file)

        .spawn().expect("Failed to start ffmpeg");

    let mut child_stdin = child.stdin.as_ref().unwrap();
    let mut update_times: Vec<u128> = Vec::new();
    let mut render_times: Vec<u128> = Vec::new();
    let mut write_times: Vec<u128> = Vec::new();

    for _t in 0..ticks {
        let mut now = Instant::now();
        let mut pixels = render_grid(grid.clone(), textures.clone());
        let end1 = now.elapsed().as_millis();
        render_times.push(end1);
        
        now = Instant::now();
        update(grid);
        let end2 = now.elapsed().as_micros();
        update_times.push(end2);
        
        now = Instant::now();
        unsafe {
            pixels.set_len(pixels.len() * 4);
            child_stdin.write(std::mem::transmute(pixels.as_slice())).unwrap();
        }
        let end3 = now.elapsed().as_millis();
        write_times.push(end3);

        println!("update(): {}μs | render_grid(): {}ms | write(): {}ms", end2, end1, end3);
    }
    child.wait().unwrap();
    
    (update_times, render_times, write_times)
}
