FROM rust:alpine AS build

WORKDIR /usr/src/honeybot
COPY . .
RUN cargo build --release

FROM scratch
WORKDIR /usr/src/honeybot
COPY --from=build /usr/src/honeybot/target/release/honeybot .
COPY --from=build /usr/src/honeybot/migrations/ migrations

EXPOSE 1234
ENV RUST_LOG=error

CMD ["/usr/src/honeybot/honeybot"]
