<!--
SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
SPDX-License-Identifier: CC-BY-NC-SA-4.0
-->

[![REUSE status](https://api.reuse.software/badge/github.com/fsfe/reuse-tool)](https://api.reuse.software/info/github.com/fsfe/reuse-tool)

# Ecocoding Optimization Analysis

## Abstract

Following [a french article](https://www.davidson.fr/blog/righttech-thinking) about optimization of produced code by [ChatGPT](https://openai.com/blog/chatgpt/), this project contains a runnable programme that we can execute in an iterative way or an optimized one.

## Compiling

Use cargo command `cargo build --release`

## Executing

Use embeded help `./target/release/find_pythagorean_triple_by_sum -h`

```
Find pythagorean triplets for which members a + b + c corresponding to a given sum (with a < b < c)

Usage: find_pythagorean_triple_by_sum <sum> [mode]

Arguments:
  <sum>   the target sum (must be even)
  [mode]  search mode [default: optimized] [possible values: optimized, iterative]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

_!!! NOT YET WRITTEN !!!_
