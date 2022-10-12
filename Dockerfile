FROM rust:1.64 AS builder
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder ./target/release/todo-ci ./target/release/todo-ci
ENTRYPOINT  ["/target/release/todo-ci"]