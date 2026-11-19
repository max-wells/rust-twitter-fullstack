# Cross Platform Fullstack Starter

## Requirements

You need to have Leptos (nightly), Just, Tauri, and PSQL on your machine.
If not already, you can refer to [PREREQUISITES.md](PREREQUISITES.md).


## CLI

```bash
cargo install ui-cli

ui starters
# └─> start-tauri-fullstack 
```


## Setup the project

```bash
# Install Tailwind CSS
pnpm install

# Create the DB (+ seed)
just reset_db
```

## Run the project

```bash
cargo leptos watch  # For Web
cargo tauri dev     # For desktop
just run_ios        # For iOS
just run_android    # For Android
```


## Run with Docker

The easiest way to run the full stack (app + Postgres) without installing Rust,
Tauri, or PostgreSQL locally:

```bash
# Build and start the app and database
docker compose up --build

# The app is then available at http://localhost:3000
# Postgres is available at localhost:5432

# Stop and remove containers
docker compose down

# To reset the database volume (drops all data)
docker compose down -v
```

The `app` service runs database migrations automatically on startup, so the
database is seeded with sample data on first launch. The `DATABASE_URL` is set
by `docker-compose.yml` to point at the `db` service, overriding the local
`127.0.0.1` value in `.env`.


## Deploy with Zerops

Install the CLI:

```bash
pnpm i -g @zerops/zcli
```

Deploy:

```bash
zcli push
```

## Credits

Backend based on [rust-twitter-clone](https://github.com/FrancescoXX/rust-twitter-clone) by Francesco Ciulla.

## License

MIT License - see [LICENSE](LICENSE) for details.
