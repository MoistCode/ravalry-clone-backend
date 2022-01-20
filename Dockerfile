FROM rust:latest as builder
WORKDIR /usr/src/ravalry
COPY . .
RUN cargo install diesel_cli
RUN diesel setup --database-url=test.db
RUN diesel migration run --database-url=test.db
RUN cargo install --path .
RUN ls -a

FROM debian:buster-slim
RUN apt-get update && apt-get install -y sqlite3 libsqlite3-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ravalry /usr/local/bin/ravalry
COPY --from=builder /usr/src/ravalry/test.db test.db
RUN sqlite3 test.db
CMD ["ravalry"]