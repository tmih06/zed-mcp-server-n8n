# n8n MCP Server for Zed

This extension connects Zed's Assistant to n8n MCP endpoints over HTTP.

It supports:

- n8n instance-level MCP at `/mcp-server/http`
- MCP Server Trigger workflow URLs
- local `http://localhost` n8n
- remote n8n with bearer auth
- remote n8n with OAuth

## Install in Zed

1. Open `Extensions` in Zed.
2. Run `zed: install dev extension`.
3. Select this repository directory.
4. Enable `n8n MCP Server` in your assistant tools/profile.

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

Leave `bearer_token` unset or empty. Zed will open the browser and complete OAuth if your n8n instance advertises it.

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

For local n8n with an access token, always set `bearer_token`.

## References

- Zed MCP docs: https://zed.dev/docs/ai/mcp
- Zed MCP extension docs: https://zed.dev/docs/extensions/mcp-extensions
- n8n MCP docs: https://docs.n8n.io/advanced-ai/mcp/accessing-n8n-mcp-server/
- n8n MCP Server Trigger docs: https://docs.n8n.io/integrations/builtin/core-nodes/n8n-nodes-langchain.mcptrigger/
