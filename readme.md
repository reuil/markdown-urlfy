# markdown-urlfy

## 概要

テキストに含まれる URL を Markdown 形式のリンクに変換するツールです。

### 例 1

```markdown:標準入力
https://example.com
```

```markdown:標準出力
[Example Page](https://example.com)
```

### 例 2

```markdown:標準入力
彼のサイト [](https://supertitle.example.com) にはこう書いてありました。
```

```markdown:標準出力
彼のサイト [スーパータイトル](https://sugotitle.example.com) にはこう書いてありました。
```

## 導入

以下のコマンドでビルドします。

```bash
cargo build --release
```

`./target/release/`に実行ファイル`markdown-urlfy`が生成されるので、これをお使いの環境に合わせて適当な場所に配置してください。

## 使用例

### vim

以下のように設定すると、`<c-o>`でカーソル位置の URL を Markdown 形式のリンクに変換できるようになります。

```vim
inoremap <c-o> <Esc>:.!markdown-urlfy <CR>gi
nnoremap <c-o> :.!markdown-urlfy <CR>
```
