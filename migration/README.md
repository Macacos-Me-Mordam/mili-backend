# 📦 Como Rodar as Migrations (SeaORM)

Este guia explica como aplicar as migrations no banco de dados PostgreSQL usando o subprojeto `migration` com SeaORM.

---

## ✅ Passo 1: Definir a URL do Banco de Dados

No terminal, exporte a variável de ambiente `DATABASE_URL` com os dados do seu container PostgreSQL:

```bash
export DATABASE_URL=postgres://postgres:postgres@localhost:55432/keycloak
cargo run -p migration
