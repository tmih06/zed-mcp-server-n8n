use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const EXTENSION_ID: &str = "mcp-server-n8n";
const MCP_REMOTE_PACKAGE_NAME: &str = "mcp-remote";
const MCP_REMOTE_SERVER_PATH: &str = "node_modules/mcp-remote/dist/proxy.js";
const SUPERGATEWAY_PACKAGE_NAME: &str = "supergateway";
const SUPERGATEWAY_SERVER_PATH: &str = "node_modules/supergateway/dist/index.js";

#[derive(Debug, Deserialize, JsonSchema)]
struct N8nContextServerSettings {
    server_url: String,
    #[serde(default)]
    bearer_token: Option<String>,
    #[serde(default)]
    headers: Vec<String>,
    #[serde(default)]
    allow_http: bool,
    #[serde(default = "default_transport")]
    transport: String,
    #[serde(default)]
    host: Option<String>,
    #[serde(default)]
    callback_port: Option<u16>,
    #[serde(default)]
    auth_timeout_seconds: Option<u16>,
    #[serde(default)]
    resource: Option<String>,
    #[serde(default)]
    debug: bool,
    #[serde(default = "default_silent")]
    silent: bool,
    #[serde(default)]
    enable_proxy: bool,
    #[serde(default)]
    ignore_tools: Vec<String>,
}

fn default_transport() -> String {
    "http-first".to_string()
}

fn default_silent() -> bool {
    true
}

struct N8nModelContextExtension;

fn ensure_npm_package(package_name: &str) -> Result<()> {
    let latest_version = zed::npm_package_latest_version(package_name)?;
    let version = zed::npm_package_installed_version(package_name)?;
    if version.as_deref() != Some(latest_version.as_ref()) {
        zed::npm_install_package(package_name, &latest_version)?;
    }
    Ok(())
}

impl zed::Extension for N8nModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project(EXTENSION_ID, project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `server_url` setting".into());
        };

        let settings: N8nContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        if settings.server_url.trim().is_empty() {
            return Err("`server_url` must not be empty".into());
        }

        let has_bearer_token = settings
            .bearer_token
            .as_ref()
            .is_some_and(|token| !token.trim().is_empty());
        let has_custom_headers = settings.headers.iter().any(|header| !header.trim().is_empty());

        if has_bearer_token || has_custom_headers {
            ensure_npm_package(SUPERGATEWAY_PACKAGE_NAME)?;

            let entrypoint = env::current_dir()
                .unwrap()
                .join(SUPERGATEWAY_SERVER_PATH)
                .to_string_lossy()
                .to_string();

            let mut args = vec![
                entrypoint,
                "--streamableHttp".to_string(),
                settings.server_url.clone(),
                "--outputTransport".to_string(),
                "stdio".to_string(),
            ];

            if settings.debug {
                args.push("--logLevel".to_string());
                args.push("debug".to_string());
            } else if settings.silent {
                args.push("--logLevel".to_string());
                args.push("none".to_string());
            } else {
                args.push("--logLevel".to_string());
                args.push("info".to_string());
            }

            if let Some(token) = settings.bearer_token.as_ref() {
                if !token.trim().is_empty() {
                    args.push("--oauth2Bearer".to_string());
                    args.push(token.clone());
                }
            }

            for header in settings.headers {
                if !header.trim().is_empty() {
                    args.push("--header".to_string());
                    args.push(header);
                }
            }

            return Ok(Command {
                command: zed::node_binary_path()?,
                args,
                env: Vec::new(),
            });
        }

        ensure_npm_package(MCP_REMOTE_PACKAGE_NAME)?;

        let entrypoint = env::current_dir()
            .unwrap()
            .join(MCP_REMOTE_SERVER_PATH)
            .to_string_lossy()
            .to_string();

        let mut args = vec![entrypoint, settings.server_url.clone()];

        if let Some(port) = settings.callback_port {
            args.push(port.to_string());
        }

        if settings.allow_http {
            args.push("--allow-http".to_string());
        }

        if settings.transport != "http-first" {
            args.push("--transport".to_string());
            args.push(settings.transport.clone());
        }

        if let Some(host) = settings.host.as_ref() {
            if !host.trim().is_empty() {
                args.push("--host".to_string());
                args.push(host.clone());
            }
        }

        if let Some(timeout) = settings.auth_timeout_seconds {
            args.push("--auth-timeout".to_string());
            args.push(timeout.to_string());
        }

        if let Some(resource) = settings.resource.as_ref() {
            if !resource.trim().is_empty() {
                args.push("--resource".to_string());
                args.push(resource.clone());
            }
        }

        if settings.enable_proxy {
            args.push("--enable-proxy".to_string());
        }

        for tool in settings.ignore_tools {
            if !tool.trim().is_empty() {
                args.push("--ignore-tool".to_string());
                args.push(tool);
            }
        }

        if settings.debug {
            args.push("--debug".to_string());
        }

        if settings.silent {
            args.push("--silent".to_string());
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: Vec::new(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(N8nContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(N8nModelContextExtension);
