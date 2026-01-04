use zed_extension_api::{self as zed, Result};

struct TailwindCssExtension;

impl zed::Extension for TailwindCssExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "tailwindcss-language-server" => {
                // Use the system-installed tailwindcss-language-server or install via npm
                let server_path = worktree
                    .which("tailwindcss-language-server")
                    .unwrap_or_else(|| "tailwindcss-language-server".to_string());

                Ok(zed::Command {
                    command: server_path,
                    args: vec!["--stdio".to_string()],
                    env: Default::default(),
                })
            }
            _ => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match language_server_id.as_ref() {
            "tailwindcss-language-server" => Ok(Some(zed::serde_json::json!({
                "userLanguages": {
                    "eex": "html",
                    "heex": "html",
                    "svelte": "html",
                    "vue": "html",
                    "astro": "html",
                    "jsx": "javascriptreact",
                    "tsx": "typescriptreact",
                    "php": "html",
                    "erb": "html",
                    "twig": "html",
                    "blade": "html"
                },
                "includeLanguages": {
                    "typescript": "javascript",
                    "typescriptreact": "javascript",
                    "vue": "html",
                    "svelte": "html"
                },
                "classAttributes": [
                    "class",
                    "className",
                    "ngClass",
                    "class:list",
                    ":class",
                    "v-bind:class"
                ],
                "experimental": {
                    "classRegex": [
                        "tw`([^`]*)",
                        "tw=\"([^\"]*)",
                        "tw={\"([^\"}]*)",
                        "tw\\.\\w+`([^`]*)",
                        "tw\\(.*?\\)`([^`]*)",
                        "classNames\\s*\\(\\s*['\"`]([^'\"`]*)['\"`]",
                        "cva\\s*\\([^)]*['\"`]([^'\"`]*)['\"`]",
                        "cn\\s*\\([^)]*['\"`]([^'\"`]*)['\"`]"
                    ]
                }
            }))),
            _ => Ok(None),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        match language_server_id.as_ref() {
            "tailwindcss-language-server" => {
                let settings = zed::settings::LspSettings::for_worktree(
                    "tailwindcss-language-server",
                    worktree,
                )
                .ok()
                .and_then(|lsp_settings| lsp_settings.settings.clone())
                .unwrap_or_else(|| {
                    zed::serde_json::json!({
                        "emmetCompletions": false,
                        "includeLanguages": {
                            "eex": "html",
                            "heex": "html",
                            "svelte": "html",
                            "vue": "html",
                            "astro": "html"
                        },
                        "files": {
                            "exclude": [
                                "**/.git/**",
                                "**/node_modules/**",
                                "**/.hg/**",
                                "**/.svn/**"
                            ]
                        },
                        "editor": {
                            "quickSuggestions": {
                                "strings": true
                            }
                        },
                        "validate": true,
                        "lint": {
                            "cssConflict": "warning",
                            "invalidApply": "error",
                            "invalidScreen": "error",
                            "invalidVariant": "error",
                            "invalidConfigPath": "error",
                            "invalidTailwindDirective": "error",
                            "recommendedVariantOrder": "warning"
                        },
                        "hovers": true,
                        "suggestions": true,
                        "codeActions": true,
                        "colorDecorators": true
                    })
                });

                Ok(Some(zed::serde_json::json!({
                    "tailwindCSS": settings
                })))
            }
            _ => Ok(None),
        }
    }
}

zed::register_extension!(TailwindCssExtension);
