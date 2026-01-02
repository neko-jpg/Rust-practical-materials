CREATE TABLE IF NOT EXISTS products (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price INTEGER NOT NULL
);

INSERT INTO products (name, price) VALUES ('Rust Book', 4500);
INSERT INTO products (name, price) VALUES ('Mechanical Keyboard', 12000);
