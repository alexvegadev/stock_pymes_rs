FROM rust:1.59.0-alpine
RUN apk add libressl-dev
RUN apk add --no-cache musl-dev
WORKDIR /opt/stock_pymes_rs
COPY . ./
RUN cargo build --release
EXPOSE 8080

ENV DATABASE_URL=mysql://root:root@localhost:3306/stockpymes

CMD ["/opt/stock_pymes_rs/target/release/stock_pymes_rs"]