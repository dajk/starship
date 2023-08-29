use regex::Regex;

use super::{Context, Module, ModuleConfig};

use crate::configs::wrangler::WranglerConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current wrangler version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("wrangler");
    let config = WranglerConfig::try_load(module.config);

    let is_wrangler_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_wrangler_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        let wrangler_version =
            parse_wrangler_version(&context.exec_cmd("wrangler", &["--version"]).unwrap().stdout)
                .unwrap();

        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(if wrangler_version.starts_with("1.") {
                    config.v1_symbol
                } else {
                    config.symbol
                }),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => VersionFormatter::format_module_version(
                    module.get_name(),
                    &wrangler_version,
                    config.version_format,
                )
                .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `wrangler`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_wrangler_version(wrangler_version: &str) -> Option<String> {
    if Regex::new(r"wrangler \d+\.\d+\.\d+")
        .unwrap()
        .is_match(wrangler_version)
    {
        // split into ["wrangler", "(1|2|3).x.x", ...]
        let mut splitted = wrangler_version.split_whitespace();
        let _ = splitted.position(|t| t == "wrangler");
        // return "(1|2|3).x.x"
        let version = splitted.next()?;

        return Some(version.to_string());
    }

    if Regex::new(r"\d+\.\d+\.\d+")
        .unwrap()
        .is_match(wrangler_version)
    {
        return Some(wrangler_version.to_string());
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_wrangler_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("wrangler").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_wrangler_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("wrangler.toml"))?.sync_all()?;
        let actual = ModuleRenderer::new("wrangler").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Rgb(246, 130, 31).bold().paint("⛅️ v2.4.0 "),
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn wrangler_as_dependency() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("wrangler").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_wrangler_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("wrangler.toml"))?.sync_all()?;
        let actual = ModuleRenderer::new("wrangler")
            .path(dir.path())
            .cmd("wrangler --version", None)
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Rgb(246, 130, 31).bold().paint("⛅️ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
