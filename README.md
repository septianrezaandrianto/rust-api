<img width="477" height="149" alt="image" src="https://github.com/user-attachments/assets/6e5e7d62-a132-4c00-92d7-0b37da7fec62" /># rust-api

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
  <img width="477" height="149" alt="image" src="https://github.com/user-attachments/assets/fe6a5476-95e1-4a50-842f-7e0b6b1d3618" />

---

## Setup

### 1) Clone repo
```bash
git clone https://github.com/septianrezaandrianto/rust-api.git
cd rust-api
