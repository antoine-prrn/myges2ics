FROM rust:1.72.1-bookworm as build

RUN USER=root cargo new --bin myges2ics
WORKDIR /myges2ics

COPY . .

RUN cargo build --release

FROM debian:12.1-slim as myges2ics
COPY --from=build /myges2ics/target/release/myges2ics .
RUN apt update && apt install -y openssl
CMD ["./myges2ics"]
