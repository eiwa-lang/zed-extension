use zed_extension_api as zed;

struct EiwaExtension;

impl zed::Extension for EiwaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        // Looked up on PATH as a fallback. Zed GUI apps don't inherit your
        // shell PATH, so prefer an absolute path in settings.json:
        //
        //   "lsp": { "eiwa-lsp": { "binary": { "path": "/abs/path/eiwa-lsp" } } }
        //
        // (The settings binary overrides this command.)
        Ok(zed::Command {
            command: "eiwa-lsp".to_string(),
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(EiwaExtension);
