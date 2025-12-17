CREATE TABLE IF NOT EXISTS product (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    category VARCHAR(100) NOT NULL,
    qty INT,                        
    price NUMERIC NOT NULL DEFAULT 0,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    created_by VARCHAR(100) NOT NULL DEFAULT 'SYSTEM',
    created_date TIMESTAMPTZ NOT NULL DEFAULT now(),
    modified_by VARCHAR(100) NOT NULL DEFAULT 'SYSTEM',
    modified_date TIMESTAMPTZ NOT NULL DEFAULT now()
);
