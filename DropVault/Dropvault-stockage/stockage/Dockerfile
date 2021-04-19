# build stage
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

#On installe openssl
#RUN apt-get install wget -y
RUN apt install pkg-config -y
RUN apt install libssl-dev -y

#WORKDIR /tmp

#RUN wget https://www.openssl.org/source/openssl-1.0.1t.tar.gz
#RUN tar xzf openssl-1.0.1t.tar.gz
#RUN export MACHINE=armv7
#RUN export ARCH=arm
#RUN export CC=arm-linux-gnueabihf-gcc
#RUN cd openssl-1.0.1t && ./config shared && make && cd -
#RUN export OPENSSL_LIB_DIR=/tmp/openssl-1.0.1t/
#RUN export OPENSSL_INCLUDE_DIR=/tmp/openssl-1.0.1t/include

# rebuild app with project source
RUN cargo build

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
RUN mkdir /app/stockms
RUN mkdir /app/stock

WORKDIR /app/stockms

# install libpq and libsqlite
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq5 libsqlite3-0; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/debug/stockage .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .
COPY ./wait-for-it.sh .

# expose port
EXPOSE 8084

RUN chmod +x wait-for-it.sh

# run the binary
CMD ["./wait-for-it.sh", "db-stock:5432", "--", "/app/stockms/stockage"]

