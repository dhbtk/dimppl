# Dev setup

1. Install rustup: https://rustup.rs/
2. Install rust nightly: `rustup install nightly`
3. Install diesel_cli: `cargo install diesel_cli --no-default-features --features postgres`

# Migrations

1. Create a new migration: `diesel migration generate <migration_name>`
2. Run migrations: `diesel migration run`

# Deploy to fly.io

1. Install flyctl: https://fly.io/docs/getting-started/installing-flyctl/
2. Login to fly.io: `flyctl auth login`
3. Deploy: `flyctl deploy`
