<center>
    <h1 align="center">OPVM</h1>
    <h3 align="center">Your Ultimate OptiFine Version Manager</h3>
    <p align="center">Effortlessly Download, Apply, and Manage OptiFine Versions for an Enhanced Minecraft Experience</p>
</center>

![MIT License](https://img.shields.io/badge/License-MIT-blue.svg)
![Platform Support](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows%20%7C%20macOS-blue)
![Status](https://img.shields.io/badge/status-stable-brightgreen)
![Build Status](https://github.com/zzzJe/ropvm/actions/workflows/build-and-release.yml/badge.svg)

### Introduction

<img alt="Welcome to OPVM" src="https://raw.githubusercontent.com/zzzJe/ropvm/vhs-demo-generate/assets/demo.gif" width="600" />

OPVM is a command-line tool designed to simplify the management of OptiFine files. It allows you to search, download, and manage OptiFine versions effortlessly—all without ads.

Whether you're a casual Minecraft player, or a mod developer, OPVM provides a clean and efficient way to handle OptiFine versions. The source code is open and available on GitHub, so if you have any security concerns, feel free to review it.

Getting started is easy—just follow the [installation guide](#installation) and run a few simple commands!

### Features

- 🚀 **Effortless Optifine Management**: Quickly install, update, and manage multiple OptiFine versions with simple commands.
- 🔍 **Enhanced Version Indexing Syntax**: A new and intuitive index syntax designed for a seamless user experience.
- ⚡ **Fast and Reliable Performance**: Optimized for blazing-fast downloads and hassle-free removals.
- 🗂️ **Seamless Compatibility**: Easily load existing OptiFine files and start using them right away.

### Installation

1. Go to [release page](https://github.com/zzzje/ropvm/releases/latest), and download the appropriate binary for your platform
2. Rename the downloaded binary as `opvm` (or `opvm.exe` on Windows)
3. Place the binary in a **dedicated folder**
4. Add the folder to your system's `PATH` environment variable (`path/to/your/dedicated/folder`)
5. Add environment variable `OPVM_HOME` as path to your **dedicated folder**
6. Run `opvm --version` to verify the installation
7. Run `opvm config` to config `opvm` parameters
    - `--minecraft-dir` is mandatory, set to `path/to/.minecraft`
    - `--java-path` can be left blank, which `opvm` will use `javew` in the environment
    - `--repo-dir` can be left blank, which `opvm` will use `$OPVM_HOME/repo` dir
8. Run `opvm config --test` to verify configuration
9. You're all set! 🎉

### Usages

#### Search available Minecraft/Optifine versions:

```sh
# List available Minecraft versions
opvm search
```
```sh
# List available Optifine versions for Minecraft 1.16.5
opvm search 1.16.5
```

#### Download one or more version(s):

```sh
opvm add 1.16.5
```
```sh
# Support index syntax
# index := [Range/Range/Range/...]
# range := `from~to` or `~to` or `from~` or `~` or `single indice`

# These 2 are equivalent
opvm add 1.16.5[]
opvm add 1.16.5[1]

# These 4 are equivalent
opvm add 1.16.5[1/2/3]
opvm add 1.16.5[1~3]
opvm add 1.16.5[1/2~3]
opvm add 1.16.5[~]
```
```sh
# Download multiple version
opvm add 1.16.5[~] 1.21.4 1.8.9
```

#### Remove one or more version(s):

```sh
opvm remove 1.16.5
```
```sh
opvm remove 1.16.5_HD_U_G8
```

#### List downloaded version(s)

```sh
opvm list
```
```sh
# with time stamp
opvm list --time
```

#### Config the tool

```sh
opvm config
```
```sh
# test config integrity
opvm config --test
```

#### Use the downloaded version

```sh
# opvm apply [PATTERN]
opvm apply 1.16.5
```
```sh
opvm apply 1.16.5_
```
```sh
opvm apply 1.16.5_HD_U_G8
```

#### Load files inside repo dir to database

Usually needed after you manually change contents inside `repo_dir`

```sh
opvm load
```

### Compatibility

OPVM is cross-platform and supports any environment where the Rust toolchain can be installed.

#### Pre-built Binaries

We provide **different versions** on our [release page](https://github.com/zzzje/ropvm/releases/latest) covering major operating systems and architectures. Please choose the one that matches your environment.

| OS      | Architecture             | Target Triple                |
| :------ | :----------------------- | :--------------------------- |
| macOS   | Apple Silicon            | aarch64-apple-darwin         |
| macOS   | Intel 64-bit             | x86_64-apple-darwin          |
| Linux   | ARM 64-bit               | aarch64-unknown-linux-gnu    |
| Linux   | Intel/AMD 64-bit         | x86_64-unknown-linux-gnu     |
| Windows | Intel/AMD 64-bit         | x86_64-pc-windows-msvc       |

#### Custom Build

If your specific platform is not listed in the releases, OPVM supports **all platforms and architectures** as long as `cargo` is able to build for them. You can simply compile it yourself using:
```bash
cargo build --release
```
