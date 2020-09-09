use std::ops::Range;

use crate::piet::{FontFamily, FontWeight, Color, FontStyle};
use crate::{KeyOrValue, FontDescriptor};

struct AttributeSpans {
    family: SpanSet<FontFamily>,
    size: SpanSet<KeyOrValue<f64>>,
    weight: SpanSet<FontWeight>,
    fg_color: SpanSet<KeyOrValue<Color>>,
    style: SpanSet<FontStyle>,
    underline: SpanSet<bool>,
    font_descriptor: SpanSet<KeyOrValue<FontDescriptor>>,
}

struct SpanSet<T> {
    spans: Vec<Span<T>>,
}

#[derive(Debug, Clone)]
struct Span<T> {
    range: Range<usize>,
    attr: T,
}

/// Attributes that can be applied to text.
#[derive(Debug, Clone)]
enum Attribute {
    /// The font family.
    FontFamily(FontFamily),
    /// The font size, in points.
    FontSize(KeyOrValue<f64>),
    /// The [`FontWeight`](struct.FontWeight.html).
    Weight(FontWeight),
    /// The foreground color of the text.
    ForegroundColor(KeyOrValue<Color>),
    /// The [`FontStyle`]; either regular or italic.
    ///
    /// [`FontStyle`]: enum.FontStyle.html
    Style(FontStyle),
    /// Underline.
    Underline(bool),
    Descriptor(KeyOrValue<FontDescriptor>),
}

impl AttributeSpans {
    pub fn add(&mut self, range: Range<usize>, attr: Attribute) {
        match attr {
            Attribute::FontFamily(attr) => self.family.add(Span::new(range, attr)),
            Attribute::FontSize(attr) => self.size.add(Span::new(range, attr)),
            Attribute::Weight(attr) => self.weight.add(Span::new(range, attr)),
            Attribute::ForegroundColor(attr) => self.fg_color.add(Span::new(range, attr)),
            Attribute::Style(attr) => self.style.add(Span::new(range, attr)),
            Attribute::Underline(attr) => self.underline.add(Span::new(range, attr)),
            Attribute::Descriptor(attr) => self.font_descriptor.add(Span::new(range, attr)),
        }
    }
}

impl<T> SpanSet<T> {
    fn add(&mut self, span: Span<T>) {
        // ignore all spans preceding the new span:
        //let split_point = self.spans.
        for existing in &mut self.spans {
            if existing.range.end <= span.range.start {
                continue;
            }
        }

    }

}

impl<T> Span<T> {
    fn new(range: Range<usize>, attr: T) -> Self {
        Span {
            range, attr
        }
    }
}


impl Attribute {
    pub fn size(size: impl Into<KeyOrValue<f64>>) -> Self {
        Attribute::FontSize(size.into())
    }

    pub fn foregound_color(color: impl Into<KeyOrValue<Color>>) -> Self {
        Attribute::ForegroundColor(color.into())
    }

    pub fn font_family(family: FontFamily) -> Self {
        Attribute::FontFamily(family)
    }

    pub fn weight(weight: FontWeight) -> Self {
        Attribute::Weight(weight)
    }

    pub fn style(style: FontStyle) -> Self {
        Attribute::Style(style)
    }

    pub fn underline(underline: bool) -> Self {
        Attribute::Underline(underline)
    }

    pub fn font_descriptor(font: impl Into<KeyOrValue<FontDescriptor>>) -> Self {
        Attribute::Descriptor(font.into())
    }

    /// When applying attributes with equal start positions, we prefer to add
    /// some before others.
    ///
    /// A lower sort order is applied first.
    ///
    /// Realistically this is just `FontDescriptor`, since we want to let other
    /// attributes override its constituent parts. But why not be systematic..
    fn sort_order(&self) -> usize {
        match self {
            Attribute::Descriptor(_) => 0,
            _ => 1,
        }
    }
}

//impl Span {
    //fn sort_order(&self, other: &Span)
//}


//impl From<FontFamily> for Attribute {

    //fn from(src: FontFamily) -> Attribute {
        //Attribute::FontFamily(src)
    //}
//}

//impl From<FontWeight> for Attribute {
    //fn from(src: FontWeight) -> Attribute {
        //Attribute::ForegroundColor(src)
    //}
//}

//impl From<KeFontWeight> for Attribute {
    //fn from(src: FontWeight) -> Attribute {
        //Attribute::ForegroundColor(src)
    //}
//}

//impl From<Color> for Attribute {
    //fn from(src: Color) -> Attribute {
        //Attribute::Weight(src)
    //}
//}

//impl<T: Into<KeyOrValue<Color>>> From<T> for Attribute {
    //fn from(src: T) -> Attribute {
        //Attribute::ForegroundColor(src.into())
    //}
//}

//impl From<FontStyle> for Attribute {
    //fn from(src: FontStyle) -> Attribute {
        //Attribute::Style(src)
    //}
//}


//impl<T: Into<KeyOrValue<FontDescriptor>>> From<T> for Attribute {
    //fn from(src: T) -> Attribute {
        //Attribute::Descriptor(src.into())
