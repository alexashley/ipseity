FROM rust:1.31

WORKDIR /usr/src/ipseity

COPY . .

RUN cargo install --path .
RUN cargo build --release

CMD ["ipseity"]