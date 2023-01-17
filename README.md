# Wikipedia-Word-Counter

wikipedia 日本語版の本文データのタイプ頻度を json で出力するプログラムです。

## 使い方

```=bash
cargo run テキストファイルのパス 結果の出力ファイルのパス
```

例えば、 テキストファイル './data/text.txt' を使った場合の結果を './output/result.json' に出力する場合は

```=bash
cargo run ./data/tetx.txt ./output/result.json
```

を実行することで、結果を出力できます。

## 各種スクリプトについて

`script/` にはデータの整形や抽出のためのスクリプトがあります。

- compare.py
    - 複数データから得られたタイプ頻度を比較するもの
- extract_tetx.py
    - wikiextractor によって得られたテキストファイルから本文のみを出力するもの
- extract.sh
    - [wikiextractor](https://github.com/attardi/wikiextractor) を用いて Wikipedia のダンプを json で出力するもの
    - pip での wikiextractor のインストールが必要です
- format_data.py
    - タイプ頻度の json を降順に並び替えるもの
- output_anasysis.py
    - タイプ頻度の json から上位20位を抽出するもの
    - 同時にタイプ頻度とトークン頻度を標準出力に出力している
- setup.sh
    - venv で仮想環境を構築し、wikiextractor をインストールするもの
    - パスは適宜変更してください
