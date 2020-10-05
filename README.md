# hello_serverless_rust_world

ServerlessFramework を用いた Rust 開発のサンプルリポジトリ。

## Requirements

- Rust
- npm

## ディレクトリ作成

プロジェクトディレクトリを作成し、ServerlessFramework 関係のライブラリをインストールする。
Rust のマニフェストファイルである `Cargo.toml` を作成する。

```bash
$ mkdir {project_dir} && cd $_
$ npm init
$ npm install serverless-rust serverless --save
$ touch serverless.yml
$ touch Cargo.toml
```

ServerlessFramework の設定を記述する。
その他の設定は [Reference](https://www.serverless.com/framework/docs/providers/aws/guide/serverless.yml/#serverlessyml-reference/)を参照。

```yaml
service: hello-serverless-rust-world
provider:
  name: aws
  runtime: rust
plugins:
  - serverless-rust
package:
  individually: true
```

Rust の設定を記述する。
各エンドポイント別にクレート（Rust のライブラリを表す単位）を作成していくので、 [Cargo.toml] は以下のように記述しておく。

```toml
[workspace]
members = []
```

## ユーザ一覧を取得するエンドポイント

### クレートの作成

まずはじめに Users クレートを作成する。
Rust のパッケージマネージャ Cargo を利用し、users クレートを作成する。

```bash
$ cargo new users
```

users ディレクトリが作成され、以下のようなファイル構成になっている。

```
users
├── Cargo.toml
└── src
    └── main.rs
```

プロジェクトディレクトリにある [Cargo.toml](./Cargo.toml)に users クレートをワークスペースとして追加する。

```toml
[workspace]
members = [
    "users"
]
```

users クレートに必要なライブラリを [Cargo.toml](./users/Cargo.toml)に追加しビルドする。

```toml
[dependencies]
lambda_runtime = "0.2.1"
lambda_runtime_errors = "0.1.1"
lambda_http = "0.1.1"
serde = { version = "1.0.116", features = ["derive"] }
serde_json = "1.0"
http = "0.1.1"
log = "0.4.11"
simple_logger = "1.10.0"
```

### ハンドラの作成

[main.rs](./users/src/main.rs)にハンドラを記述する。
HTTP メソッドが GET 以外の場合は、ステータスコード 405 を返す。
GET の場合はユーザの一覧を取得する。

```rust
use std::error::Error;

use http::{Response, StatusCode};
use lambda_http::{lambda, Body, IntoResponse, Request};
use lambda_runtime::Context;
use lambda_runtime_errors::HandlerError;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger;
use simple_logger::SimpleLogger;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    username: String,
    email: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init()?;
    lambda!(routes);
    Ok(())
}

fn routes(req: Request, con: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "GET" => get_user(req, con),
        _ => {
            log::error!("Method not allowed");
            let res = Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::Text("Method not allowed".to_string()))
                .unwrap();

            Ok(res)
        }
    }
}

fn get_user(_: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    let users = vec![
        User {
            username: "test_user1".to_string(),
            email: "example1@example.com".to_string(),
        },
        User {
            username: "test_user2".to_string(),
            email: "example2@example.com".to_string(),
        },
    ];
    Ok(serde_json::json!(users).into_response())
}
```

### ハンドラの登録

[serverless.yml](./serverless.yml) に作成したハンドラを登録するため、以下を追記する。
`handler` の値は作成したクレート名と同じにする。

```yaml
functions:
  users:
    handler: users
    events:
      - http:
          path: /users
          method: get
```

### リクエストファイルの作成

ローカルテスト用のリクエストファイルを作成します。
単なる GET リクエストの場合でもローカル実行の際にこのファイルが必要になる。

```bash
$ mkdir -p users/test/resources && cd $_
$ touch get_users_request.json
```

```json
{
  "path": "/users",
  "httpMethod": "GET",
  "headers": {
    "Host": "example.com"
  },
  "requestContext": {
    "accountId": "",
    "resourceId": "",
    "stage": "development",
    "requestId": "",
    "identity": {
      "sourceIp": ""
    },
    "resourcePath": "",
    "httpMethod": "",
    "apiId": ""
  },
  "queryStringParameters": null
}
```

### ローカル実行

`sls invoke local` コマンドを実行し、users ハンドラにローカルでリクエスト送信する。

```bash
$ sls invoke local -f users --path users/test/resources/get_users_request.json
  Serverless: Running "serverless" installed locally (in service node_modules)
  Serverless: Configuration warning at 'provider.runtime': should be equal to one of the allowed values [dotnetcore2.1, dotnetcore3.1, go1.x, java11, java8, java8.al2, nodejs10.x, nodejs12.x, provided, provided.al2, python2.7, python3.6, python3.7, python3.8, ruby2.5, ruby2.7]
  Serverless:
  Serverless: Learn more about configuration validation here: http://slss.io/configuration-validation
  Serverless:
  Serverless: Building Rust users func...
  Serverless: Running containerized build
      Finished release [optimized] target(s) in 1.68s
  objcopy: stxouIL0: debuglink section already exists
    adding: bootstrap (deflated 60%)
  Serverless: Packaging service...
  Serverless: Downloading base Docker image...
  START RequestId: 5d165a51-9fbb-1374-2cb2-38ebb4f415e6 Version: $LATEST
  2020-10-05 13:56:33,963 INFO  [lambda_runtime_core::runtime] Received new event with AWS request id: 5d165a51-9fbb-1374-2cb2-38ebb4f415e6
  2020-10-05 13:56:33,965 INFO  [lambda_runtime_core::runtime] Response for 5d165a51-9fbb-1374-2cb2-38ebb4f415e6 accepted by Runtime API
  END RequestId: 5d165a51-9fbb-1374-2cb2-38ebb4f415e6
  REPORT RequestId: 5d165a51-9fbb-1374-2cb2-38ebb4f415e6  Init Duration: 37.37 ms Duration: 3.94 ms       Billed Duration: 100 ms Memory Size: 1024 MB    Max Memory Used: 11 MB

  {"statusCode":200,"headers":{"content-type":"application/json"},"multiValueHeaders":{"content-type":["application/json"]},"body":"[{\"email\":\"example1@example.com\",\"username\":\"test_user1\"},{\"email\":\"example2@example.com\",\"username\":\"test_user2\"}]","isBase64Encoded":false}
```

## ユーザを新規作成するエンドポイント

users クレートにユーザを新規作成するエンドポイント `[POST] /users` を追加する。

### ServerlessFramework

[./serverless.yml]に以下の設定を追記する。

```yaml
- http:
  path: /users
  method: post
```

### POST ハンドラの追加

ユーザを新規作成する関数を[main.rs](./users/src/main.rs)に追記する。
HTTP メソッドが POST の場合に、body に与えられた JSON 文字列を Serialize し、ステータスコード 201 を返す。

```rust
fn create_user(req: Request, _: Context) -> Result<Response<Body>, HandlerError> {
    match serde_json::from_slice::<User>(req.body().as_ref()) {
        Ok(user) => {
            let res = Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::Text(serde_json::json!(user).to_string()))
                .unwrap();
            Ok(res)
        }
        Err(e) => {
            log::error!("error {}", e);
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Bad request".into())
                .expect("err creating response"))
        }
    }
}
```

また、メソッドの判定を行っている `routes` 関数を以下のように変更する。

```rust
fn routes(req: Request, con: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "POST" => create_user(req, con),
        "GET" => get_user(req, con),
        _ => {
            log::error!("Method not allowed");
            let res = Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::Text("Method not allowed".to_string()))
                .unwrap();

            Ok(res)
        }
    }
}
```

### リクエストファイルの作成

ローカル実行用のリクエストファイル [post_users_request.json](./users/test/resources/post_users_request.json)を作成する。
GET のときと異なる点として、 `body` プロパティを追加している。

```json
{
  "path": "/users",
  "httpMethod": "POST",
  "headers": {
    "Host": "example.com"
  },
  "requestContext": {
    "accountId": "",
    "resourceId": "",
    "stage": "development",
    "requestId": "",
    "identity": {
      "sourceIp": ""
    },
    "resourcePath": "",
    "httpMethod": "",
    "apiId": ""
  },
  "queryStringParameters": null,
  "body": "{\"username\":\"new user\", \"email\": \"new_user@example.com\"}"
}
```

### ローカル実行

`sls invoke local` コマンドを実行し、users ハンドラにローカルでリクエスト送信する。

```bash
$ sls invoke local -f users --path users/test/resources/post_users_request.json
Serverless: Running "serverless" installed locally (in service node_modules)
Serverless: Configuration warning at 'provider.runtime': should be equal to one of the allowed values [dotnetcore2.1, dotnetcore3.1, go1.x, java11, java8, java8.al2, nodejs10.x, nodejs12.x, provided, provided.al2, python2.7, python3.6, python3.7, python3.8, ruby2.5, ruby2.7]
Serverless:
Serverless: Learn more about configuration validation here: http://slss.io/configuration-validation
Serverless:
Serverless: Building Rust users func...
Serverless: Running containerized build
   Compiling users v0.1.0 (/code/users)
    Finished release [optimized] target(s) in 25.29s
  adding: bootstrap (deflated 60%)
Serverless: Packaging service...
Serverless: Downloading base Docker image...
START RequestId: cfff52df-03df-10e8-6c1d-4f9aeee03a74 Version: $LATEST
2020-10-05 14:14:21,236 INFO  [lambda_runtime_core::runtime] Received new event with AWS request id: cfff52df-03df-10e8-6c1d-4f9aeee03a74
2020-10-05 14:14:21,237 INFO  [lambda_runtime_core::runtime] Response for cfff52df-03df-10e8-6c1d-4f9aeee03a74 accepted by Runtime API
END RequestId: cfff52df-03df-10e8-6c1d-4f9aeee03a74
REPORT RequestId: cfff52df-03df-10e8-6c1d-4f9aeee03a74  Init Duration: 44.84 ms Duration: 2.67 ms       Billed Duration: 100 ms Memory Size: 1024 MB    Max Memory Used: 11 MB

{"statusCode":201,"headers":{},"multiValueHeaders":{},"body":"{\"email\":\"new_user@example.com\",\"username\":\"new user\"}","isBase64Encoded":false}
```
