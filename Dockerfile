FROM rust:1.31

WORKDIR /usr/src/ipseity

EXPOSE 8080

COPY . .

RUN cargo install --path .
RUN cargo build --release

CMD ["ipseity"]