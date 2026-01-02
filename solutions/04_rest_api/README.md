# Level 4: 本格的なWebバックエンド

Rust のモダンな Web フレームワーク `Axum` と、非同期 SQL クエリビルダー `SQLx` を組み合わせて、
実用的な REST API サーバーを構築します。

## 学習カリキュラム

### 4.1: REST APIの基本 (Axum)
- **Goal**: HTTP メソッド（GET, POST）に応答し、JSON を送受信する API を作成する。
- **Key Concepts**:
    - `Router`: URL パスとハンドラの紐付け。
    - `Extractor`: リクエストからデータを取り出す仕組み (`Json`, `Path`, `State`)。
    - `Layer`: ミドルウェア（ロギングなど）の適用。

### 4.3: データベースの永続化 (SQLx)
- **Goal**: インメモリではなく、SQLite データベースにデータを永続化する。
- **Key Concepts**:
    - `SqlitePool`: データベース接続プールによる効率的な接続管理。
    - `FromRow`: SQL の結果を Rust の構造体に自動マッピング。
    - 非同期クエリ実行。

## 実行方法

1. **サーバーの起動**:
   ```bash
   cargo run
   ```
   ※ 初回起動時に自動的に `products.db` が作成され、初期データが投入されます。

2. **動作確認**:
   
   - **商品一覧の取得**:
     ```bash
     curl http://localhost:3000/products
     ```
   
   - **商品の作成**:
     ```bash
     curl -X POST -H "Content-Type: application/json" -d '{"name": "New Rust Book", "price": 5000}' http://localhost:3000/products
     ```

   - **特定商品の取得**:
     ```bash
     curl http://localhost:3000/products/1
     ```