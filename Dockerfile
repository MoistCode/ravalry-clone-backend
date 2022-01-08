FROM rust:1.53 as builder
WORKDIR /usr/src/ravalry
COPY . .
RUN cargo install diesel_cli
RUN diesel setup
RUN diesel migration run
RUN cargo install --path .
RUN ls -a

FROM debian:buster-slim
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ravalry /usr/local/bin/ravalry
RUN sqlite3 test.db
CMD ["ravalry"]