use std::{fmt, str::FromStr};

use super::error::ThemeError;

const STYLE_DEFAULT: &str = "default";
const STYLE_DARK: &str = "prefer-dark";
const STYLE_LIGHT: &str = "prefer-light";

#[derive(Debug, Copy, Clone)]
pub struct Theme {
    pub style: Style,
    pub accent: Accent,
}

#[derive(Debug, Copy, Clone)]
pub enum Style {
    Default,
    Dark,
    // What's the difference is between default and prefer-light?
    Light,
}

impl Style {
    pub fn toggle(&self) -> Self {
        match self {
            Style::Dark => Style::Default,
            Style::Default => Style::Dark,
            Style::Light => Style::Dark,
        }
    }

    pub fn to_dconf_value(&self) -> &'static str {
        match self {
            Style::Default => STYLE_DEFAULT,
            Style::Dark => STYLE_DARK,
            Style::Light => STYLE_LIGHT,
        }
    }
}

impl FromStr for Style {
    type Err = ThemeError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value = s.to_lowercase();
        let value = value.as_str();

        match value {
            STYLE_DEFAULT => Ok(Style::Default),
            STYLE_LIGHT => Ok(Style::Light),
            STYLE_DARK => Ok(Style::Dark),
            _ => Err(ThemeError::UnknownStyleValue(s.to_string())),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Accent {
    Orange,
    Bark,
    Sage,
    Olive,
    Viridian,
    Green,
    Blue,
    Purple,
    Magenta,
    Red,
}

impl Accent {
    pub fn to_dconf_value(&self, is_dark: bool) -> String {
        if is_dark {
            format!("{}-dark", self)
        } else {
            format!("{}", self)
        }
    }
}

impl fmt::Display for Accent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self {
            Accent::Orange => String::from("Yaru"),
            Accent::Bark => String::from("Yaru-bark"),
            Accent::Sage => String::from("Yaru-sage"),
            Accent::Olive => String::from("Yaru-olive"),
            Accent::Viridian => String::from("Yaru-viridian"),
            Accent::Green => String::from("Yaru-green"),
            Accent::Blue => String::from("Yaru-blue"),
            Accent::Purple => String::from("Yaru-purple"),
            Accent::Magenta => String::from("Yaru-magenta"),
            Accent::Red => String::from("Yaru-red"),
        };
        write!(f, "{}", value)
    }
}

impl FromStr for Accent {
    type Err = ThemeError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let value = s.to_lowercase();

        if value.contains("orange") {
            Ok(Accent::Orange)
        } else if value.contains("bark") {
            Ok(Accent::Bark)
        } else if value.contains("sage") {
            Ok(Accent::Sage)
        } else if value.contains("olive") {
            Ok(Accent::Olive)
        } else if value.contains("viridian") {
            Ok(Accent::Viridian)
        } else if value.contains("green") {
            Ok(Accent::Green)
        } else if value.contains("blue") {
            Ok(Accent::Blue)
        } else if value.contains("purple") {
            Ok(Accent::Purple)
        } else if value.contains("magenta") {
            Ok(Accent::Magenta)
        } else if value.contains("red") {
            Ok(Accent::Red)
        } else {
            Err(ThemeError::UnknownAccentValue(s.to_string()))
        }
    }
}
