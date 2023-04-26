# What is this?
This project is a messaging system build using Rust/Rocket, Redis, and PostgreSQL.

Rust with Rocket handles all the server functionality.

Redis handles login sessions.

Postgres handles accounts and messages.
(Messages could probably be moved to Redis)

# Developing:
You will need [diesel](https://diesel.rs/) installed to work with the ORM.

Use `diesel migration run` to set up the databases the first time. If you need to reset the database you can use `diesel migration redo`.

Included is a docker compose file that contains a postgres database for easy setup.

Included is a configureation file for insomnia making it easy to test the methods already there.

# Running
You will need Rust to compile the code then docker to setup to databases. Using the command
```shell
docker compose up
```
should load all the databases as you will need them.

