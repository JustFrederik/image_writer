use std::f32::consts::PI;
use std::io::{Cursor, Write};

use cairo_rs::{Context, Format, ImageSurface, LineJoin, PdfSurface, PsSurface, SvgSurface};
use pango::{
    Alignment, Direction, FontDescription, Layout, Stretch, Style, Variant, Weight, SCALE,
};
use pangocairo::{create_context, layout_path};

use crate::input::{
    Alignments, Background, Data, Ellipsize, FontStretch, FontStyle, FontVariant, FontWeight,
    HorizontalAlignment, Mode, OutputMode, Pos2, ReadDirection, Rgb, Rgba, Size2, Styling, Text,
    VerticalAlignment, Wrap,
};

enum Surfaces {
    Pdf(PdfSurface),
    Png(ImageSurface),
    Svg(SvgSurface),
    Ps(PsSurface),
}

pub struct Painter {
    surface: Surfaces,
    _context1: Context,
    context2: Context,
    context3: Context,
    context4: Context,
    pango_context1: pango::Context,
    pango_context2: pango::Context,
}

impl VerticalAlignment {
    fn set(&self, layout: &Layout, context: &Context, pos: &Pos2, size: &Size2, vertical: bool) {
        if vertical {
            HorizontalAlignment::set(
                &match self {
                    Self::Top => HorizontalAlignment::Left,
                    Self::Center => HorizontalAlignment::Center,
                    Self::Bottom => HorizontalAlignment::Right,
                },
                layout,
                context,
                pos,
                size,
                false,
            );
        } else {
            let v = layout.pixel_size();

            context.move_to(
                pos.x,
                pos.y
                    + match self {
                        Self::Top => 0.,
                        Self::Bottom => size.height - v.1 as f64,
                        Self::Center => (size.height - v.1 as f64) / 2.,
                    },
            );
        }
    }
}

impl HorizontalAlignment {
    fn set(&self, layout: &Layout, context: &Context, pos: &Pos2, size: &Size2, vertical: bool) {
        if vertical {
            let v = layout.pixel_size();
            context.move_to(
                pos.x,
                pos.y
                    + match self {
                        Self::Left => size.width - v.1 as f64,
                        Self::Center => (size.width - v.1 as f64) / 2.,
                        Self::Right => 0.,
                    },
            );
        } else {
            match self {
                Self::Left => layout.set_alignment(Alignment::Left),
                Self::Right => layout.set_alignment(Alignment::Right),
                Self::Center => layout.set_alignment(Alignment::Center),
            };
        }
    }
}

impl Wrap {
    fn set(&self, layout: &Layout) {
        match self {
            Wrap::Word => layout.set_wrap(pango::WrapMode::Word),
            Wrap::Char => layout.set_wrap(pango::WrapMode::Char),
            Wrap::WordChar => layout.set_wrap(pango::WrapMode::WordChar),
        };
    }
}

impl Ellipsize {
    fn set(&self, layout: &Layout) {
        match self {
            Ellipsize::None => layout.set_ellipsize(pango::EllipsizeMode::None),
            Ellipsize::Start => layout.set_ellipsize(pango::EllipsizeMode::Start),
            Ellipsize::Middle => layout.set_ellipsize(pango::EllipsizeMode::Middle),
            Ellipsize::End => layout.set_ellipsize(pango::EllipsizeMode::End),
        };
    }
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Rgb {
        Rgb { r, g, b }
    }
}

