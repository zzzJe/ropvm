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
5. You're all set! Run `opvm --version` to verify the installation

### Usages

To search available Minecraft/Optifine versions:
```sh
# List available Minecraft versions
opvm search
```
```sh
# List available Optifine versions for Minecraft 1.16.5
opvm search 1.16.5
```

Download one or more version(s):
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

List downloaded version(s)
```sh
opvm list
```
```sh
# with time stamp
opvm list --time
```

Config the tool
```sh
opvm config
```
```sh
# test config integrity
opvm config --test
```

Use the downloaded version
```sh
opvm apply [PATTERN]
```
```sh
opvm apply 1.16.5
```
```sh
opvm apply 1.16.5_
```
```sh
opvm apply 1.16.5_HD_U_G8
```

### Compatibility

OPVM supports all platforms and architectures as long as `cargo` is able to build for that platform.

The binaries available on the [release page](https://github.com/zzzje/ropvm/releases/latest) are currently built for the `x86_64` architecture only. If you need to use OPVM on other architectures, you can download the source code and run `cargo build --release` to compile the tool yourself.
