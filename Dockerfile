FROM rust:1.79-bookworm AS builder
WORKDIR /usr/src/ghost-bot
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt update && \
	apt install sqlite3 curl -y && \
	rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/ghost /usr/local/bin/ghost
CMD /usr/local/bin/ghost ${URL} ${USR} ${PW}
