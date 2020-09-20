# buttons

[![Build Status](https://travis-ci.org/Soft/buttons.svg?branch=master)](https://travis-ci.org/Soft/buttons)
[![GitHub release](https://img.shields.io/github/release/Soft/buttons.svg)](https://github.com/Soft/buttons/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

<img align="right" src="https://raw.githubusercontent.com/Soft/buttons/master/extra/screenshot.png">

`buttons` is a small progressive web app that allows users to execute commands
from a predetermined set on the server. `buttons` exposes a simple mobile-first
web UI with a button for each command. Tapping the button will cause the
associated command to be executed.

⚠ **Important** ⚠: You should obviously think carefully about what commands are
safe to expose. While the set of exposed commands is fully static and there is
no possibility for users to pass additional input to them, you should still
consider placing limits on who can access the web server.

## Installation

Pre-built binaries can be downloaded from [GitHub
Releases](https://github.com/Soft/buttons/releases). These should work on any
64-bit Linux system.

### Usage

```
buttons 0.1.0
button service

USAGE:
    buttons <config>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <config>    Configuration file path
```

## Configuration

``` toml
address = "0.0.0.0:8080"
title = "Actions"

[[buttons]]
label = "Hello World"
command = "echo 'Hello World'"

[[buttons]]
label = "Toggle music"
command = "playerctl play-pause"
```
