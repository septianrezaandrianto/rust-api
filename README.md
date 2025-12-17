# rust-api

Simple REST API dengan **Rust + Axum + SQLx + PostgreSQL** (CRUD Product) + **SQLx migrations**.

---

## Tech Stack

- Rust (Cargo)
- Axum (HTTP server & routing)
- SQLx (PostgreSQL driver + query mapper)
- dotenvy (load `.env`)
- chrono (timestamp)
- rust_decimal (tipe aman untuk `NUMERIC`/uang)

---

## Prerequisites

- PostgreSQL sudah running (local / docker)
- Rust toolchain ter-install
  <p>
  <img src="https://github.com/user-attachments/assets/fe6a5476-95e1-4a50-842f-7e0b6b1d3618"
       alt="Rust toolchain"
       width="520" />
</p>

---

## Setup

### 1) Clone repo
```bash
git clone https://github.com/septianrezaandrianto/rust-api.git
cd rust-api
