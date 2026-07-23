<h1 align="center">
  <img src="assets/icon.png?v=1.0.31" width="48" height="48" valign="middle"> Todo
</h1>

<p align="center">
  <b>Minimalist, collaborative lists and task todo manager written in Rust.</b>
</p>

---

### Instant One-Line Install (Docker Container)

Run the official zero-dependency container on port 4403:

```bash
docker run -d --name todo -p 4403:4403 -v /mnt/user/appdata/todo:/config ghcr.io/studio2201/todo:latest
```

Open your browser to `http://localhost:4403` to start creating task lists immediately.

---

### One-Line Install (Native Package Manager)

On Debian, Ubuntu, Fedora, or RHEL:

```bash
curl -fsSL https://studio2201.github.io/packages/install.sh | sudo bash
```

---

### Unraid NAS Deployment

Deploy via the official Unraid Template:

1. Copy [`todo.xml`](todo.xml) to your Unraid flash drive under `/boot/config/plugins/dockerMan/templates-user/`.
2. Open **Docker** -> **Add Container** -> Select **todo** from the template dropdown.
3. Click **Apply**.

---

### Environment Configuration

The backend service can be customized using the following environment variables:

| Variable | Description | Default |
| :--- | :--- | :---: |
| `PORT` | Network port the web server binds to | `4403` |
| `TODO_PIN` | Security PIN required for application access | *(Disabled)* |
| `TODO_DATA_DIR` | Directory path for persistent data and lists | `/config` |
| `TODO_ALLOWED_ORIGINS` | CORS allowed origins list (comma-separated) | `*` |
| `TRUST_PROXY` | Honor reverse proxy headers (`X-Forwarded-For`) | `false` |
| `TRUSTED_PROXY_IPS` | Comma-separated CIDR list of trusted reverse proxies | *(None)* |
| `LOG_LEVEL` | Tracing filter (`error`, `warn`, `info`, `debug`) | `info` |

---

### Administration CLI & TUI Dashboard

Every container and package includes a built-in administration utility (`todo`).

Launch interactive TUI dashboard:
```bash
docker exec -it todo todo tui
```

System diagnostics and self-healing check:
```bash
docker exec -it todo todo doctor
```

CLI Command Reference:
- `todo tui` — Interactive terminal user interface.
- `todo doctor` — Diagnoses storage permissions, ports, and database health.
- `todo status` — Displays network configuration and security parameters.
- `todo data stats` — Shows storage utilization and entry metrics.
- `todo data list` — Lists task lists and item entries.

---

### Architecture & Security

- **Axum Web Backend**: High-concurrency async HTTP/JSON runtime built on Tokio.
- **Yew WebAssembly Frontend**: Type-safe client bundle running natively in browser WASM runtime.
- **Strict Input & Path Sanitization**: Path canonicalization guards preventing directory traversal escapes.
- **Fail-Closed Security PIN Authentication**: Rate-limited brute force protection with automatic lockout timers.

---

### License

Distributed under the Apache 2.0 License. See [LICENSE](LICENSE) for details.
