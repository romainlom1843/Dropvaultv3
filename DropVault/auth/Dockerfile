### build stage ###
FROM rust:slim as build

# install libpq, libsqlite and create new empty binary project
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev libsqlite3-dev;

WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

RUN touch .env;

# rebuild app with project source
RUN cargo build

#### deploy stage ###
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5 libsqlite3-0; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/debug/auth .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .
COPY ./wait-for-it.sh .

# expose port
EXPOSE 8080

RUN chmod +x wait-for-it.sh

## Faire les migrations
#Installation diesel cli
RUN apt-get update -y && apt install curl -y

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt install postgresql -y
RUN apt install libpq-dev -y

RUN apt install gcc -y
#ENV MACHINE=armv7
#ENV ARCH=arm
#ENV CC=arm-linux-gnueabihf-gcc

RUN cargo install diesel_cli

WORKDIR /tmp

COPY Cargo.toml .
COPY migrations/ ./migrations/
COPY migrate.sh .
RUN chmod +x migrate.sh

RUN ./migrate.sh

# run the binary
#CMD ["./wait-for-it.sh", "db-auth:5432", "--", "/app/auth"]
CMD ["/app/auth"]
