# micropolis-rs

## Development

> If you have Docker

```bash
docker-compose build && docker-compose run
```

> You can launch the game client in development mode with hot-reloading.

Follow the quicksilver game library's installation [instructions](https://github.com/ryanisaacg/quicksilver), then run:

```bash
# in ./micropolis_quicksilver
cargo web start --features quicksilver/stdweb --port 8000
```

> Classic debug mode is also available in desktop mode:

```bash
# in ./micropolis_quicksilver
cargo run
```
