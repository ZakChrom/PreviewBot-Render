use std::time::Instant;

use image::{ImageBuffer, Rgba};
use wgpu::{Buffer, Device, Maintain, MapMode, Queue, RenderPipeline, Texture};

use crate::quell::cells::Grid;

pub async fn init(grid: &Grid, width: usize, height: usize, textures: &Vec<[ImageBuffer<Rgba<u8>, Vec<u8>>; 4]>) -> (Device, RenderPipeline, Texture, Buffer, Queue) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits {
                    max_buffer_size: 4294967296,
                    max_texture_dimension_2d: 16384,
                    ..Default::default()
                },
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .unwrap();

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let render_target = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
    });
    let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: width as u64 * height as u64 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            compilation_options: Default::default(),
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(wgpu::TextureFormat::Rgba8UnormSrgb.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    (device, pipeline, render_target, output_staging_buffer, queue)
}

pub fn deinit() {

}

pub fn render(grid: &Grid, width: usize, height: usize, device: &Device, pipeline: &RenderPipeline, render_target: &Texture, output_staging_buffer: &Buffer, queue: &Queue) -> Vec<u32> {
    let mut texture_data = Vec::<u8>::with_capacity(width * height * 4);
    let texture_view = render_target.create_view(&wgpu::TextureViewDescriptor::default());

    let mut command_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(&pipeline);
        render_pass.draw(0..4, 0..1);
    }
    // The texture now contains our rendered image
    command_encoder.copy_texture_to_buffer(
        wgpu::ImageCopyTexture {
            texture: &render_target,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::ImageCopyBuffer {
            buffer: &output_staging_buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(width as u32 * 4),
                rows_per_image: Some(height as u32),
            },
        },
        wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(command_encoder.finish()));
    println!("Commands submitted.");
    let start = Instant::now();
    //-----------------------------------------------

    // Time to get our image.
    let buffer_slice = output_staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(MapMode::Read, move |r| sender.send(r).unwrap());
    device.poll(Maintain::wait()).panic_on_timeout();
    receiver.recv().unwrap().unwrap();
    println!("Output buffer mapped.");
    
    texture_data.extend_from_slice(&buffer_slice.get_mapped_range()[..]);
    println!("Image data copied to local.");
    
    output_staging_buffer.unmap();
    
    println!("{}ms", start.elapsed().as_millis_f64());

    unsafe {
        texture_data.set_len(texture_data.len() / 4);
        std::mem::transmute::<&[u8], &[u32]>(texture_data.as_slice()).to_vec()
    }
}