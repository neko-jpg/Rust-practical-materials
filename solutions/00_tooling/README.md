# Level 0: ツールチェーン & 習慣化

## 目的
Rust開発における「当たり前」の品質基準とワークフローを身につけます。
ここで作成するクレート `my_utils` は、今後の課題でインポートして使用しても構いません。

## 学習項目

1. **Formatter**: `cargo fmt` を保存時に自動実行する設定を行う。
2. **Linter**: `cargo clippy` を使い、指摘を全て修正する習慣をつける。
3. **Documentation**: `cargo doc --open` でドキュメントを生成・閲覧する。
4. **Testing**: 基本的なユニットテストとドキュメンテーションテストを書く。

## 課題

`src/lib.rs` に以下のユーティリティ関数を実装し、テストとドキュメントを書いてください。

1. **`setup_logger`**: `env_logger` を初期化する関数。
2. **`add`**: 2つの数値を足す関数（ドキュメンテーションテスト付き）。

## 実践コマンド
```bash
# コード整形
cargo fmt

# リントチェック（警告をエラーとして扱う）
cargo clippy -- -D warnings

# テスト実行
cargo test

# ドキュメント生成
cargo doc --open
```
