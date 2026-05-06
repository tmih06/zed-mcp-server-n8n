# n8n MCP Server for Zed

This extension connects Zed's Assistant to n8n MCP endpoints.

It supports:

- n8n instance-level MCP at `/mcp-server/http`
- MCP Server Trigger workflow URLs
- local `http://localhost` n8n
- remote n8n with bearer auth
- remote n8n with OAuth via `mcp-remote`

## Why this extension exists

Zed MCP extensions launch local commands over stdio.

n8n's MCP endpoints use HTTP, SSE, or streamable HTTP instead of stdio. This extension bridges the gap in two ways:

- token/header-based connections use [`supergateway`](https://github.com/supercorp-ai/supergateway)
- OAuth-only connections use [`mcp-remote`](https://github.com/geelen/mcp-remote)

This avoids unnecessary OAuth browser flows for local or token-authenticated n8n servers.

## Install in Zed

1. Open `Extensions` in Zed.
2. Run `zed: install dev extension`.
3. Select this repository directory.
4. Enable `n8n MCP Server` in your assistant tools/profile.

## Publish to the Zed registry

This repository is structured as a standalone Zed extension and can be submitted
to the public registry through [`zed-industries/extensions`](https://github.com/zed-industries/extensions).

Submission steps:

1. Push this repository to a public GitHub repository.
2. Fork and clone `https://github.com/zed-industries/extensions`.
3. Add this repository as a submodule at `extensions/mcp-server-n8n`.
4. Add an entry for `mcp-server-n8n` in the top-level `extensions.toml`.
5. Run `pnpm sort-extensions`.
6. Open a pull request to `zed-industries/extensions`.

Notes:

- The extension license is MIT and is included at the repository root.
- The published extension version in the registry must match `version` in `extension.toml`.
- The `repository` value in `extension.toml` must match the final public GitHub repository you submit.
- If you change the extension ID, do it before first publication. Zed treats the ID as the permanent identifier.
- Exact registry snippets are included in `PUBLISHING.md`.

## Settings

Add settings under `context_servers.mcp-server-n8n.settings`.

### Instance-level n8n with access token

```json
{
  "context_servers": {
    "mcp-server-n8n": {
      "settings": {
        "server_url": "https://your-n8n.example.com/mcp-server/http",
        "bearer_token": "YOUR_N8N_MCP_TOKEN"
      }
    }
  }
}
```

### Instance-level n8n with OAuth

```json
{
  "context_servers": {
    "mcp-server-n8n": {
      "settings": {
        "server_url": "https://your-n8n.example.com/mcp-server/http"
      }
    }
  }
}
```

Leave `bearer_token` unset or empty. `mcp-remote` will open the browser and complete OAuth if your n8n instance advertises it.

### Local self-hosted n8n

```json
{
  "context_servers": {
    "mcp-server-n8n": {
      "settings": {
        "server_url": "http://localhost:5678/mcp-server/http",
        "bearer_token": "YOUR_N8N_MCP_TOKEN",
        "allow_http": true
      }
    }
  }
}
```

### MCP Server Trigger workflow URL

```json
{
  "context_servers": {
    "mcp-server-n8n": {
      "settings": {
        "server_url": "https://your-n8n.example.com/mcp/your-trigger-path",
        "headers": [
          "Authorization:Bearer YOUR_MCP_BEARER_TOKEN"
        ]
      }
    }
  }
}
```

If your trigger uses Header auth instead of Bearer auth, replace the `Authorization:...` entry with the header required by your node credentials.

## Settings reference

- `server_url`: required n8n MCP endpoint
- `bearer_token`: optional token used to send `Authorization: Bearer ...`
- `headers`: optional repeated raw headers for custom auth
- `allow_http`: allow non-HTTPS endpoints such as localhost
- `transport`: `http-first`, `http-only`, `sse-first`, or `sse-only`
- `host`: optional OAuth callback host override
- `callback_port`: optional OAuth callback port override
- `auth_timeout_seconds`: optional OAuth timeout override
- `resource`: optional OAuth resource override
- `enable_proxy`: use `HTTP_PROXY` / `HTTPS_PROXY` / `NO_PROXY`
- `ignore_tools`: filter tools by exact name or wildcard
- `debug`: enable verbose `mcp-remote` logging
- `silent`: suppress normal bridge logs

## Runtime behavior

- If `bearer_token` or `headers` are set, the extension uses `supergateway`.
- If neither is set, the extension uses `mcp-remote` and expects OAuth or unauthenticated access.

For local n8n with an access token, always set `bearer_token`. That avoids the repeated browser tabs and callback timeout behavior you can get from OAuth-oriented bridging.

## Development

```bash
cargo build
```

## References

- Zed MCP docs: https://zed.dev/docs/ai/mcp
- Zed MCP extension docs: https://zed.dev/docs/extensions/mcp-extensions
- Zed extension development and publishing docs: https://zed.dev/docs/extensions/developing-extensions
- n8n MCP docs: https://docs.n8n.io/advanced-ai/mcp/accessing-n8n-mcp-server/
- n8n MCP Server Trigger docs: https://docs.n8n.io/integrations/builtin/core-nodes/n8n-nodes-langchain.mcptrigger/
