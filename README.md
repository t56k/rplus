# rplus

## 🦀 Backend rebuild with typechecking

### Database

The application needs a running Postgres database. Use local Postgres or run it in a Docker container:
```bash
docker run -p 5432:5432 --rm -e POSTGRES_PASSWORD=password postgres:12
```

When the database is running make sure its connection string matches the one in `.env` file.

Use Diesel CLI to setup database and run migrations:
```bash
diesel database setup
```

### Application

Before starting the application set `DATABASE_URL` environment variable. If the database connection string matches the one in `.env` file, run:
```bash
export $(cat .env | xargs)
```

Run application with `cargo`:
```bash
cargo run
```
