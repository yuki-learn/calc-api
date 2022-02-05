# build
FROM rust:1.58.1 AS build

WORKDIR /app
COPY ./calc-api .
RUN cargo build --release

# production
FROM debian:buster-slim AS production
COPY --from=build /app/target/release/calc-api .
CMD ["./calc-api"]