# wav_analyzer

WAVファイルの基本情報とラウドネス指標(ピーク値・RMS)を解析するコマンドラインツールです。

## 概要

指定したWAVファイルを読み込み、以下の情報を表示します。

- サンプルレート
- チャンネル数
- ビット深度
- サンプル数
- ピーク値(絶対値の最大サンプル値)
- RMS(実効値、音量の平均的な大きさの指標)

出力形式はテキストとJSONの2種類に対応しています。

## 使い方

### ビルド

```bash
git clone https://github.com/ユーザー名/wav_analyzer.git
cd wav_analyzer
cargo build --release
```

### 実行

```bash
# テキスト形式(デフォルト)
cargo run -- path/to/file.wav

# JSON形式
cargo run -- path/to/file.wav --format json
```

### 出力例

**テキスト形式**

```
サンプルレート: 44100
チャンネル数: 1
サンプル深度: 16
サンプル数: 91334
ピーク値: 12158
RMS: 1226.6710641225209
```

**JSON形式**

```json
{
  "sample_rate": 44100,
  "channels": 1,
  "bits_per_sample": 16,
  "samples_count": 91334,
  "peek": 12158,
  "rms": 1226.6710641225209
}
```

## 使用技術

- [Rust](https://www.rust-lang.org/)
- [hound](https://crates.io/crates/hound) — WAVファイルの読み込み
- [clap](https://crates.io/crates/clap) — コマンドライン引数のパース

## 実装のポイント

- `Result`と`?`演算子によるエラーハンドリングを一貫して採用し、ファイルが存在しない・破損している場合もパニックせず、わかりやすいエラーメッセージを表示するようにしています。
- 解析結果を構造体(`AnalysisResult`)にまとめることで、「計算処理」と「表示処理(テキスト/JSON)」の責務を分離しています。
- `clap`の`derive`機能を使い、コマンドライン引数の定義を宣言的に記述しています。

## 今後の展望

- 複数ファイルの一括処理
- WAV以外の音声フォーマット対応
- 波形の簡易表示(ターミナル上でのアスキーアート表示など)

## ライセンス

MIT
