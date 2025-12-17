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
```

### 2) sesuaikan config db file .env pada dengan config db mu
```
DATABASE_URL=postgres://{username}:{password}@{host}:5432/{nama_db}
```

### 3) Build
```
cargo build
```

### 4) Run Project
```
cargo run
```

## API Requests (requests.http)
Repo ini menyediakan file **`requests.http`** untuk mencoba endpoint tanpa Postman.
Kalau kamu pakai VS Code:

1. Install extension **REST Client** (by Huachao Mao)
2. Buka file `requests.http`
3. Klik **Send Request** di atas tiap request
   
### Daftar endpoint di `requests.http`

- ** GET http://localhost:8000/  
  Health check untuk memastikan server hidup.
<p>
  <img width="664" height="270" alt="image" src="https://github.com/user-attachments/assets/4cd82d09-6753-43a1-b50e-2decebf5b99f" />
</p>
  

  
