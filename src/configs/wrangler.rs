use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct WranglerConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub v1_symbol: &'a str,
    // pub v2_symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for WranglerConfig<'a> {
    fn default() -> Self {
        WranglerConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "⛅️ ",
            v1_symbol: "🤠 ",
            // v2_symbol: "⛅️ ",
            style: "bold orange",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["package.json", "wrangler.toml"],
            detect_folders: vec![],
        }
    }
}
