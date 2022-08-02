FROM rust:latest

WORKDIR /usr/src/imagine_backend
COPY . .

RUN cargo install --path .

CMD ["imagine_backend"]