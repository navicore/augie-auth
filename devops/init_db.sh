docker run -p 5432:5432 --name augie-auth-postgres -e POSTGRES_PASSWORD=CHANGE_ME -e POSTGRES_USER=augie-auth -d postgres

echo DATABASE_URL=postgres://augie-auth:CHANGE_ME@localhost/augie-auth > .env
