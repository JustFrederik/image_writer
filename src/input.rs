use image::codecs::pnm::PnmSubtype;
/// Main Item that contains the data
pub struct Data {
    /// Text block
    pub items: Vec<Text>,
    /// Style to fallback to when nothing is specified in Text
    pub global_style: Styling,
    /// The alignment of the text within the box when not specified in Text
    pub global_align: Alignments,
    /// The background image as png in bytes
    pub background: Option<Vec<u8>>,
}

/// Alignment of the text within the box
pub struct Alignments {
    /// Horizontal alignment
    /// left -- center -- right
    pub ha: HorizontalAlignment,
    /// Vertical alignment
    /// top -- center -- bottom
    pub va: VerticalAlignment,
}

/// Text section
pub struct Text {
    /// Different inputs like String or Markdown
    pub mode: Mode,
    /// Text in textblock
    pub value: String,
    /// upper left corner of textblock
    pub pos: Pos2,
    /// width and height of textblock
    pub size: Size2,
    /// fontsize
    pub font_size: f64,
    /// fontcolor
    pub font_color: Rgb,
    /// Background of text section
    pub background: Background,
    /// Alternative style for text section
    pub style: Option<Styling>,
    /// Alternative alignment for text section
    pub align: Option<Alignments>,
    /// the color of the outline
    pub outline_color: Rgba,
    /// The thickness of the outline/stroke/shadow...
    pub font_stroke: f64,
}

/// background of text box
pub enum Background {
    /// bytes of image
    Bytes(Vec<u8>),
    /// image color in rgb format from 0 to 255
    Rgb(Rgb),
    /// Do nothing
    None,
}

/// Text input type
pub enum Mode {
    /// Plain
    Text,
    /// Markup format
    Markup,
    /// TODO: idk what this does
    MarkupWithAccel,
}

#[derive(Default)]
/// Read direction of text
pub enum ReadDirection {
    /// A strong left-to-right direction.
    LR,
    /// A strong right-to-left direction.
    RL,
    /// A weak left-to-right direction.
    #[default]
    WeakLR,
    /// A weak right-to-left direction.
    WeakRL,
}

pub struct Styling {
    /// Sets the amount of spacing between the lines of the layout.
    pub spacing: Option<i32>,
    /// Sets a factor for line spacing.
    pub line_spacing: Option<f32>,
    /// Sets the type of Ellipsization being performed for layout.
    /// Text shortening with ...
    pub ellipsize: Ellipsize,
    /// Sets the wrap mode.
    pub wrap: Wrap,
    /// Sets the width to indent each paragraph.
    /// css: text-indent
    pub indent: Option<i32>,
    /// Sets the single paragraph mode of layout.
    pub single_paragraph_mode: bool,
    /// Sets whether to calculate the base direction for the layout according to its contents.
    pub auto_dir: bool,
    pub read_direction: ReadDirection,
    pub vertical: bool,
    pub font: Option<Font>,
    ///  Sets whether each complete line should be stretched to fill the entire width of the layout.
    pub justiy: bool,
    /// Sets whether the last line should be stretched to fill the entire width of the layout.
    pub justify_last_line: bool,
}

#[derive(Default)]
/// Shortening of text when overflowing with ...
pub enum Ellipsize {
    /// No Ellipsization
    #[default]
    None,
    /// Ellipsization is at the start of the text
    Start,
    /// Ellipsization is in the middle of the text
    Middle,
    /// Ellipsization is at the end of the text
    End,
}

/// Color with alpha chanel
pub struct Rgba {
    /// red 0-255
    pub r: f64,
    /// green 0-255
    pub g: f64,
    /// blue 0-255
    pub b: f64,
    /// alpha 0-1
    pub a: f64,
}

/// Color without alpha chanel
pub struct Rgb {
    /// red 0-255
    pub r: f64,
    /// green 0-255
    pub g: f64,
    /// blue 0-255
    pub b: f64,
}

/// Position in pixels
pub struct Pos2 {
    /// x position in px
    pub x: f64,
    /// y position in px
    pub y: f64,
}

