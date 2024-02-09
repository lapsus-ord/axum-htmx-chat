# Axum / HTMX chat app

An axum / htmx experiment with websocket.

## Libraries used

- [axum](https://github.com/tokio-rs/axum/): Web server (rest + websocket)
- [htmx](https://github.com/bigskysoftware/htmx/): Library that allows you to access modern browser features directly from HTML attributes
- [askama](https://github.com/djc/askama/): Templating engine (based on Jinja)

## Run dev

(I will do a docker compose later if I have time)

```sh
pnpm install
```

```sh
pnpm watch
```

> ℹ️ It uses mprocs to run [multiple](https://github.com/pvolok/mprocs/) tasks in parallel with a nice little UI.
