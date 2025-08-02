# Estágio de Build: Compila a aplicação Rust
FROM rust:latest AS builder

# Define o diretório de trabalho
WORKDIR /usr/src/app

# Copia todo o código-fonte
COPY . .

# Compila o projeto em modo de release para otimização
RUN cargo build --release

# --- ESTÁGIO DE PRODUÇÃO ---
# Usamos uma base um pouco mais moderna e instalamos a dependência que falta
FROM debian:bookworm-slim

# Instala o OpenSSL (que provê a libssl.so.3) e limpa o cache do apt
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*

# Copia apenas o binário compilado do estágio de build
COPY --from=builder /usr/src/app/target/release/axum-api /usr/local/bin/axum-api

# Expõe a porta que a aplicação usa
EXPOSE 3000

# Define o comando para executar a aplicação
CMD ["axum-api"]