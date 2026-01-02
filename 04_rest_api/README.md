# Level 4: 本格的 REST API (Product API)

## 課題
Rustの実務開発で標準的に使われる技術スタック (`Axum` + `SQLx` + `Serde`) を用いて、製品情報を管理するREST APIを作成します。
永続化には SQLite を使用します。

## 学習目標
- **Webフレームワーク (Axum)**: ルーティング、抽出器 (Extractors)、レスポンス生成。
- **データベース (SQLx)**: コネクションプール、非同期クエリ実行、マイグレーション。
- **データモデリング**: 構造体とDBテーブルのマッピング。
- **レイヤーアーキテクチャ**: ハンドラ、モデル、DB操作の分離。

## 実行準備

1. `sqlx-cli` をインストールします（DB作成用）:
   ```bash
   cargo install sqlx-cli --no-default-features --features native-tls,sqlite
   ```
2. `.env` ファイルを作成し、DATABASE_URLを設定します:
   ```text
   DATABASE_URL=sqlite:products.db
   ```
3. データベースを作成します:
   ```bash
   sqlx database create
   ```
4. マイグレーションを実行します（テーブル作成）:
   - まずマイグレーションファイルを作成: `sqlx migrate add create_products_table`
   - 生成されたsqlファイルに `CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT NOT NULL, price INTEGER NOT NULL);` を記述
   - 実行: `sqlx migrate run`

## 実装ステップ

1. `src/main.rs`: `tokio`ランタイムのセットアップ、DBプールの作成、ルーターの定義。
2. `src/model.rs`: `Product` 構造体の定義と、`CreateProduct` DTO (Data Transfer Object) の定義。
3. `src/handler.rs`:
    - `create_product`: POSTリクエストを受け取りDBに保存。
    - `list_products`: 全件取得してJSONで返す。
    - `get_product`: ID指定で取得。

## API エンドポイント例
- `POST /products`: `{"name": "Rust Book", "price": 3000}`
- `GET /products`: 製品一覧を取得
- `GET /products/:id`: 指定IDの製品を取得

## Advanced Challenges (発展課題)

### 1. 入力バリデーション (`validator`)
- `CreateProduct` 構造体にバリデーションを追加してください（例: `name` は1文字以上、`price` は正の整数）。
- バリデーションエラー時に `400 Bad Request` と詳細なメッセージを返すようにしてください。

### 2. APIドキュメント (`utoipa`)
- OpenAPI (Swagger) 定義をコードから生成し、`/swagger-ui` で閲覧できるようにしてください。
- フロントエンド開発者にとって、API仕様書は必須です。

### 3. エラーハンドリングの共通化
- 全てのハンドラのエラー型を統一し、`IntoResponse` を実装したカスタムエラー型を作成してください。
- DBエラー、バリデーションエラー、Not Foundなどを適切にHTTPステータスコードにマッピングしてください。
