use bevy::prelude::*;

mod plugin;

pub use plugin::ThemePlugin;

#[derive(Debug, Default, Reflect, Clone)]
pub enum TextTheme {
    Subtle,
    #[default]
    Regular,
    Heavy,
    Primary,
    Secondary,
}

impl TextTheme {
    pub fn get_style<'a>(&self, theme: &'a UiTheme) -> &'a TextStyle {
        match self {
            TextTheme::Subtle => &theme.text_subtle,
            TextTheme::Regular => &theme.text_regular,
            TextTheme::Heavy => &theme.text_heavy,
            TextTheme::Primary => &theme.text_primary,
            TextTheme::Secondary => &theme.text_secondary,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub color: Color,
}

#[derive(Debug, Resource)]
pub struct UiTheme {
    pub text_subtle: TextStyle,
    pub text_regular: TextStyle,
    pub text_heavy: TextStyle,

    text_primary: TextStyle,
    pub text_secondary: TextStyle,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            text_subtle: TextStyle {
                color: Color::hsv(0.0, 0.0, 0.3),
            },
            text_regular: TextStyle {
                color: Color::hsv(0.0, 0.0, 0.6),
            },
            text_heavy: TextStyle {
                color: Color::hsv(0.0, 0.0, 1.2),
            },
            text_primary: TextStyle {
                color: Color::srgb_u8(0xff, 0x61, 0x88),
            },
            text_secondary: TextStyle {
                color: Color::srgb_u8(0xab, 0x9d, 0xf),
            },
        }
    }
}
