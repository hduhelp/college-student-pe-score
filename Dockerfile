FROM rustlang/rust:nightly-alpine AS builder

WORKDIR /workspace

COPY . .

RUN apk add --no-cache -U musl-dev
RUN cargo build -Z unstable-options \
    --release \
    --target x86_64-unknown-linux-musl \
    --out-dir /build

COPY ./config.yaml /build


FROM alpine

WORKDIR /app

COPY --from=builder /build .

ENV RUST_LOG=info

CMD ["./college-student-pe-score"]
