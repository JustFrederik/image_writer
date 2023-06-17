use crate::cairopango::Painter;
use crate::input::OutputMode;
use image::{DynamicImage, ImageOutputFormat, ImageResult};
use oxipng::PngError;
use std::fs::File;
use std::io::{Cursor, Seek, Write};
use std::path::PathBuf;

fn convert_bytes_to_image(bytes: &[u8]) -> ImageResult<DynamicImage> {
    image::load_from_memory(bytes)
}

pub fn export_image<W: Write + Seek>(
    writer: &mut W,
    image: DynamicImage,
    format: ImageOutputFormat,
) -> ImageResult<()> {
    image.write_to(writer, format)
}

pub enum OutputError {
    Custom(String),
    Io(std::io::Error),
    ImageError(image::ImageError),
    CompressionError(PngError),
}

pub fn output(
    output_format: OutputMode,
    filename: PathBuf,
    f: Painter,
    add_ext: bool
) -> Result<Option<Vec<u8>>, OutputError> {
    let filepath = match add_ext {
        true => match output_format {
            OutputMode::Pdf(_) => add_ending(filename, "pdf"),
            OutputMode::Png(_) => add_ending(filename, "png"),
            OutputMode::Svg => add_ending(filename, "svg"),
            OutputMode::Ps => add_ending(filename, "ps"),
            OutputMode::Jpeg(_) => add_ending(filename, "jpeg"),
            OutputMode::Pnm(_) => add_ending(filename, "pnm"),
            OutputMode::Ico => add_ending(filename, "ico"),
            OutputMode::Bmp => add_ending(filename, "bmp"),
            OutputMode::Farbfeld => add_ending(filename, "farbfeld"),
            OutputMode::Tga => add_ending(filename, "tga"),
            OutputMode::OpenExr => add_ending(filename, "openexr"),
            OutputMode::Tiff => add_ending(filename, "tiff"),
            OutputMode::Avif => add_ending(filename, "avif"),
            OutputMode::Qoi => add_ending(filename, "qoi"),
            OutputMode::WebP => add_ending(filename, "webp"),
        },
        false => Ok(filename),
    }.map_err(OutputError::Custom)?;

    let mut file = File::create(filepath).map_err(OutputError::Io)?;

    if output_format == OutputMode::Svg
        || output_format == OutputMode::Png(false)
        || output_format == OutputMode::Pdf(false)
        || output_format == OutputMode::Ps
    {
        f.export(&mut file).map_err(OutputError::Custom)?;
        return Ok(None);
    }

    let mut cursor = Cursor::new(vec![]);
    f.export(&mut cursor).map_err(OutputError::Custom)?;
    if output_format == OutputMode::Png(true) {
        let img = oxipng::optimize_from_memory(&cursor.into_inner(), &oxipng::Options::default())
            .map_err(OutputError::CompressionError)?;
        file.write_all(&img).map_err(OutputError::Io)?;
        return Ok(None);
    }

    if output_format == OutputMode::Pdf(true) {
        return Ok(Some(cursor.into_inner()));
    }

    let image = convert_bytes_to_image(&cursor.into_inner()).map_err(OutputError::ImageError)?;
    export_image(
        &mut file,
        image,
        match output_format {
            OutputMode::Jpeg(quality) => ImageOutputFormat::Jpeg(quality),
            OutputMode::Pnm(subtype) => ImageOutputFormat::Pnm(subtype),
            OutputMode::Ico => ImageOutputFormat::Ico,
            OutputMode::Bmp => ImageOutputFormat::Bmp,
            OutputMode::Farbfeld => ImageOutputFormat::Farbfeld,
            OutputMode::Tga => ImageOutputFormat::Tga,
            OutputMode::OpenExr => ImageOutputFormat::OpenExr,
            OutputMode::Tiff => ImageOutputFormat::Tiff,
            OutputMode::Avif => ImageOutputFormat::Avif,
            OutputMode::Qoi => ImageOutputFormat::Qoi,
            OutputMode::WebP => ImageOutputFormat::WebP,
            OutputMode::Pdf(_) => unreachable!(),
            OutputMode::Png(_) => unreachable!(),
            OutputMode::Svg => unreachable!(),
            OutputMode::Ps => unreachable!(),
        },
    )
    .map_err(OutputError::ImageError)?;
    Ok(None)
}

fn add_ending(path: PathBuf, ending: &str) -> Result<PathBuf, String> {
    let mut v = path.iter().collect::<Vec<_>>();
    let last = v
        .pop()
        .ok_or("No file name")?
        .to_str()
        .ok_or("No file name")?
        .to_string();
    Ok(PathBuf::from_iter(v).join(format!("{}.{}", last, ending)))
}
