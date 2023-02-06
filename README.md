<!--
SPDX-FileCopyrightText: 2023 Davidson <twister@davidson.fr>
SPDX-License-Identifier: CC-BY-NC-SA-4.0
-->

[![REUSE status](https://api.reuse.software/badge/github.com/fsfe/reuse-tool)](https://api.reuse.software/info/github.com/fsfe/reuse-tool)

# Ecocoding Optimization Analysis

## Abstract

Following [a french article](https://www.davidson.fr/blog/righttech-thinking) about optimization of produced code by [ChatGPT](https://openai.com/blog/chatgpt/), this project contains a runnable programme that we can execute in an **iterative** way or an **optimized** one.

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

## Measurements

### About

* **Iteration**: are computed inside the code each time a triplet is evaluated to solve the problem 
* **Energy**: are analyzed by vjoule tool and expressed by a number of joules
* **Time**: with the well know unix tool time (in seconds)

**Here we compare the seraching for this set of sum: 12, 100, 1000, 10000, 100000, 1000000.**

*All measurements are done on a laptop with Intel(R) Core(TM) i5-10210U CPU @ 1.60GHz and 16Go of ram*

### Results

|sum    |I iter     |I joule   |I time|O iter|O joule |O time|
|------:|----------:|---------:|-----:|-----:|-------:|-----:|
|     12|         10|  0.007086|  0.00|     9|0.007162|  0.00|
|    100|        817|  0.008674|  0.00|    18|0.008143|  0.00|
|   1000|      83167|  0.007626|  0.00|    60|0.006333|  0.00|
|  10000|    8331667|  0.083538|  0.04|   150|0.014923|  0.00|
| 100000|  833316667| 12.476200|  1.27|   315|0.010210|  0.00|
|1000000|83333166667| 1292.1700|122.56|   588|0.025337|  0.00|


![Iteration %diff](https://github.com/davidson-consulting/Ecocoding-Optimization-Analysis/blob/main/measurements/iteration.png)

![Energy %diff](https://github.com/davidson-consulting/Ecocoding-Optimization-Analysis/blob/main/measurements/energy.png)

![Time %diff](https://github.com/davidson-consulting/Ecocoding-Optimization-Analysis/blob/main/measurements/time.png)
