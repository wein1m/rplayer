use ratatui::style::Color;

pub struct Colors;

impl Colors {
    pub const BORDER: Color = Color::Rgb(125, 122, 129);
    pub const BG_HOVER: Color = Color::Rgb(182, 215, 221);
    pub const TEXT: Color = Color::Rgb(170, 165, 179);
    pub const TEXT_HOVER: Color = Color::Black;
}
