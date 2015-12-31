//pub fn loadTexture(path: &str) -> SrgbTexture2d {
//    let image = image::load(std::io::Cursor::new(&include_bytes!(
//        "../assets/textures/dirt.png"
//    )[..]),image::PNG).unwrap().to_rgba();
//    let image_dimensions = image.dimensions();
//    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
//    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();
//}

//TODO somehow load a texture ...
