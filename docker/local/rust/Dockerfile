FROM rust:1.87

WORKDIR /app

COPY . .

# ホットリロード用のライブラリをインストール
RUN cargo install cargo-watch

# Rust用のフォーマッターをインストール
RUN rustup component add rustfmt

# Rust用のリンターをインストール
RUN rustup component add clippy