/// where to put - to fit box
pub enum Wrap {
    /// Wrap lines at word boundaries.
    Word,
    /// Wrap lines at character boundaries.
    Char,
    /// Wrap lines at word boundaries, but fall back to character boundaries if there is not enough space for a full word.
    WordChar,
}

/// Alignment of text
pub enum HorizontalAlignment {
    /// Left side
    Left,
    /// Right side
    Right,
    /// center
    Center,
}

/// Alignment of text
pub enum VerticalAlignment {
    /// top
    Top,
    /// center
    Center,
    /// bottom
    Bottom,
}

#[derive(PartialEq, Eq)]
/// Output format
pub enum OutputMode {
    /// Portable Document Format
    /// bool: if bytes should be returned instead of creating file
    Pdf(bool),
    /// Portable Network Graphics
    /// bool: lossless compression
    Png(bool),
    /// Scalable Vector Graphics
    Svg,
    /// PostScript
    Ps,
    /// An Image in JPEG Format with specified quality, up to 100
    Jpeg(u8),
    /// An Image in one of the PNM Formats
    Pnm(PnmSubtype),
    /// An Image in ICO Format
    Ico,
    /// An Image in BMP Format
    Bmp,
    /// An Image in farbfeld Format
    Farbfeld,
    /// An Image in TGA Format
    Tga,
    /// An Image in OpenEXR Format
    OpenExr,
    /// An Image in TIFF Format
    Tiff,
    /// An image in AVIF Format
    Avif,
    /// An image in QOI Format
    Qoi,
    /// An image in WebP Format.
    WebP,
}

/// font configuration
pub struct Font {
    /// font
    pub font_family: String,
    /// caps
    pub variant: FontVariant,
    /// char spacing
    pub stretch: FontStretch,
    /// bold, normal, light
    pub weight: FontWeight,
    /// italic, oblique, normal
    pub style: FontStyle,
}

#[derive(Default)]
pub enum FontStyle {
    #[default]
    /// The font is upright.
    Normal,
    /// The font is slanted in an italic style.
    Italic,
    /// The font is slanted, but in a roman style.
    Oblique,
}

#[derive(Default)]
pub enum FontVariant {
    #[default]
    /// A normal font.
    Normal,
    /// A font with the lower case characters replaced by smaller variants of the capital characters.
    SmallCaps,
    /// A font with all characters replaced by smaller variants of the capital characters.
    AllSmallCaps,
    /// A font with the lower case characters replaced by smaller variants of the capital characters. Petite Caps can be even smaller than Small Caps.
    PetiteCaps,
    /// A font with all characters replaced by smaller variants of the capital characters. Petite Caps can be even smaller than Small Caps.
    AllPetiteCaps,
    /// A font with the upper case characters replaced by smaller variants of the capital letters.
    Unicase,
    /// A font with capital letters that are more suitable for all-uppercase titles.
    TitleCaps,
}

#[derive(Default)]
pub enum FontStretch {
    /// Ultra condensed width.
    UltraCondensed,
    /// Extra condensed width.
    ExtraCondensed,
    /// Condensed width.
    Condensed,
    /// Semi condensed width.
    SemiCondensed,
    /// The normal width.
    #[default]
    Normal,
    /// Semi expanded width.
    SemiExpanded,
    /// Expanded width.
    Expanded,
    /// Extra expanded width.
    ExtraExpanded,
    /// Ultra expanded width.
    UltraExpanded,
}

#[derive(Default)]
pub enum FontWeight {
    /// The thin weight (= 100)
    Thin,
    /// The ultralight weight (= 200)
    Ultralight,
    /// The light weight (= 300)
    Light,
    /// The semilight weight (= 350)
    Semilight,
    /// The book weight (= 380)
    Book,
    /// The default weight (= 400)
    #[default]
    Normal,
    /// The medium weight (= 500)
    Medium,
    /// The semibold weight (= 600)
    Semibold,
    /// The bold weight (= 700)
    Bold,
    /// The ultrabold weight (= 800)
    Ultrabold,
    /// The heavy weight (= 900)
    Heavy,
    /// The ultraheavy weight (= 1000)
    Ultraheavy,
}

/// Size in pixels
pub struct Size2 {
    /// width in px
    pub width: f64,
    /// height in px
    pub height: f64,
}
