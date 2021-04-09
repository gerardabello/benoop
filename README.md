# Benoop

Benoop is a benchmarking tool for HTTP servers. Similar to `ab` (Apache HTTP server benchmarking tool) but allowing more complex benchmarks, like multiple URLs at the same time. It is focused on having a feedback loop while developing, so it has a baseline comparison feature to see if your changes had an impact on performance.

## Install

Install it with cargo: `cargo install benoop`
Or get pre-build binaries from the latest [release](https://github.com/gerardabello/benoop/releases).

## Usage

Initialize config with `benoop init`. The initial configuration file will need to be edited to use your URLs, but it will help you see how it should look.

To run a benchmark you simply need to run `benoop`.

To use the baseline comparison feature, create a baseline by running `benoop --save-baseline`. All following benchmarks will be compared agains the saved baseline. To clear the baseline simply run `benoop clear`.
