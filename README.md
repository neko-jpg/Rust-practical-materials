# Rust Engineering Bootcamp

このカリキュラムは、Rustの文法を理解したエンジニアが、**「実務で通用する高品質なソフトウェア」** を設計・実装できるようになるための統合プログラムです。

## カリキュラム構造

### Phase 1: Core (必修)
Rustエンジニアとしての「地力」と「品質基準」を作る一本道です。

- **Level 0: ツールチェーン & 習慣化** (`00_tooling`)
    - 開発環境、整形、Lint、ドキュメント生成、共通ライブラリ作成。
- **Level 1: CLIツール & 設計** (`01_mini_grep`)
    - 構造化、エラー設計 (`thiserror`), 引数解析 (`clap`), ベンチマーク。
- **Level 2: HTTPサーバー & 低レイヤ** (`02_single_threaded_server`)
    - 標準ライブラリ縛り、ルーティング自作、堅牢なパーサ実装。
- **Level 3: 非同期処理 & 並行性** (`03_async_chat`)
    - `tokio` 詳細、Graceful Shutdown、Backpressure、アクターパターン。
- **Level 4: 実践 REST API** (`04_rest_api`)
    - `Axum` + `SQLx`。認証、OpenAPI、レイヤーアーキテクチャ、DBマイグレーション。
- **Level 5: テスト戦略 & CI** (`05_testing`)
    - Unit/Integration/E2E、Mock、Property-based testing、GitHub Actions。
- **Level 6: 可観測性 (Observability)** (`06_observability`)
    - 構造化ログ (`tracing`)、メトリクス、分散トレーシング、エラー監視。

### Phase 2: Tracks (選択)
Core修了後、興味のある専門領域を深掘りします。(`tracks/` ディレクトリ)

- **Track A: Web & Backend** (Redis, Message Queue, GraphQL, gRPC)
- **Track B: Systems Network** (Multi-thread Server, Lock設計, Unsafe)
- **Track C: Performance** (Profiling, Memory Optimization)
- **Track D: Data Engineering** (Batch, Stream, Parquet/Arrow)

### Phase 3: Capstone (卒業制作)
- **Integrated System** (`99_capstone`)
    - これまでの全知識を総動員した、履歴書に書けるレベルの統合システム構築。

---

## 進め方と合格基準

各Levelには明確な「合格基準」があります。これを満たしてから次へ進んでください。

1. **Test**: `cargo test` が全てパスすること。
2. **Lint**: `cargo clippy -- -D warnings` で警告が出ないこと。
3. **Format**: `cargo fmt` が適用されていること。
4. **Docs**: 公開APIにはドキュメントコメント (`///`) があること。

**Let's build reliable software.**
