# Этап сборки
FROM rust:1.92-alpine AS builder

# Устанавливаем зависимости для сборки
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
# RUN apk add --no-cache x86_64-linux-musl-gcc

# Устанавливаем целевой архитектуру для Linux
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

# Копируем  весь исходный код
COPY . .
RUN cargo fetch --locked
RUN cargo build --release --target x86_64-unknown-linux-musl --frozen

# Этап рантайма
FROM alpine:latest

# Устанавливаем зависимости для рантайма
RUN apk add --no-cache libgcc openssl

WORKDIR /app

# Копируем бинарник и ассеты
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/anonimus-animals /app/
COPY --from=builder /usr/src/app/assets /app/assets

EXPOSE 8000

CMD ["/app/anonimus-animals"]