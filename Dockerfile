FROM rust:latest

LABEL authors="lukasbecker"

WORKDIR /usr/src/ActixWebTest
COPY . .
RUN cargo install --path .
CMD ["ActixWebTest"]