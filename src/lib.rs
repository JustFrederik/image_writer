#![allow(dead_code)]
pub mod cairopango;
pub mod input;
pub mod merge_pdf;
pub mod save;

#[cfg(test)]
mod testing {
    use crate::input::{
        Alignments, Background, Data, Font, HorizontalAlignment, Mode, OutputMode, Pos2,
        ReadDirection, Rgb, Rgba, Size2, Styling, Text, VerticalAlignment, Wrap,
    };
    use crate::save::{output, OutputError};
    use std::fs::File;

    #[test]
    fn test1() {
        let text = Text {
            mode: Mode::Text,
            value: "GHello world".to_string(),
            pos: Pos2::new(50.0, 50.0),
            size: Size2::new(100.0, 200.0),
            font_size: 12.0,
            font_color: Rgb::new(0.0, 255.0, 0.0),
            outline_color: Rgba::new(0.0, 0.0, 1.0, 0.5),
            font_stroke: 5.0,
            background: Background::Rgb(Rgb::new(1.0, 0.0, 0.0)),
            style: None,
            align: None,
        };
        let data = Data {
            items: vec![text],
            global_style: Styling {
                spacing: None,
                line_spacing: None,
                ellipsize: Default::default(),
                wrap: Wrap::Word,
                indent: None,
                single_paragraph_mode: true,
                auto_dir: false,
                read_direction: ReadDirection::default(),
                vertical: false,
                font: Some(Font {
                    font_family: "Arial Hebrew Scholar".to_string(),
                    variant: Default::default(),
                    stretch: Default::default(),
                    weight: Default::default(),
                    style: Default::default(),
                }),
                justiy: false,
                justify_last_line: false,
            },
            global_align: Alignments {
                ha: HorizontalAlignment::Left,
                va: VerticalAlignment::Top,
            },
            background: None,
        };
        let painter = data.painter(OutputMode::Svg, 1518., 2150.).unwrap();
        let mut file = File::create("test.svg").unwrap();
        match output(OutputMode::Pdf(false), "".into(), painter) {
            Ok(v) => {
                //TODO: add to output
            }
            Err(e) => match e {
                OutputError::Custom(v) => {
                    println!("Error: {}", v);
                }
                OutputError::Io(e) => {
                    println!("Io Error: {}", e);
                }
                OutputError::ImageError(e) => {
                    println!("Image Error: {}", e);
                }
                OutputError::CompressionError(e) => {
                    println!("Compression Error: {}", e);
                }
            },
        }
    }
}
