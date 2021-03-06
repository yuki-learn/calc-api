
# 計算機API
式をjsonで渡すと、文字列をパーサで抽象構文木(ATS)に変換し、そこから四則演算を行います。<br>
※小数点の計算には対応していません。

APIは[Heroku](https://jp.heroku.com/)にデプロイ済み(Heroku上でDockerを動かしている)で、動作可能な状態になっています。<br>
(無料枠を使用のため、初回リクエスト時は多少レスポンスが帰ってくるまで時間がかかるかもです)

* ベースURL: https://lit-mesa-21252.herokuapp.com/


### GET: `/expr`
式をjsonで渡し、結果を返す。<br>
四則演算、`()`にも対応。
#### Request
`application/json`

```json
{
    "expr": "4+20/(5*2)"
}
```

#### Response
```
6
```

## 動作確認
`計算機API.postman_collection.json`をPostmanに読み込ませることで動作確認できます。
### 単体テスト
* パーサのテスト(文字列 -> ATSの変換): https://github.com/yuki-learn/calc-api/blob/main/calc-api/src/parser/tests.rs
* ATSのテスト(実際の計算): https://github.com/yuki-learn/calc-api/blob/main/calc-api/src/ast/tests.rs
## Herokuデプロイ
1. `git clone https://github.com/yuki-learn/calculator-api.git`
2. `heroku login`
3. `heroku create`
4. `heroku stack:set container`
5. `git push heroku main`: DockerfileがHeroku上でビルドされデプロイ
## 技術
* Rust
* actix-web
* nom
* Docker
* Heroku

## 開発
「VS Code Remote Containers」を使用しているため、Rustをわざわざインストールする必要がなく、<br>
VS Code上で`.devcontainer`内のファイルを読み込ませることで開発環境をDockerで整え、そのまま開発することができます。
## 参考記事
* [電卓を作る（Parsecによるパーサーの実装）](https://minoki.github.io/ks-material/haskell/parser.html): Haskellの記事だが、四則演算のパーサー、左結合の無限ループを取り除く方法など参考
* [nom によるパーサー実装](https://hazm.at/mox/lang/rust/nom/index.html)
* [Rustで作ったAPIのDocker on Heroku](https://akfm.dev/blog/2020-11-07/rust-api-docker-on-heroku.html)