### Juniper Resistance

Simple example repo implementing a GraphQL API in rust, using Juniper and Rocket

### Setting up database

```bash
# Prerequisite: Install postgresql

# Install diesel cli, handling installation issues along the way.
# For example, I needed to `sudo apt install libmysqlclient-dev`
cargo install diesel_cli

# Run database setup, with restricted user
./scripts/setup_db
```


### Running

```bash
# Use nightly for rocket
rustup override set nightly
cargo build
cargo run
```
