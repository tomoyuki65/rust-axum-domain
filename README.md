# RustのaxumによるDDD構成のAPIサンプル
Rustのフレームワーク「axum」およびDDD（ドメイン駆動設計）によるバックエンドAPI開発用サンプルです。  
  
<br />
  
## DDDのディレクトリ構成　　
ディレクトリ構成としてはDDDの思想に基づいたレイヤードアーキテクチャを採用しています。  
  
```
/src
 ├── /application（アプリケーション層）
 |    └── usecase（ユースケース層）
 |
 ├── /config（コンフィグ設定）
 |
 ├── /domain（ドメイン層）
 |    ├── model（ドメインモデルの定義。ビジネスロジックは可能な限りドメインに集約させる。）
 |    ├── repository（リポジトリのインターフェース定義）
 |    └── （仮）service（外部サービスのインターフェース定義）
 |
 ├── /infrastructure（インフラストラクチャ層）
 |    ├── database（データベース設定）
 |    ├── logger（ロガーの実装。インターフェース部分はユースケース層で定義。）
 |    ├── persistence（リポジトリの実装。DB操作による永続化層。）
 |    ├── （仮）cache（キャッシュを含めたリポジトリの実装。インターフェースはリポジトリと同一。）
 |    └── （仮）externalapi（外部サービスの実装）
 |
 ├── /presentation（プレゼンテーション層）
 |    ├── handler（ハンドラー層。ルーターで設定したAppStateから対象のユースケースを実行。）
 |    ├── middleware（ミドルウェアの定義）
 |    └── router（ルーター設定。ハンドラーとレジストリのAppStateを利用。）
 |
 └── /registry（レジストリ。依存注入によるユースケースのインスタンスをAppStateにまとめる。）
```
> <span style="color:red">※（仮）のものは将来的に追加する想定の例</span>  
  
</br>
  
### APIの作成手順  
  1. ドメインの定義  
    ドメインを新規追加、または既存のドメインにビジネスロジックの追加。  
    永続化が必要ならリポジトリの定義、外部サービスとの連携が必要ならサービスの定義を追加。 
  
  2. リポジトリやサービスの実装  
    リポジトリやサービスのインターフェース定義を追加した場合、インフラストラクチャ層に実装を定義。  
  
  3. ユースケースの定義  
    ドメインやリポジトリを用いてユースケースにビジネスロジックを定義。
  
  4. レジストリ登録  
    リポジトリ、ユースケースのインスタンスをAppState（アプリケーション全体の状態管理）に登録。  
  
  5. ハンドラーの定義  
    レジストリのAppStateを用いてハンドラーの定義。  
  
  6. ルーター設定の追加  
    ハンドラーとレジストリのAppStateを用いてルーター設定を追加。
  
<br />
  
## 要件
・Rustのバージョンは<span style="color:green">1.87</span>です。  
  
<br />
  
## ローカル開発環境構築
### 1. 環境変数ファイルをリネーム
```
cp ./.env.example ./.env
```  
  
### 2. コンテナのビルドと起動
```
docker compose build --no-cache
docker compose up -d
```  
> <span style="color:red">※テストコードを実行させる際はテスト用の環境変数ファイルを使うため、「docker compose --env-file ./.env.testing up -d」で起動すること。</span>
  
### 3. コンテナの停止・削除
```
docker compose down
```  
  
<br />
  
## コード修正後に使うコマンド
ローカルサーバー起動中に以下のコマンドを実行可能です。  
  
### 1. フォーマット修正
```
docker compose exec api cargo fmt
```  
  
### 2. コード解析チェック
```
docker compose exec api cargo clippy
```  
  
### 3. テストコードの実行
<span style="color:red">事前にテスト用環境変数を設定したローカルサーバーを起動（docker compose --env-file ./.env.testing up -d）してから以下のコマンドを使ってテストを実行して下さい</span>  
```
docker compose exec -e CARGO_TEST=testing api cargo test -- --nocapture --test-threads=1
```  
> ※DBのデータを同期させるため「--test-threads=1」で実行する
  
<br />
  
## 参考記事  
[]()  
  