# Shorekeeper

![Screenshot](https://git.xeondev.com/Shorekeeper/Shorekeeper/raw/branch/master/screenshot.png)

## About
**Shorekeeper is an open-source Wuthering Waves server emulator written in Rust**. The goal of this project is to ensure a clean, easy-to-understand code environment. Shorekeeper uses **tokio** for asynchronous networking operations, **axum** as http framework and **ZeroMQ** for communication between servers. It also implements **performant and extensible ECS** for emulation of the game world.

## Getting started
#### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)
- [Protoc](https://github.com/protocolbuffers/protobuf/releases) (for protobuf codegen)

#### Setup
##### a) building from sources

```sh
git clone https://git.xeondev.com/Shorekeeper/Shorekeeper.git
cd Shorekeeper
cargo run --bin config-server
cargo run --bin hotpatch-server
cargo run --bin login-server
cargo run --bin gateway-server
cargo run --bin game-server
```

##### b) using pre-built binaries
Navigate to the [Releases](https://git.xeondev.com/Shorekeeper/Shorekeeper/releases)
page and download the latest release for your platform.<br>
Launch all servers: `config-server`, `hotpatch-server`, `login-server`, `gateway-server`, `game-server`

##### NOTE: you don't have to install Rust and Protoc if you're going to use pre-built binaries, although the preferred way is building from sources.<br>We don't provide any support for pre-built binaries.

#### Configuration
You should configure each server using their own config files. They're being created in current working directory upon first startup.

##### Database section
You have to specify credentials for **PostgreSQL**<br>
###### An example of database configuration:
```
[database]
host = "localhost:5432"
user_name = "postgres"
password = ""
db_name = "shorekeeper"
```
##### NOTE: don't forget to create database with specified `db_name` (default: `shorekeeper`). For example, you can do so with PgAdmin.

#### Data
The data files: Logic JSON collections (`assets/logic/json`) and config/hotpatch indexes (`assets/config`, `assets/hotpatch`) are included in this repository. Keep in mind that you need to have the `assets` subdirectory in current working directory.

#### Connecting
You have to download client of Wuthering Waves Beta 1.3, apply the [shorekeeper-patch](https://git.xeondev.com/xeon/shorekeeper-patch/releases) and add necessary `.pak` files, which you can get here: [shorekeeper-pak](https://git.xeondev.com/Shorekeeper/shorekeeper-pak)

### Troubleshooting
[Visit our discord](https://discord.gg/reversedrooms) if you have any questions/issues

### Support
If you want to support this project, feel free to [send a tip via boosty](https://boosty.to/xeondev/donate)