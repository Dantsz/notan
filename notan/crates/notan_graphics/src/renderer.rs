use crate::buffer::*;
use crate::commands::*;
use crate::pipeline::*;
use crate::texture::*;

#[derive(Default, Clone)]
pub struct Renderer<'a> {
    commands: Vec<Commands<'a>>,
    size: (i32, i32),
}

impl<'a> Renderer<'a> {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            size: (width, height),
            commands: vec![],
        }
    }

    pub fn begin(&mut self, options: &ClearOptions) {
        self.commands.push(Commands::Begin {
            color: options.color,
            stencil: options.stencil,
            depth: options.depth,
        });
    }

    pub fn end(&mut self) {
        self.commands.push(Commands::End);
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.size = (width, height);
        self.commands.push(Commands::Size { width, height });
    }

    pub fn size(&self) -> (i32, i32) {
        self.size
    }

    pub fn width(&self) -> i32 {
        self.size.0
    }

    pub fn height(&self) -> i32 {
        self.size.1
    }

    pub fn set_viewport(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.commands.push(Commands::Viewport {
            x,
            y,
            width,
            height,
        });
    }

    pub fn set_pipeline(&mut self, pipeline: &Pipeline) {
        self.commands.push(Commands::Pipeline {
            id: pipeline.id(),
            options: pipeline.options.clone(),
        });
    }

    pub fn bind_vertex_buffer(&mut self, buffer: &Buffer, data: &'a [f32]) {
        self.commands.push(Commands::BindBuffer {
            id: buffer.id(),
            ptr: bytemuck::cast_slice(data),
            usage: BufferUsage::Vertex,
            draw: buffer.draw.unwrap_or(DrawType::Dynamic),
        });
    }

    pub fn bind_index_buffer(&mut self, buffer: &Buffer, data: &'a [u32]) {
        self.commands.push(Commands::BindBuffer {
            id: buffer.id(),
            ptr: bytemuck::cast_slice(data),
            usage: BufferUsage::Index,
            draw: buffer.draw.unwrap_or(DrawType::Dynamic),
        });
    }

    pub fn bind_uniform_buffer(&mut self, buffer: &Buffer, data: &'a [f32]) {
        self.commands.push(Commands::BindBuffer {
            id: buffer.id(),
            ptr: bytemuck::cast_slice(data),
            usage: buffer.usage,
            draw: buffer.draw.unwrap_or(DrawType::Dynamic),
        });
    }

    pub fn draw(&mut self, offset: i32, count: i32) {
        self.commands.push(Commands::Draw { offset, count })
    }

    pub fn bind_texture(&mut self, location: u32, texture: &Texture) {
        self.bind_texture_slot(0, location, texture);
    }

    pub fn bind_texture_slot(&mut self, slot: u32, location: u32, texture: &Texture) {
        self.commands.push(Commands::BindTexture {
            slot,
            location,
            id: texture.id(),
        })
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

impl<'a> ToCommandBuffer<'a> for Renderer<'a> {
    fn commands(&'a self) -> &'a [Commands<'a>] {
        &self.commands
    }
}