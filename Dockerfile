FROM rust:latest

LABEL authors="lukasbecker"

WORKDIR /usr/src/ActixWebTest
COPY . .
RUN cargo install --path .
CMD ["ActixWebTest"]
#TODO: Connect to Postgres DB Container through this container