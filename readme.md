# Projet virtualisation Docker

> This project was made for the course of "Admin virtualisation" of [Mr. Anthony Busson](https://anthonybusson.fr/). <br>

#### It consists of a really simple web app that is generating random numbers with a refresh button. It has distinct client and server, in order to demonstrate the use of Docker and Docker Compose.

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
├── database # the database config and data folder
│   └─ data # contains the docker volume of the pgsql database
├── server # the server source code
│   ├─ Dockerfile
│   ├─ .dockerignore
│   └─ ...
└── client # the client source code
    ├─ Dockerfile
    ├─ .dockerignore
    └─ ...
```

## Usage

1. Go in the app directory
2. Run the `docker compose up` command (or `docker compose up -d` if you want to be detached)
3. Open `http://localhost:3000` in your browser

## Remarks

- Both server and clients use multi-stage builds to optimize caching capabilities of docker and therefore the final image.
- **The server image build is very long (~10min)**. I optimized it using `cargo-chef` which lets us separate the build of the dependencies from the build of our application binary itself, so they can be cached and not re-compiled each time the source code is modified. But this trick will not really be useful in the context of this project as you'll only build it once.
- I did my best to comment as far as i can the Dockerfiles.
- Applications communicate with each other through the `docker-compose` network, using the container names as hostnames (also known as *link aliases*).