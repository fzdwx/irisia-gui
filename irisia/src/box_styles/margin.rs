use irisia_core::{style::Pixel, Style};

#[derive(Style, Clone, Copy)]
#[irisia(style(from, from = "", impl_default))]
pub struct StyleMargin {
    #[irisia(style(option, default))]
    pub left: Pixel,

    #[irisia(style(option, default))]
    pub top: Pixel,

    #[irisia(style(option, default))]
    pub right: Pixel,

    #[irisia(style(option, default))]
    pub bottom: Pixel,
}

impl From<(Pixel,)> for StyleMargin {
    fn from((px,): (Pixel,)) -> Self {
        Self {
            left: px,
            top: px,
            right: px,
            bottom: px,
        }
    }
}

impl From<(Pixel, Pixel)> for StyleMargin {
    fn from((x, y): (Pixel, Pixel)) -> Self {
        Self {
            left: x,
            top: y,
            right: x,
            bottom: y,
        }
    }
}
