# Projet virtualisation Docker

> This project was made for the course of "Admin virtualisation" of [Mr. Anthony Busson](https://anthonybusson.fr/). <br>
It consists of a really simple web app with distincts client and server, in order to demonstrate the use of Docker and Docker Compose.

## Architecture

![](https://i.imgur.com/R7PLN32.png)

The application is composed of three containers:
- `Client`: a [Next.js](https://nextjs.org/) web application written in [Typescript](https://www.typescriptlang.org/)
- `Server`: a [Rocket.rs](https://rocket.rs/) api back-end written in [Rust](https://www.rust-lang.org/fr)
- `Database`: a [PostgreSQL](https://www.postgresql.org/) database

## Structure

```bash
project 
├── docker-compose.yml # the docker-compose confi file, which manage the containers
├── server # the server source code
│   ├─ Dockerfile
│   ├─ .dockerignore
│   ├─ ...
└── client # the client source code
    ├─ Dockerfile
    ├─ .dockerignore
    └─ ...
```

## Usage

1. Go in the app directory
2. Run the `docker compose up` command (or `docker compose up -d` if you want to be detached)
3. Open `http://localhost:3000` in your browser