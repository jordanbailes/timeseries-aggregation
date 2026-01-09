FROM rust AS build-env
RUN apt update

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain.toml ./rust-toolchain.toml
COPY ./src ./src

COPY ./diesel.toml ./diesel.toml
COPY ./migrations ./migrations

RUN cargo build --release

FROM gcr.io/distroless/cc-debian13:nonroot
COPY --from=build-env /app/target/release/timeseries-aggregation /home/nonroot/
WORKDIR /home/nonroot
ENTRYPOINT ["/home/nonroot/timeseries-aggregation"]