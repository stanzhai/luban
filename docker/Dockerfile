FROM ekidd/rust-musl-builder:stable as builder
USER root

# Cache Rust dependencies, 参考： https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
# 原理： https://blog.mgattozzi.dev/caching-rust-docker-builds/
RUN cargo new --bin temp
WORKDIR ./temp
COPY ./temp/migration ./migration
COPY ./temp/Cargo.lock ./Cargo.lock
COPY ./temp/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm -rf src

ADD ./temp/. ./.

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/luban*
RUN cargo build --release

FROM alpine:latest

ARG APP=/luban

RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/temp/target/x86_64-unknown-linux-musl/release/luban ${APP}/luban
COPY --from=builder /home/rust/src/temp/.env ${APP}/

WORKDIR ${APP}
EXPOSE 20430
VOLUME ["/luban/.env"]

CMD ["./luban"]
