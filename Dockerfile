FROM rust:1.80-slim-bullseye as build
RUN apt-get update && apt-get install --no-install-recommends -y libpq-dev
WORKDIR /app/server

ADD server /app/server
ADD shared /app/shared
RUN cargo build --release

FROM rust:1.80-slim-bullseye
RUN apt-get update && apt-get install --no-install-recommends -y libpq5
COPY --from=build /app/server/target/release/dimppl-server .

CMD ["./dimppl-server"]
