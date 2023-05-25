use super::{
    error::ThemeError,
    infrastructure::ThemeSettings,
    value_objects::{Accent, Style, Theme},
};
use core::result::Result;
use log::debug;
use std::str::FromStr;

const ACCENT_KEY: &str = "gtk-theme";
const STYLE_KEY: &str = "color-scheme";

pub struct ThemeManager {
    settings: ThemeSettings,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self {
            settings: ThemeSettings::new(),
        }
    }

    pub fn set(&self, style: Style) -> Result<(), ThemeError> {
        debug!("Set theme called with {:?}", style);

        let accent = self.read_accent()?;
        let theme = Theme { style, accent };

        self.set_theme(theme)
    }

    pub fn toggle(&self) -> Result<Theme, ThemeError> {
        debug!("Toggling theme");

        let theme = self.read_theme()?;
        let next_style = theme.style.toggle();
        let theme = Theme {
            style: next_style,
            accent: theme.accent,
        };

        self.set_theme(theme)?;

        Ok(theme)
    }

    fn read_theme(&self) -> Result<Theme, ThemeError> {
        let style = self.read_style()?;
        let accent = self.read_accent()?;
        let theme = Theme { style, accent };

        Ok(theme)
    }

    fn read_style(&self) -> Result<Style, ThemeError> {
        let raw = self.settings.read(STYLE_KEY)?;

        match Style::from_str(&raw) {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }

    fn read_accent(&self) -> Result<Accent, ThemeError> {
        let s = self.settings.read(ACCENT_KEY)?;

        Accent::from_str(s.as_str())
    }

    fn set_theme(&self, theme: Theme) -> Result<(), ThemeError> {
        self.set_style(theme)?;
        self.set_accent(theme)?;

        Ok(())
    }

    fn set_accent(&self, theme: Theme) -> Result<(), ThemeError> {
        let is_dark = if let Style::Dark = theme.style {
            true
        } else {
            false
        };

        let value = theme.accent.to_dconf_value(is_dark);
        self.settings.write(ACCENT_KEY, value.as_str())?;

        Ok(())
    }

    fn set_style(&self, theme: Theme) -> Result<(), ThemeError> {
        let value = theme.style.to_dconf_value();
        self.settings.write(STYLE_KEY, value)
    }
}
