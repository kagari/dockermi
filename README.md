# dockermi
docker imageを選択して消せる（だけ）

## 前準備
バイナリファイルをパスの通ったディレクトリに配置してください。  
`$ cp ./target/x86_64-apple-darwin/debug/dockermi /usr/local/bin/`

debug版なのでrelease版を使用したい場合はrustでのbuildが必要です...

## 使い方
`dockermi` を実行したら下のように `docker images` を実行した場合と似た画面が出てきます。
```
    REPOSITORY      TAG                 IMAGE ID            CREATED             SIZE
[ ] ubuntu          latest              a2a15febcdf3        12 days ago         64.2MB
[ ] hello-world     latest              fce289e99eb9        7 months ago        1.84kB
...
```
使用できるキー
- `j` , `k` で上下移動
- `x` で選択・選択解除
- `q`, または `Ctrl + c` で終了
- `Enter` で削除
