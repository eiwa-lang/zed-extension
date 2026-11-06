use std::path::PathBuf;

use zed_extension_api::{
    self as zed, Architecture, Command, DownloadedFileType, LanguageServerId,
    LanguageServerInstallationStatus, Os, Result, Worktree,
};

const GITHUB_REPO: &str = "eiwa-lang/eiwa";

struct EiwaExtension;

impl EiwaExtension {
    /// Release asset suffix for this platform, plus the download type and
    /// the executable extension. Mirrors the `release.yml` matrix
    /// (`linux-x86_64`, `linux-arm64`, `darwin-arm64`, `darwin-x86_64`,
    /// `windows-x86_64`; zip only on Windows).
    fn platform_target() -> Result<(&'static str, DownloadedFileType, &'static str)> {
        match zed::current_platform() {
            (Os::Mac, Architecture::Aarch64) => {
                Ok(("darwin-arm64", DownloadedFileType::GzipTar, ""))
            }
            (Os::Mac, Architecture::X8664) => {
                Ok(("darwin-x86_64", DownloadedFileType::GzipTar, ""))
            }
            (Os::Linux, Architecture::Aarch64) => {
                Ok(("linux-arm64", DownloadedFileType::GzipTar, ""))
            }
            (Os::Linux, Architecture::X8664) => {
                Ok(("linux-x86_64", DownloadedFileType::GzipTar, ""))
            }
            (Os::Windows, Architecture::X8664) => {
                Ok(("windows-x86_64", DownloadedFileType::Zip, ".exe"))
            }
            _ => Err(
                "eiwa-lsp: this platform is not covered by Eiwa releases \
                 (see https://github.com/eiwa-lang/eiwa/releases)"
                    .to_string(),
            ),
        }
    }

    /// Absolute path when the sandbox exposes a CWD, else the bare
    /// extension-relative path (Zed resolves `command` against the
    /// extension work dir; `env` values need the absolute form).
    fn absolutize(relative: &str) -> String {
        match std::env::current_dir() {
            Ok(cwd) => PathBuf::from(&cwd)
                .join(relative)
                .to_string_lossy()
                .into_owned(),
            Err(_) => relative.to_string(),
        }
    }

    /// Local install first: reuse `eiwa-lsp` from the user's `PATH`
    /// (e.g. installed via `install.sh` or built from source) instead of
    /// downloading a duplicate. Returns `(server, eiwac?)`.
    fn local_server(worktree: &Worktree) -> Option<(String, Option<String>)> {
        let server = worktree.which("eiwa-lsp")?;
        let eiwac = worktree
            .shell_env()
            .into_iter()
            .find(|(k, _)| k == "EIWAC")
            .map(|(_, v)| v)
            .or_else(|| worktree.which("eiwac"));
        Some((server, eiwac))
    }

    /// Ensures a released `eiwa-lsp` (+ `eiwac` for diagnostics) is present
    /// in the extension work dir, downloading the latest GitHub release when
    /// needed. Returns `(server_binary, eiwac_binary)`.
    fn ensure_server(&mut self, id: &LanguageServerId) -> Result<(String, String)> {
        let (target, file_type, exe) = Self::platform_target()?;
        let ext = if exe.is_empty() { "tar.gz" } else { "zip" };

        let release = zed::latest_github_release(
            GITHUB_REPO,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )
        .map_err(|e| format!("eiwa-lsp: could not fetch latest Eiwa release: {e}"))?;
        // `version` is the tag (`v0.0.106`); tolerate a title form too.
        let tag = release
            .version
            .trim()
            .split_whitespace()
            .last()
            .unwrap_or(&release.version);
        let version = tag.strip_prefix('v').unwrap_or(tag);

        let asset_name = format!("eiwa-{version}-{target}.{ext}");
        let asset = release
            .assets
            .iter()
            .find(|a| a.name == asset_name)
            .ok_or_else(|| {
                format!(
                    "eiwa-lsp: release {} has no asset {asset_name} \
                     (see https://github.com/{GITHUB_REPO}/releases)",
                    release.version
                )
            })?;

        let dir = format!("eiwa-{version}-{target}");
        let server = Self::absolutize(&format!("{dir}/bin/eiwa-lsp{exe}"));
        let eiwac = Self::absolutize(&format!("{dir}/bin/eiwac{exe}"));

        if !std::path::Path::new(&server).exists() {
            zed::set_language_server_installation_status(
                id,
                &LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(&asset.download_url, &dir, file_type)
                .map_err(|e| format!("eiwa-lsp: download failed: {e}"))?;
            if exe.is_empty() {
                zed::make_file_executable(&server)
                    .map_err(|e| format!("eiwa-lsp: chmod failed: {e}"))?;
                // The same tarball ships `eiwac`, used for diagnostics.
                let _ = zed::make_file_executable(&eiwac);
            }
            zed::set_language_server_installation_status(
                id,
                &LanguageServerInstallationStatus::None,
            );
        }
        Ok((server, eiwac))
    }
}

impl zed::Extension for EiwaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        // A `binary.path` in user settings overrides this command entirely,
        // so manual setups keep working untouched. Otherwise prefer a local
        // install over downloading a duplicate into the work dir.
        if let Some((server, eiwac)) = Self::local_server(worktree) {
            let env = eiwac
                .map(|e| vec![("EIWAC".to_string(), e)])
                .unwrap_or_default();
            return Ok(Command {
                command: server,
                args: Vec::new(),
                env,
            });
        }
        let (server, eiwac) = self.ensure_server(language_server_id)?;
        Ok(Command {
            command: server,
            args: Vec::new(),
            env: vec![("EIWAC".to_string(), eiwac)],
        })
    }
}

zed::register_extension!(EiwaExtension);
