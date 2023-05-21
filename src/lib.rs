#![allow(dead_code)]
mod cairopango;
mod input;
mod merge_pdf;
mod save;

#[cfg(test)]
mod testing {
    use crate::input::{
        Alignments, Background, Data, HorizontalAlignment, Mode, OutputMode, Pos2, ReadDirection,
        Rgb, Rgba, Size2, Styling, Text, VerticalAlignment, Wrap,
    };
    use std::fs::File;

    #[test]
    fn test1() {
        let now = std::time::Instant::now();
        let text = Text {
            mode: Mode::Text,
            value: "中文".to_string(),
            pos: Pos2::new(50.0, 50.0),
            size: Size2::new(100.0, 200.0),
            font_size: 22.0,
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
                vertical: true,
                font: None,
                justiy: false,
                justify_last_line: false,
            },
            global_align: Alignments {
                ha: HorizontalAlignment::Left,
                va: VerticalAlignment::Top,
            },
            background: None,
        };
        let painter = data.painter(OutputMode::Pdf(false), 1518., 2150.).unwrap();
        let mut file = File::create("test.pdf").unwrap();
        painter.export(&mut file).unwrap();
        println!("Time: {:?}", now.elapsed());
    }
}