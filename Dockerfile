# docker run --rm $(docker build -q .)
FROM rust:1.89-alpine3.20 AS builder

# RUN apk add --no-cache ca-certificates
# RUN update-ca-certificates

WORKDIR /src
COPY Cargo.* ./
COPY src ./src
RUN cargo build --release

# since I'm using pure rust dependencies, scratch is fine without additional changes
FROM scratch AS dist

COPY --from=builder /src/target/release/my_order_book /my_order_book
CMD ["/my_order_book"]