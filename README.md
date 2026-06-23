# RustDo - Blazing Fast Todo List

<p align="center">
  <img src="https://raw.githubusercontent.com/UberMetroid/RustDo/main/frontend/Assets/favicon.png" alt="RustDo Logo" width="128" height="128">
</p>

RustDo is a blazing fast, single-purpose todo list application written in 100% Rust using Axum on the backend and Yew (WebAssembly) on the frontend.

---

## 🐳 Container Installation

### Option 1: Docker Compose (Recommended)

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  rustdo:
    image: ubermetroid/rustdo:latest
    container_name: rustdo
    restart: unless-stopped
    ports:
      - 4403:4403
    volumes:
      - ./data:/app/data
    environment:
      - PORT=4403
      - RUSTDO_PIN=1234
      - RUSTDO_SITE_TITLE=RustDo
      - SINGLE_LIST=false
      - ALLOWED_ORIGINS=*
```

2. Run the container:

```bash
docker compose up -d
```

3. Open your browser and navigate to `http://localhost:4403`.

### Option 2: Docker CLI

Run the following command to start the container:

```bash
docker run -d \
  --name rustdo \
  --restart unless-stopped \
  -p 4403:4403 \
  -v $(pwd)/data:/app/data \
  -e RUSTDO_PIN=1234 \
  ubermetroid/rustdo:latest
```

---

## 📋 Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4403` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. *(Supports fallback `RUSTRUSTDO_TITLE`)* | `RustDo` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4403` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `RUSTDO_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `SINGLE_LIST` | Force UI to hide list switcher and display only a single list. | `false` |
