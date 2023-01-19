# ==================
# == dependencies ==
# ==================

# shortcut to avoid having to write the full image name for the base image
FROM lukemathwalker/cargo-chef:latest AS chef
WORKDIR /app

# use chef to prepare the dependency tree json
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ==================
# ==== builder =====
# ==================
FROM chef as builder

# first, build the dependencies in a separate layer, in order to cache them and avoid rebuilding them every time the source code changes
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release --recipe-path recipe.json

# copy all the source code
COPY . .

# build the application
RUN cargo build --release --bin server

# ==================
# ==== runner ======
# ==================
FROM alpine:latest as runner
WORKDIR /app

# copy needed files for the binary from the previous stage
COPY --from=builder /app/Rocket.toml ./
# copy the binary from the previous stage
COPY --from=builder /app/target/release/server ./server

RUN chmod u+x ./server

# start the app
ENTRYPOINT ["./server"]