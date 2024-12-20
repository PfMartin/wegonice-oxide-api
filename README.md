# wegonice-oxide-api

Rust wegonice api

## Database setup

1. Create a new file called `.env` and define your database connection variables like in `.example.env`

2. Start database and create user

```bash
cd database
docker compose up -d
cd ..

make db-create-user

# Check if you can connect as a user and as an admin
make db-connect-admin
make db-connect-user
```

## Testing

### Unit tests

Run unit tests with coverage using the following command.

```bash
make unit-tests
```

## TODOs