impl Styling {
    fn layouter(&self, pango_context: &pango::Context) -> Layout {
        if self.vertical {
            pango_context.set_base_gravity(pango::Gravity::East);
        } else {
            pango_context.set_base_gravity(pango::Gravity::South);
        }
        self.read_direction.set(pango_context);
        let layout = Layout::new(pango_context);

        self.wrap.set(&layout);
        self.ellipsize.set(&layout);

        if let Some(indent) = self.indent {
            layout.set_indent(indent * SCALE);
        }
        if let Some(spacing) = self.spacing {
            layout.set_spacing(spacing * SCALE);
        }
        if let Some(line_spacing) = self.line_spacing {
            layout.set_line_spacing(line_spacing);
        }

        layout.set_justify(self.justiy);
        layout.set_justify_last_line(self.justify_last_line);
        if let Some(font) = &self.font {
            let mut fd = FontDescription::new();
            fd.set_family(&font.font_family);
            fd.set_stretch(font.stretch.to_pango());
            fd.set_variant(font.variant.to_pango());
            fd.set_weight(font.weight.to_pango());
            fd.set_style(font.style.to_pango());
            fd.set_size(12 * SCALE);
            layout.set_font_description(Some(&fd));
        }

        layout.set_single_paragraph_mode(self.single_paragraph_mode);
        layout.set_auto_dir(self.auto_dir);

        //set_attributes
        layout
    }
}

impl FontVariant {
    fn to_pango(&self) -> Variant {
        match self {
            Self::Normal => Variant::Normal,
            Self::SmallCaps => Variant::SmallCaps,
            Self::AllSmallCaps => Variant::AllSmallCaps,
            Self::PetiteCaps => Variant::PetiteCaps,
            Self::AllPetiteCaps => Variant::AllPetiteCaps,
            Self::Unicase => Variant::Unicase,
            Self::TitleCaps => Variant::TitleCaps,
        }
    }
}

impl FontStretch {
    fn to_pango(&self) -> Stretch {
        match self {
            FontStretch::UltraCondensed => Stretch::UltraCondensed,
            FontStretch::ExtraCondensed => Stretch::ExtraCondensed,
            FontStretch::Condensed => Stretch::Condensed,
            FontStretch::SemiCondensed => Stretch::SemiCondensed,
            FontStretch::Normal => Stretch::Normal,
            FontStretch::SemiExpanded => Stretch::SemiExpanded,
            FontStretch::Expanded => Stretch::Expanded,
            FontStretch::ExtraExpanded => Stretch::ExtraExpanded,
            FontStretch::UltraExpanded => Stretch::UltraExpanded,
        }
    }
}

impl FontWeight {
    fn to_pango(&self) -> Weight {
        match self {
            FontWeight::Thin => Weight::Thin,
            FontWeight::Ultralight => Weight::Ultralight,
            FontWeight::Light => Weight::Light,
            FontWeight::Semilight => Weight::Semilight,
            FontWeight::Book => Weight::Book,
            FontWeight::Normal => Weight::Normal,
            FontWeight::Medium => Weight::Medium,
            FontWeight::Semibold => Weight::Semibold,
            FontWeight::Bold => Weight::Bold,
            FontWeight::Ultrabold => Weight::Ultrabold,
            FontWeight::Heavy => Weight::Heavy,
            FontWeight::Ultraheavy => Weight::Ultraheavy,
        }
    }
}

impl FontStyle {
    fn to_pango(&self) -> Style {
        match self {
            Self::Normal => Style::Normal,
            Self::Italic => Style::Italic,
            Self::Oblique => Style::Oblique,
        }
    }
}

impl Text {
    fn set_font_size(&self, context: &Context, layout: &Layout) {
        if let Some(font) = layout.font_description() {
            let mut font = font;
            font.set_size(self.font_size as i32 * SCALE);
            layout.set_font_description(Some(&font));
        }
        context.set_source_rgb(self.font_color.r, self.font_color.g, self.font_color.b);
    }
    fn set(&self, layout: &Layout, context: &Context, vertical: bool) {
        match self.mode {
            Mode::Text => layout.set_text(&self.value),
            Mode::Markup => layout.set_markup(&self.value),
            Mode::MarkupWithAccel => {
                layout.set_markup_with_accel(&self.value, 'c');
            }
        }
        if vertical {
            context.translate(
                self.pos.x + self.size.width / 2.,
                self.pos.y + self.size.height / 2.,
            );
            context.rotate(90. * (PI as f64 / 180.));
            context.translate(
                -self.size.height / 2. - self.pos.x,
                -self.size.width / 2. - self.pos.y,
            );
            layout.set_width(self.size.height.ceil() as i32 * SCALE);
            layout.set_height(self.size.width.ceil() as i32 * SCALE);
        } else {
            layout.set_width(self.size.width.ceil() as i32 * SCALE);
            layout.set_height(self.size.height.ceil() as i32 * SCALE);
        }
    }

