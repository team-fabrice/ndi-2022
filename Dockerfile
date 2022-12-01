FROM rustlang/rust:nightly as builder

ADD . .
RUN cargo build --release

FROM alpine:latest

EXPOSE 7878
CMD ["target/release/app"]