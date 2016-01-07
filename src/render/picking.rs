use std;

use glium::{ self, Display, Surface, DrawParameters };
use glium::framebuffer::{ DepthRenderBuffer, SimpleFrameBuffer };
use glium::index::IndicesSource;
use glium::program::Program;
use glium::texture::{ IntegralTexture2d, UncompressedIntFormat, MipmapsOption, DepthFormat };
use glium::texture::pixel_buffer::PixelBuffer;
use glium::uniforms::Uniforms;
use glium::vertex::MultiVerticesSource;

use super::error::PickerCreationError;

pub enum PickingResult {
    Block([i32; 3])
}

pub struct Picker {
    pbo: PixelBuffer<(i32, i32, i32)>,
    tex: IntegralTexture2d,
    depth: DepthRenderBuffer,
    program: Program,
}

impl Picker {
    pub fn new(display: &Display) -> Result<Picker, PickerCreationError> {
        use super::shader;
        let tex = try!(IntegralTexture2d::empty_with_format(
            display,
            UncompressedIntFormat::I32I32I32,
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
                shader::PICK_VERTEX,
                shader::PICK_FRAGMENT,
                None,
            )),
        })
    }

    pub fn pick(&self) -> Option<PickingResult> {
        let (width, height) = self.get_dimensions();
        let read_target = glium::Rect {
            left: std::cmp::max(width/2, 1) - 1,
            bottom: height - (std::cmp::max(height/2, 1) - 1),
            width: 1,
            height: 1,
        };

        if read_target.left < width
        && read_target.bottom < height {
            self.tex
                .main_level()
                .first_layer()
                .into_image(None).unwrap()
                .raw_read_to_pixel_buffer(&read_target, &self.pbo);
            return self.pbo.read().map(|x| PickingResult::Block([x[0].0, x[0].1, x[0].2])).ok();
        } else {
            return None;
        }
    }

    pub fn draw<'b, 'c, V, I, U>(
        &mut self,
        display: &Display,
        vert_src: V,
        idx_src: I,
        uniforms: &U,
        params: &DrawParameters,
        dimensions: (u32, u32)
    ) where
        V: MultiVerticesSource<'b>,
        I: Into<IndicesSource<'c>>,
        U: Uniforms
    {
        //update target frame
        if self.get_dimensions() != dimensions {
            let (width, height) = dimensions;
            self.tex = IntegralTexture2d::empty_with_format(
                display,
                UncompressedIntFormat::I32I32I32,
                MipmapsOption::NoMipmap,
                width, height,
            ).unwrap();
            self.depth = DepthRenderBuffer::new(
                display,
                DepthFormat::F32,
                width, height,
            ).unwrap();
        }

        //clearing the attachments
        self.tex
            .main_level()
            .first_layer()
            .into_image(None).unwrap()
            .raw_clear_buffer([20, 20, 20, 20i32]);
        let mut target = SimpleFrameBuffer::with_depth_buffer(display, &self.tex, &self.depth).unwrap();

        target.clear_depth(1.0);
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
