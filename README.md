# StageSync

![Work in Progress](https://img.shields.io/badge/status-WIP-yellow)

> [!IMPORTANT]
> *This project is still very much a work in progress, and far from completion.*

**StageSync** is an artist booking dashboard designed for booking agencies to streamline their workflow. With StageSync, agencies can easily manage ongoing bookings, track outstanding invoices, and update artist profiles all in one place. The app combines a powerful Rust backend for efficient, secure data handling with a modern, dynamic React (JavaScript) frontend to deliver a smooth, intuitive user experience.

This project aims to simplify the day-to-day tasks of booking agents, ensuring they have a clear overview of upcoming events, artist details, and financials, all within a user-friendly interface.

## Prerequisites
Make sure you have the following dependencies installed on your machine:
- Rust 1.78 or later (including `cargo`) and make sure it's in your system's PATH environment variable.
- PostgreSQL
- `diesel_cli` (`cargo install diesel_cli --no-default-features --features postgres` should do the trick)

## Getting started
1. Clone the project and create a `.env` file inside the root directory, containing the URL to a PostgreSQL database (replace with your own credentials):
```bash
git clone https://github.com/MartijnD92/stagesync-rust
cd stagesync-rust
echo DATABASE_URL=postgres://username:password@localhost/stagesync > .env
```
2. Create the database (if it didn't exist) and schema by running:
```bash
diesel setup
```
3. Run the project:
```bash
cargo run
```
The API should now be available at `http://127.0.0.1:8080/`.

## Available endpoints
Make sure to prefix all endpoints with `/api/v1`.

| Method | Endpoint            | Description                       |
|--------|---------------------|-----------------------------------|
| GET    | /artists         | Fetch all artists                |
| POST   | /artists         | Create a new artist              |
| GET    | /artists/{id}    | Fetch artist by ID               |
| GET    | /users               | Fetch all users                  |
| POST   | /users               | Add a new user                   |
| GET    | /gigs                | Fetch all gigs                   |
| GET    | /gigs/{id}           | Fetch gig by ID                  |
| POST   | /gigs                | Add a new gig						|

## Roadmap
- [x] Add, modify and delete users, artists and gigs through API endpoints
- [x] User authentication
- [x] Multi-user roles for agents and artists
- [ ] Dashboard UI
