# the-rust-programming-language

Rust やる

- 教材
  - [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)
  - [Rust 入門 (Zenn, 2021/09 更新)](https://zenn.dev/mebiusbox/books/22d4c1ed9b0003)
  - [RustCoder ―― AtCoder と Rust で始める競技プログラミング入門 (2022/05 更新)](https://zenn.dev/toga/books/rust-atcoder-old)
- 勉強メモ
  - https://zenn.dev/pyteyon/scraps/4e04a66ce38074

## 開発環境

- macOS 12.5.1
- rustup 1.25.1 (bb60b1e89 2022-07-12)
- rustc 1.66.0 (69f9c33d7 2022-12-12)
- cargo 1.66.0 (d65d197ad 2022-11-15)

## 目次

- [x] はじめに
- [x] 1. 事始め
  - 1.1. インストール
  - 1.2. Hello, World!
  - 1.3. Hello, Cargo!
- [x] 2. 数当てゲームのプログラミング
- [x] 3. 一般的なプログラミングの概念
  - 3.1. 変数と可変性
  - 3.2. データ型
  - 3.3. 関数
  - 3.4. コメント
  - 3.5. 制御フロー
- [ ] 4. 所有権を理解する
- [ ] 5. 構造体を使用して関係のあるデータを構造化する
- [ ] 6. Enum とパターンマッチング
- [ ] 7. 肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する
- [ ] 8. 一般的なコレクション
- [ ] 9. エラー処理
- [ ] 10. ジェネリック型、トレイト、ライフタイム
- [ ] 11. 自動テストを書く
- [ ] 12. 入出力プロジェクト：コマンドラインプログラムを構築する
- [ ] 13. 関数型言語の機能：イテレータとクロージャ
- [ ] 14. Cargo と Crates.io についてより詳しく
- [ ] 15. スマートポインタ
- [ ] 16. 恐れるな！並行性
- [ ] 17. Rust のオブジェクト指向プログラミング機能
- [ ] 18. パターンとマッチング
- [ ] 19. 高度な機能
- [ ] 20. 最後のプロジェクト：マルチスレッドの Web サーバを構築する
- [ ] 21. 付録
  - [ ] 21.1. 付録 A：キーワード
  - [ ] 21.2. 付録 B：演算子と記号
  - [ ] 21.3. 付録 C：導出可能なトレイト
  - [ ] 21.4. 付録 D：便利な開発ツール
  - [x] 21.5. 付録 E：エディション
  - [ ] 21.6. 付録 F：本の翻訳
  - [ ] 21.7. 付録 G：Rust の作られ方と“Nightly Rust”

## 用語

- rustup
  - Rust のバージョンと関連するツールを管理する CLI ツール
- cargo
  - Rust のパッケージマネージャ兼ビルドシステム
- クレート crate
  - Cargo によって管理される Rust のパッケージのこと。Python や Node.js などのパッケージの Rust 版の呼び方。
- `Cargo.toml`
  - Cargo プロジェクトの設定ファイル
- `Cargo.lock`
  - Cargo プロジェクトの依存関係のバージョンを記録、固定するためのファイル。`cargo build` を初めて実行したときに Cargo プロジェクトのルートディレクトリに生成される。手動で変更する必要はない。

## 使ったコマンド

```sh
# rustc
rustc main.rs

# cargo
cargo new [project name]
cargo init
cargo build           # 開発用ビルド（最適化なし、ビルド時間短い、実行速度遅い）
cargo build --release # リリース用ビルド（最適化あり、ビルド時間長い、実行速度速い）
cargo run
cargo run --release
cargo check
cargo update
```
