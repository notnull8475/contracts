# Сборка приложения
FROM rust:1.72 as builder

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем зависимости Cargo
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release

# Копируем исходный код
COPY src ./src
RUN cargo build --release

# Финальный образ
FROM debian:buster-slim

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем скомпилированный бинарник
COPY --from=builder /app/target/release/backend .

# Команда запуска
CMD ["./backend"]