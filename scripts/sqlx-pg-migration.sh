#!/bin/sh
DATABASE_URL=postgres://postgres:postgres@localhost/vpn-panel sqlx migrate add --source ./backend/src/external/repo/implementations/sea_query_postgres/migrations -r "$@"
