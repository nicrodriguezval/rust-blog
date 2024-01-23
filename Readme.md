# README

## Diesel ORM

### Install diesel_cli

Before installing diesel_cli, make sure you have installed postgresql and libpq-dev.

```bash
cargo install diesel_cli --no-default-features --features postgres
```

### Setup

```bash
diesel setup
```

### Create migration

```bash
diesel migration generate create_posts
```

### Run migration

```bash
diesel migration run
```
### Revert migration

```bash
diesel migration revert
```

## Database

### Run
```bash
docker-compose up -d
```

### Credentials
```
jdbc:postgresql://localhost:5432/blog 
username: postgres
password: admin
```