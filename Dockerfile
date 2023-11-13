FROM rust

RUN apt-get update && apt-get install -y wget
RUN wget -q https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt-get install -y ./google-chrome-stable_current_amd64.deb
RUN rm google-chrome-stable_current_amd64.deb

WORKDIR /app

COPY Cargo.toml Cargo.toml

COPY Cargo.lock Cargo.lock

RUN mkdir src && echo 'fn main() {panic!("not ready");}' > src/main.rs

RUN cargo build --release --locked

RUN rm -rf src

COPY src src

RUN touch src/main.rs && cargo build --release --locked

CMD cargo run --release --locked
