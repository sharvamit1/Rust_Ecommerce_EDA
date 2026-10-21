FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:latest AS broker_runtime
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/event_broker_service .
CMD ["./event_broker_service"]

FROM alpine:latest AS payment_runtime
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/payment_service .
CMD ["./payment_service"]

FROM alpine:latest AS order_runtime
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/order_service .
CMD ["./order_service"]

FROM alpine:latest AS shipping_runtime
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/shipping_service .
CMD ["./shipping_service"]