    fn set_stroke(&self, layout: &Layout, context: &Context) -> Result<(), cairo_rs::Error> {
        context.set_line_join(LineJoin::Round);
        context.set_source_rgba(
            self.outline_color.r,
            self.outline_color.g,
            self.outline_color.b,
            self.outline_color.a,
        );

        layout_path(context, layout);
        context.set_line_width(self.font_stroke);
        context.stroke()?;
        Ok(())
    }

    fn align(&self, layout: &Layout, context: &Context, vertical: bool, align: &Alignments) {
        align
            .ha
            .set(layout, context, &self.pos, &self.size, vertical);
        align
            .va
            .set(layout, context, &self.pos, &self.size, vertical);
        pangocairo::show_layout(context, layout);
    }
}

impl Painter {
    fn new(
        output_mode: OutputMode,
        image: &Option<Vec<u8>>,
        width: f64,
        height: f64,
    ) -> Result<Painter, cairo_rs::Error> {
        let mut img = image.as_ref().map(Cursor::new);

        //TODO: replace expect
        let image_surface = img
            .as_mut()
            .map(ImageSurface::create_from_png)
            .map(|surface| surface.expect("Failed to load image"));
        let img_some = image_surface.is_some();

        let surface = Painter::new_surface(&output_mode, image_surface.clone(), width, height)?;
        let context = Painter::new_context(&surface)?;

        let context2 = Painter::new_context(&surface)?;

        let context3 = Painter::new_context(&surface)?;
        let context4 = Painter::new_context(&surface)?;

        if (OutputMode::Png(true) != output_mode || OutputMode::Png(false) != output_mode)
            && img_some
        {
            context.set_source_surface(&image_surface.unwrap(), 0.0, 0.0)?;
            context.paint()?;
        }
        let pc = create_context(&context3);
        let pc2 = create_context(&context4);
        Ok(Painter {
            surface,
            _context1: context,
            context2,
            context3,
            context4,
            pango_context1: pc,
            pango_context2: pc2,
        })
    }

    fn new_surface(
        output_mode: &OutputMode,
        image_surface: Option<ImageSurface>,
        width: f64,
        height: f64,
    ) -> Result<Surfaces, cairo_rs::Error> {
        Ok(match output_mode {
            OutputMode::Png(_) => Surfaces::Png(match image_surface {
                Some(v) => v,
                None => {
                    ImageSurface::create(Format::ARgb32, width.ceil() as i32, height.ceil() as i32)?
                }
            }),
            OutputMode::Svg => Surfaces::Svg(SvgSurface::for_stream(width, height, vec![])?),
            OutputMode::Ps => Surfaces::Ps(PsSurface::for_stream(width, height, vec![])?),
            _ => Surfaces::Pdf(PdfSurface::for_stream(width, height, vec![])?),
        })
    }

    fn new_context(surface: &Surfaces) -> Result<Context, cairo_rs::Error> {
        Ok(match &surface {
            Surfaces::Pdf(v) => Context::new(v)?,
            Surfaces::Png(v) => Context::new(v)?,
            Surfaces::Svg(v) => Context::new(v)?,
            Surfaces::Ps(v) => Context::new(v)?,
        })
    }

