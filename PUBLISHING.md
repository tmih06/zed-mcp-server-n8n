# Publishing to the Zed Registry

This file contains the exact snippets needed to submit this extension to
`zed-industries/extensions`.

## Before submitting

1. Push this repository to a public GitHub repository.
2. Ensure the public repository URL matches `repository` in `extension.toml`.
3. Commit the current `extension.wasm` together with the source changes.
4. Tag or otherwise retain the exact commit you want the registry to consume.

Current expected repository URL:

```text
https://github.com/tmih06/zed-mcp-server-n8n
```

If you publish under a different repository, update `extension.toml` first.

## `.gitmodules` entry

Add this to `zed-industries/extensions/.gitmodules`:

```ini
[submodule "extensions/mcp-server-n8n"]
	path = extensions/mcp-server-n8n
	url = https://github.com/tmih06/zed-mcp-server-n8n.git
```

## `extensions.toml` entry

Add this to `zed-industries/extensions/extensions.toml`:

```toml
[mcp-server-n8n]
submodule = "extensions/mcp-server-n8n"
version = "0.1.0"
```

The `version` value must match `extension.toml`.

## PR checklist

- The submodule path is `extensions/mcp-server-n8n`
- The submodule URL points at the public repository
- `extensions.toml` uses the same extension ID and version
- `extension.toml` has the correct `repository` URL
- The repository root includes `LICENSE`
- The extension builds successfully
