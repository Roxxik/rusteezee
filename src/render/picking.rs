use std;

use glium::{ self, Display, Surface, DrawParameters };
use glium::framebuffer::{ DepthRenderBuffer, SimpleFrameBuffer };
use glium::index::IndicesSource;
use glium::program::Program;
use glium::texture::{ UnsignedTexture2d, UncompressedUintFormat, MipmapsOption, DepthFormat };
use glium::texture::pixel_buffer::PixelBuffer;
use glium::uniforms::Uniforms;
use glium::vertex::MultiVerticesSource;
use cgmath::Point3;

use super::error::PickerCreationError;
use super::Face;
use game::chunks::ChunkPos;
use game::chunk::BlockPos;

pub struct Picker {
    pbo: PixelBuffer<u32>,
    tex: UnsignedTexture2d,
    depth: DepthRenderBuffer,
    program: Program,
}

impl Picker {
    pub fn new(display: &Display) -> Result<Picker, PickerCreationError> {
        use super::shader;
        let tex = try!(UnsignedTexture2d::empty_with_format(
            display,
            UncompressedUintFormat::U32,
            MipmapsOption::NoMipmap,
            1024, 768,
        ));
        let depth = try!(DepthRenderBuffer::new(
            display,
            DepthFormat::F32,
            1024, 768,
        ));
        Ok(Picker {
            pbo: PixelBuffer::new_empty(display, 1),
            tex: tex,
            depth: depth,
            program: try!(Program::from_source(
                display,
                shader::picking::VERTEX,
                shader::picking::FRAGMENT,
                Some(shader::picking::GEOMETRY),
            )),
        })
    }

    pub fn pick(&self) -> Option<(ChunkPos, BlockPos, Face)> {
        let (width, height) = self.get_dimensions();
        let read_target = glium::Rect {
            left: std::cmp::max(width/2, 1) - 1,
            bottom: height - (std::cmp::max(height/2, 1) - 1),
            width: 1,
            height: 1,
        };

        //read pixel
        if read_target.left < width
        && read_target.bottom < height {
            self.tex
                .main_level()
                .first_layer()
                .into_image(None).unwrap()
                .raw_read_to_pixel_buffer(&read_target, &self.pbo);
            return self.pbo.read().map(|x| x[0]).ok().and_then(|x| if x & 1 != 0 { Some(x) } else { None }).map(Picker::decode);
        } else {
            return None;
        }
    }

    fn decode(val: u32) -> (ChunkPos, BlockPos, Face) {
        assert!(val & 1 != 0);
        (
            Point3::new(
                ((val >> 20) & 0x3) as i32 - 1,
                ((val >> 18) & 0x3) as i32 - 1,
                ((val >> 16) & 0x3) as i32 - 1,
            ),
            Point3::new(
                ((val >> 12) & 0xF) as u8,
                ((val >>  8) & 0xF) as u8,
                ((val >>  4) & 0xF) as u8,
            ),
            Face::from((val >> 1) & 0x7),
        )
    }

    pub fn resize(&mut self, display: &Display, dimensions: (u32, u32)) {
        if self.get_dimensions() != dimensions {
            let (width, height) = dimensions;
            self.tex = UnsignedTexture2d::empty_with_format(
                display,
                UncompressedUintFormat::U32,
                MipmapsOption::NoMipmap,
                width, height,
            ).unwrap();
            self.depth = DepthRenderBuffer::new(
                display,
                DepthFormat::F32,
                width, height,
            ).unwrap();
        }
    }

    pub fn clear(&mut self, display: &Display) {
        let mut target = SimpleFrameBuffer::with_depth_buffer(display, &self.tex, &self.depth).unwrap();

        //clearing the attachments
        self.tex
            .main_level()
            .first_layer()
            .into_image(None).unwrap()
            .raw_clear_buffer([0, 0, 0, 0u32]);
        target.clear_depth(1.0);
    }

    pub fn draw<'b, 'c, V, I, U>(
        &mut self,
        display: &Display,
        vert_src: V,
        idx_src: I,
        uniforms: &U,
        params: &DrawParameters,
    ) where
        V: MultiVerticesSource<'b>,
        I: Into<IndicesSource<'c>>,
        U: Uniforms
    {
        let mut target = SimpleFrameBuffer::with_depth_buffer(display, &self.tex, &self.depth).unwrap();

        //drawing
        target.draw(
            vert_src,
            idx_src,
            &self.program,
            uniforms,
            params,
        ).unwrap();
    }

    fn get_dimensions(&self) -> (u32, u32) {
        (
            self.tex.get_width(),
            self.tex.get_height().unwrap(), //texture2d always has height, can safely unwrap here
        )
    }
}