    pub fn export<W: Write>(&self, writer: &mut W) -> Result<(), String> {
        match self.surface {
            Surfaces::Pdf(ref pdf_surface) => {
                let v: Vec<u8> = *pdf_surface
                    .finish_output_stream()
                    .map_err(|e| e.to_string())?
                    .downcast()
                    .map_err(|_| "Failed to downcast".to_string())?;
                writer.write_all(&v).map_err(|e| e.to_string())?;
            }
            Surfaces::Png(ref image_surface) => {
                image_surface
                    .write_to_png(writer)
                    .map_err(|e| e.to_string())?;
            }
            Surfaces::Svg(ref svg_surface) => {
                let v: Vec<u8> = *svg_surface
                    .finish_output_stream()
                    .map_err(|e| e.to_string())?
                    .downcast()
                    .map_err(|_| "Failed to downcast".to_string())?;
                writer.write_all(&v).map_err(|e| e.to_string())?;
            }
            Surfaces::Ps(ref ps_surface) => {
                let v: Vec<u8> = *ps_surface
                    .finish_output_stream()
                    .map_err(|e| e.to_string())?
                    .downcast()
                    .map_err(|_| "Failed to downcast".to_string())?;
                writer.write_all(&v).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

impl ReadDirection {
    fn set(&self, context: &pango::Context) {
        context.set_base_dir(match self {
            ReadDirection::LR => Direction::Ltr,
            ReadDirection::RL => Direction::Rtl,
            ReadDirection::WeakLR => Direction::WeakLtr,
            ReadDirection::WeakRL => Direction::WeakRtl,
        })
    }
}

impl Size2 {
    pub fn new(width: f64, height: f64) -> Size2 {
        Size2 { width, height }
    }
}

impl Rgba {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Rgba {
        Rgba { r, g, b, a }
    }
}

impl Pos2 {
    pub fn new(x: f64, y: f64) -> Pos2 {
        Pos2 { x, y }
    }
}

impl Data {
    pub fn painter(
        self,
        output_mode: OutputMode,
        width: f64,
        height: f64,
    ) -> Result<Painter, cairo_rs::Error> {
        let painter = Painter::new(output_mode, &self.background, width, height)?;
        let mut reload = true;
        let context = &painter.context2;
        for item in &self.items {
            match &item.background {
                Background::Bytes(image) => {
                    //TODO: replace expect
                    let surface = ImageSurface::create_from_png(&mut Cursor::new(&image))
                        .expect("Failed to create image surface");
                    context.set_source_surface(surface, item.pos.x, item.pos.y)?;
                    context.rectangle(item.pos.x, item.pos.y, item.size.width, item.size.height);
                    context.fill()?;
                }
                Background::Rgb(color) => {
                    context.set_source_rgb(color.r, color.g, color.b);
                    context.rectangle(item.pos.x, item.pos.y, item.size.width, item.size.height);
                    context.fill()?;
                }
                Background::None => {}
            };
        }
        let mut layout1 = self.global_style.layouter(&painter.pango_context1);
        let mut layout2 = self.global_style.layouter(&painter.pango_context2);
        for item in self.items {
            let vertical: bool;
            match &item.style {
                None => {
                    vertical = self.global_style.vertical;
                    if reload {
                        layout1 = self.global_style.layouter(&painter.pango_context1);
                        layout2 = self.global_style.layouter(&painter.pango_context2);
                        reload = false;
                    }
                }
                Some(style) => {
                    vertical = style.vertical;
                    layout1 = style.layouter(&painter.pango_context1);
                    layout2 = style.layouter(&painter.pango_context2);
                    reload = true;
                }
            };
            let align = item.align.as_ref().unwrap_or(&self.global_align);
            item.set(&layout1, &painter.context3, vertical);
            item.set(&layout2, &painter.context4, vertical);
            item.set_font_size(&painter.context3, &layout1);
            item.set_font_size(&painter.context4, &layout2);
            item.align(&layout1, &painter.context3, vertical, align);
            item.set_stroke(&layout1, &painter.context3)?;
            item.align(&layout2, &painter.context4, vertical, align);
        }

        Ok(painter)
    }
}
