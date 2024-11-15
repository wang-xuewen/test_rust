在Rust中构建一个具有服务端和客户端的聊天系统，可以利用workspace来组织项目结构。workspace可以让你管理多个相互关联的crate（类似子项目），便于开发和维护。

以下是一个步骤指南：

### 1. 创建 Workspace

首先，创建一个新的Rust workspace目录：

```bash
cargo new chat_workspace --bin
cd chat_workspace
```

在根目录下的`Cargo.toml`文件中，修改内容以支持workspace：

```toml
[workspace]
members = ["server", "client"]
```

### 2. 创建 Server 和 Client 项目

然后，我们在workspace下创建`server`和`client`两个crate：

```bash
cargo new server --bin
cargo new client --bin
```

这会在`chat_workspace`目录中创建两个子目录：`server`和`client`，每个子目录都有自己的`Cargo.toml`和`src`目录。

### 3. 命名建议

对于聊天系统的这种结构，建议使用以下命名：

- **Workspace 名称**：`chat_workspace`，整体项目名称，可以改成和项目匹配的更合适的名称。
- **服务端名称**：`server`，代表服务器端应用，负责处理消息的转发和管理连接。
- **客户端名称**：`client`，代表客户端应用，是一个命令行工具，类似于`redis-cli`，用于连接到服务器端并进行聊天。

### 4. 配置 Workspace 的 Cargo.toml 文件

确保你的`Cargo.toml`文件在根目录中看起来类似以下内容：

```toml
[workspace]
members = ["server", "client"]
```

然后，在`server/Cargo.toml`和`client/Cargo.toml`中，定义它们各自的依赖项。比如，服务端可能需要一个WebSocket或TCP的库来管理连接。

### 5. 基本实现示例

接下来，我们可以为`server`和`client`实现基本功能：

#### Server (`server/src/main.rs`)

这是一个简单的TCP服务端，负责接收客户端的消息并转发给所有已连接的客户端。可以使用`tokio`和`tokio::net::TcpListener`来实现异步的TCP处理。

```rust
// server/src/main.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                    }
                }
            }
        });
    }
}
```

#### Client (`client/src/main.rs`)

这是一个简单的TCP客户端，用于连接到服务端并发送消息。可以通过`tokio::net::TcpStream`连接服务器，并在命令行中进行输入。

```rust
// client/src/main.rs
use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").await?;
    let (reader, mut writer) = stream.into_split();

    let mut stdin_reader = BufReader::new(io::stdin()).lines();
    let mut server_reader = BufReader::new(reader).lines();

    loop {
        tokio::select! {
            Ok(line) = stdin_reader.next_line() => {
                if let Some(line) = line {
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
            Ok(line) = server_reader.next_line() => {
                if let Some(line) = line {
                    println!("Server: {}", line);
                }
            }
        }
    }
}
```

### 6. 运行

在workspace根目录下，可以运行服务端和客户端：

```bash
# 启动服务端
cargo run -p server

# 启动客户端
cargo run -p client
```

### 7. 测试

- 启动服务端后，运行多个客户端实例（可以在多个终端窗口中执行`cargo run -p client`），每个客户端都可以向服务端发送消息。
- 服务端将消息广播给所有连接的客户端，实现了简单的聊天功能。

这就是一个简单的命令行聊天应用的基本架构。









### 方案 2：使用第三方库 `log4rs` 自动管理日志轮转

`log4rs` 是一个功能更强大的日志库，它内置了日志轮转和自动保留策略，能够实现按日期回滚并自动保留最近的日志。

#### 1. 添加 `log4rs` 依赖：

在 `Cargo.toml` 中添加 `log4rs`：

```toml
[dependencies]
log = "0.4"
log4rs = "1.0"
chrono = "0.4"
```

#### 2. 配置 `log4rs`：

`log4rs` 允许通过配置文件来设置日志轮转和日志保留策略。以下是一个按日期回滚并保留最近 7 天日志的配置示例：

```yaml
# log4rs.yml
appenders:
  - type: rolling_file
    path: "log/log_{date}.log"
    policy:
      type: compound
      trigger:
        type: size
        limit: 10MB
      rollover:
        type: time
        pattern: "%Y-%m-%d"
    encoder:
      type: pattern
      pattern: "[{d}] [{l}] {m}\n"

root:
  level: info
  appenders:
    - rolling_file
```

#### 3. 在 `main.rs` 中加载配置：

```rust
use log::{info, warn};
use log4rs;

fn main() {
    // 加载 log4rs 配置
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // 输出日志
    info!("This is an info message.");
    warn!("This is a warning message.");
}
```

### 结果：

- **`log4rs`** 会根据配置文件生成按日期回滚的日志文件，并且支持文件大小限制、日志过期清理等功能。
- **自动保留 7 天日志**：通过配置文件中的时间轮转和触发策略，`log4rs` 可以自动管理日志文件并删除过期日志。

### 总结：

- **`fern`**：不直接支持自动保留 7 天日志，但可以通过编写清理逻辑实现。
- **`log4rs`**：提供了更强大的日志管理功能，包括日志文件的轮转、大小限制以及按时间保留日志，适合需要自动保留一定天数日志的场景。

如果你希望使用现成的日志轮转和清理功能，推荐使用 `log4rs`。如果你倾向于自己实现日志清理逻辑，`fern` 配合 `chrono` 也能满足需求。
