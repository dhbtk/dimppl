FROM rust:1.72-slim-bullseye as build
RUN apt-get update && apt-get install --no-install-recommends -y libpq-dev
WORKDIR /app

COPY . /app
RUN cargo build --release

FROM rust:1.72-slim-bullseye
RUN apt-get update && apt-get install --no-install-recommends -y libpq5
COPY --from=build /app/target/release/dimppl-server .

CMD ["./dimppl-server"]
