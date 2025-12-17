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
<p>
  <img width="664" height="270" alt="image" src="https://github.com/user-attachments/assets/4cd82d09-6753-43a1-b50e-2decebf5b99f" />
</p>

- ** POST http://localhost:8000/products
Content-Type: application/json

{
  "name": "Teh Pucuk",
  "category": "Minuman",
  "qty": 5,
  "price" : 7500
}
<p>
   <img width="669" height="597" alt="image" src="https://github.com/user-attachments/assets/dfa04087-76a7-48dd-9b7d-2b73df3fee3e" />
</p>

- ** GET http://localhost:8000/products
<p>
  <img width="672" height="946" alt="image" src="https://github.com/user-attachments/assets/389698b1-b4ed-40b2-a234-a938fee70e9e" />
</p>

- ** GET http://localhost:8000/products/11
<p>
  <img width="665" height="582" alt="image" src="https://github.com/user-attachments/assets/8c894b8d-2f27-4b87-9b5c-67eebf1c436d" />
</p>

- ** PUT http://localhost:8000/products/9
Content-Type: application/json

{
  "name": "Teh Pucuk",
  "category": "Minuman",
  "qty": 5,
  "price" : 7500
}
<p>
  <img width="661" height="585" alt="image" src="https://github.com/user-attachments/assets/740378b6-2f62-400f-b4ee-526c7483a4d7" />
</p>

- ** DELETE http://localhost:8000/products/11
<p>
  <img width="673" height="257" alt="image" src="https://github.com/user-attachments/assets/078ac825-142a-4ad9-ac99-c135677f0557" />
</p>
  
- ** PUT http://localhost:8000/products/soft-delete/11
<p>
  <img width="670" height="571" alt="image" src="https://github.com/user-attachments/assets/0499bcb7-b38a-4850-bbbe-0d7a22b9e7b8" />
</p>
