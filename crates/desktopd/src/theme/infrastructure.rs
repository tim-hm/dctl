use core::result::Result;
use gio::{traits::SettingsExt, Settings as GSettings};
use log::error;

use super::error::ThemeError;

const SCHEMA_ID: &str = "org.gnome.desktop.interface";

pub struct ThemeSettings {
    settings: GSettings,
}

impl ThemeSettings {
    pub fn new() -> Self {
        Self {
            settings: GSettings::new(SCHEMA_ID),
        }
    }

    pub fn read(&self, key: &str) -> Result<String, ThemeError> {
        let value = self.settings.string(key);
        // TODO There's no error handling in GSettings, so I'm not sure what happens if this call fails
        Ok(value.to_string())
    }

    pub fn write(&self, key: &str, value: &str) -> Result<(), ThemeError> {
        match self.settings.set_string(key, value) {
            Ok(value) => Ok(()),
            Err(error) => {
                error!("{error}");
                Err(ThemeError::GSettingsWriteError {
                    key: key.to_string(),
                    value: value.to_string(),
                })
            }
        }
    }
}
