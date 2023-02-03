use std::fmt::{self, Display};

use serde::Deserialize;
use serde_json::Result;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Voice {
    display_name: String,
    gender: String,
    local_name: String,
    locale: String,
    locale_name: String,
    name: String,
    sample_rate_hertz: String,
    short_name: String,
    status: String,
    voice_type: String,
    words_per_minute: Option<String>,
    style_list: Option<Vec<String>>,
    role_play_list: Option<Vec<String>>,
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "Display name: {}", self.display_name)?;
        writeln!(f, "Local name: {} @ {}", self.local_name, self.locale)?;
        writeln!(f, "Locale: {}", self.locale)?;
        writeln!(f, "Gender: {}", self.gender)?;
        writeln!(f, "ID: {}", self.short_name)?;
        writeln!(f, "Voice type: {}", self.voice_type)?;
        writeln!(f, "Status: {}", self.status)?;
        writeln!(f, "Sample rate: {}Hz", self.sample_rate_hertz)?;
        writeln!(
            f,
            "Words per minute: {}",
            self.words_per_minute.as_deref().unwrap_or("N/A")
        )?;
        if let Some(style_list) = self.style_list.as_ref() {
            writeln!(f, "Styles: {:?}", style_list)?;
        }
        if let Some(role_play_list) = self.role_play_list.as_ref() {
            writeln!(f, "Roles: {:?}", role_play_list)?;
        }
        Ok(())
    }
}
