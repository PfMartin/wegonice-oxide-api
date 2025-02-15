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

## Authentication workflow

### Login

#### Server login

1. Receive credentials (email, password)
2. Get auth info for email from database (email, password_hash, role, is_activated)
3. Verify password with password hash from auth info
4. Verify if user is activated (in the future we can deactivate the user and it won't be able to login anymore)
5. Generate token with claims (sub: email, role: role, exp: configurable in env file)
6. Return token in payload of response

#### Client login

1. Send credentials
2. Receive token from response
3. Send token as Bearer with every request in header `curl -H 'Authorization: Bearer <TOKEN>'`

### Register

#### Server Registration

1. Receive credentials (email, password) - Same password verification is done in frontend
2. Create new user with credentials
3. Send email to provided email with verification link
4. Hitting the verification link (`<routeName>/user_id`) activates user
5. Now follow [Login Workflow](#login)

#### Client Registration

1. Send credentials (email, password)
2. User has to click on verification link in email
3. Use [Login Workflow](#login)

## TODOs

- Make token expiration configurable in .env file