Configure one n8n MCP URL, then enable the server in your Zed assistant profile.

Supported n8n targets:

1. Instance-level MCP:
   `https://<your-n8n-domain>/mcp-server/http`
2. MCP Server Trigger node:
   use the node's `Production URL` or `Test URL`

Authentication options:

1. OAuth2:
   leave `bearer_token` empty and let `mcp-remote` complete the browser login flow.
2. Access token / bearer auth:
   set `bearer_token`. This uses `supergateway` and is the preferred option for local n8n.
3. Custom header auth:
   add entries under `headers`. This also uses `supergateway`.

For local self-hosted n8n over `http://localhost`, set `allow_http` to `true`.
