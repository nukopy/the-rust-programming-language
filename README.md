# the-rust-programming-language

Rust "The Book" やる

- 書籍
  - [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/)
  - [The Rust Programming Language 原著](https://doc.rust-lang.org/book/title-page.html)
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
- [x] 4. 所有権を理解する
  - 4.1. 所有権とは？
  - 4.2. 参照と借用
  - 4.3. スライス型
- [x] 5. 構造体を使用して関係のあるデータを構造化する
  - 5.1. 構造体を定義し、インスタンス化する
  - 5.2. 構造体を使ったプログラム例
  - 5.3. メソッド記法
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
- [x] 16. 恐れるな！並行性（[日](https://doc.rust-jp.rs/book-ja/ch16-00-concurrency.html#%E6%81%90%E3%82%8C%E3%82%8B%E3%81%AA%E4%B8%A6%E8%A1%8C%E6%80%A7) | [英](https://doc.rust-lang.org/book/ch16-00-concurrency.html)）
  - 16.1. スレッドを使用してコードを同時に走らせる
  - 16.2. メッセージ受け渡しを使ってスレッド間でデータを転送する
  - 16.3. 状態共有並行性
  - 16.4. Sync と Send トレイトで拡張可能な並行性
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

## Commands

```sh
# rustc
rustc main.rs

# initialize Cargo project
cargo new [project name]
cargo init

# build
cargo build           # 開発用ビルド（最適化なし、ビルド時間短い、実行速度遅い）
cargo build --release # リリース用ビルド（最適化あり、ビルド時間長い、実行速度速い）

# run (build & run)
cargo run
cargo run --release

# compiler check without generate binary
cargo check

# run test
cargo test

# update dependencies
cargo update

# watch (hot reload)
cargo install cargo-watch
cargo watch -x run
```
