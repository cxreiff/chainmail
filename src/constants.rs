use ratatui::{style::Color, symbols::border};

pub const _PLASTIC_EMPHASIS_COLOR: Color = Color::from_u32(0x00E3E1C9);
pub const PLASTIC_PRIMARY_COLOR: Color = Color::from_u32(0x00EEF1D8);
pub const PLASTIC_SECONDARY_COLOR: Color = Color::from_u32(0x00C5BEBE);
pub const PLASTIC_LIGHT_BACKGROUND_COLOR: Color = Color::from_u32(0x00384B53);
pub const PLASTIC_MEDIUM_BACKGROUND_COLOR: Color = Color::from_u32(0x00282c31);
pub const PLASTIC_DARK_BACKGROUND_COLOR: Color = Color::from_u32(0x00151515);
pub const _PLASTIC_BLACK_BACKGROUND_COLOR: Color = Color::from_u32(0x00000000);

pub const MAC_GREEN_COLOR: Color = Color::from_u32(0x0075BD21);
pub const _MAC_YELLOW_COLOR: Color = Color::from_u32(0x00FFC728);
pub const _MAC_ORANGE_COLOR: Color = Color::from_u32(0x00FF661C);
pub const MAC_RED_COLOR: Color = Color::from_u32(0x00CF0F2B);
pub const _MAC_PURPLE_COLOR: Color = Color::from_u32(0x00B01CAB);
pub const _MAC_CYAN_COLOR: Color = Color::from_u32(0x0000A1DE);

pub const _MAC_GREEN_MUTED_COLOR: Color = Color::from_u32(0x00496844);
pub const _MAC_YELLOW_MUTED_COLOR: Color = Color::from_u32(0x00756A45);
pub const _MAC_ORANGE_MUTED_COLOR: Color = Color::from_u32(0x00754C3F);
pub const MAC_RED_MUTED_COLOR: Color = Color::from_u32(0x00643842);
pub const MAC_PURPLE_MUTED_COLOR: Color = Color::from_u32(0x00594362);
pub const _MAC_CYAN_MUTED_COLOR: Color = Color::from_u32(0x00385670);

pub const LETTER_PADDING: u16 = 1;

pub const CUSTOM_BORDERS: border::Set = border::Set {
    vertical_left: border::DOUBLE.vertical_left,
    vertical_right: border::DOUBLE.vertical_right,
    horizontal_top: border::QUADRANT_OUTSIDE.horizontal_top,
    horizontal_bottom: border::QUADRANT_OUTSIDE.horizontal_bottom,
    top_left: border::FULL.top_left,
    top_right: border::FULL.top_right,
    bottom_left: border::FULL.bottom_left,
    bottom_right: border::FULL.bottom_right,
};

// Reveal timing constants (in milliseconds)
pub const TITLE_REVEAL_TIME: u32 = 500;
pub const BODY_REVEAL_TIME: u32 = 5;
pub const HEADER_REVEAL_TIME: u32 = 400;
pub const BLESSING_REVEAL_TIME: u32 = 400;
pub const CURSE_REVEAL_TIME: u32 = 400;
pub const SIGNOFF_REVEAL_TIME: u32 = 5;
pub const FOOTER_REVEAL_TIME: u32 = 600;

// Margin of delay after each reveal section (in milliseconds).
pub const REVEAL_TIME_MARGIN: u32 = 400;
