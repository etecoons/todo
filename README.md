# Adam - Blazing Fast Todo List

<p align="center">
  <img src="https://raw.githubusercontent.com/UberMetroid/adam/main/frontend/Assets/favicon.png" alt="Adam Logo" width="128" height="128">
</p>

Adam is a blazing fast, single-purpose todo list application written in 100% Rust using Axum on the backend and Yew (WebAssembly) on the frontend.

---

## 📦 Container Registry

The Docker image is published to the following registries:

*   **Docker Hub (Recommended)**: [ubermetroid/adam](https://hub.docker.com/r/ubermetroid/adam)
*   **GitHub Container Registry (GHCR)**: [ghcr.io/ubermetroid/adam](https://github.com/UberMetroid/adam/pkgs/container/adam)

---

## 🐳 Container Installation

1. Create a `docker-compose.yml` file:

```yaml
version: '3'
services:
  adam:
    image: ubermetroid/adam:latest
    container_name: adam
    restart: unless-stopped
    ports:
      - 4403:4403
    volumes:
      - ./data:/app/data
    environment:
      - PORT=4403
      - ADAM_PIN=1234
      - SITE_TITLE=Adam
      - SINGLE_LIST=false
      - ALLOWED_ORIGINS=*
```

2. Run the container:

```bash
docker compose up -d
```

3. Open your browser and navigate to `http://localhost:4403`.

### Building the Image Locally

To build the Docker container locally from the source files:

```bash
docker build -t ubermetroid/adam:latest .
```

---

## 📋 Configuration Options

Configure these settings inside your Docker Compose environment or container environment variables:

| Variable | Description | Default |
| :--- | :--- | :--- |
| `PORT` | The port number the backend HTTP server will bind to inside the container. | `4403` |
| `SITE_TITLE` | Custom website title rendered in navigation headers, browser tabs, and PWA manifest. | `Adam` |
| `BASE_URL` | Application base URL. Essential when deploying behind reverse proxies to ensure redirect and websocket links are resolved correctly. | `http://localhost:4403` |
| `ALLOWED_ORIGINS` | Comma-separated list of allowed HTTP request origins (CORS filter). Use `*` to allow all origins. | `*` |
| `ADAM_PIN` | Optional 4–10 digit PIN (numerical only) to lock access to the interface. Leave empty for public mode. | None |
| `TZ` | Timezone for the container processes and logs. | `UTC` |
| `SINGLE_LIST` | Force UI to hide list switcher and display only a single list. | `false` |
| `ENABLE_TRANSLATION` | Enable the multi-language / translation selector in the navigation header (true/false). | `false` |
| `ENABLE_THEMES` | Enable the theme selector in the navigation header (true/false). | `true` |
| `ENABLE_PRINT` | Enable the print button in the navigation header (true/false). | `true` |
| `MAX_ATTEMPTS` | Number of failed PIN attempts permitted before locking out the user client IP address. | `5` |

---

*Note: This repository was forked from [DumbDo](https://github.com/DumbWareio/DumbDo).*
