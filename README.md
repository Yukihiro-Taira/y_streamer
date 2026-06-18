# Video Platform App

## Run the project

```bash
./reset_db.sh # This will reset the DB and run the migrations

pnpm install # If you don't have pnpm, run : npm install -g pnpm
dx serve
```

## User Credentials

The following test users are available for login:

| Role        | Email               | Password   |
|-------------|---------------------|------------|
| Root        | root@example.com    | `password` |
| Admin       | admin@example.com   | `password` |
| RegularUser | alice@example.com   | `password` |
| RegularUser | bob@example.com     | `password` |

