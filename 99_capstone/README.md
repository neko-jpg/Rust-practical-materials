# Phase 3: Capstone Project (卒業制作)

## 概要
これまでに学んだ全ての技術（Core + 選択したTrack）を統合し、実務で通用するレベルの完全なシステムを構築します。

## 課題例: リアルタイム分散タスク処理システム

以下の要件を満たすシステムを構築してください。

### Architecture
1. **API Gateway (Axum)**: REST APIでタスクを受け付ける。認証付き。
2. **Worker Nodes (Tokio)**: バックグラウンドでタスクを処理する。複数台構成を想定。
3. **Queue (Redis/SQL)**: タスクの永続化と分散。
4. **Dashboard (Web)**: WebSocketを使ってリアルタイムに進捗を表示。

### Requirements
- **Observability**: `tracing` でリクエストから処理完了までの分散トレースが見れること。
- **Reliability**: Workerがクラッシュしてもタスクがロストしないこと（Graceful Shutdown & Retry）。
- **CI/CD**: GitHub Actionsでテストとビルドが自動化されていること。

**Good Luck.**
