FROM rust
WORKDIR app
ADD . /app
EXPOSE 8084
RUN cargo build
ENTRYPOINT ["target/debug/stockage"]
