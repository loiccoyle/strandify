<p align="center"><img src="https://i.imgur.com/4jvon2p.png" width="1000"></p>
<p align="center"><b>A CLI utility to create string art.</b></p>

<p align="center">
  <a href="https://crates.io/crates/strandify"><img src="https://img.shields.io/crates/v/strandify"></a>
  <a href="https://crates.io/crates/strandify-cli"><img src="https://img.shields.io/crates/v/strandify-cli"></a>
  <a href="https://npmjs.com/package/strandify-wasm"><img src="https://img.shields.io/npm/v/strandify-wasm"></a>
  <a href="https://docs.rs/strandify/latest/strandify/"><img src="https://img.shields.io/docsrs/strandify"></a>
  <a href="https://github.com/loiccoyle/strandify/actions"><img src="https://github.com/loiccoyle/strandify/actions/workflows/ci.yml/badge.svg"></a>
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-informational">
</p>

<p align="center">
  <b>
  Create your string art <a href="https://loiccoyle.com/strandify">here</a>
  </b>
</p>

This repo contains two crates:

- `strandify` crate contains the string art library.
- `strandify-cli` crate provides the command line interface.

And one `npm` package:

- `strandify-wasm` provides the wasm bindings and allows `strandify` to be used in the browser.

## 📦 Installation

## Command line

To use the `strandify` binary to generate string art, install the `strandify-cli` crate:

```sh
cargo install strandify-cli
```

### 📋 Usage

Once installed, you can use the `strandify` binary.

Something like:

```sh
strandify input_img.png output_img.png
```

If in doubt see the help:

<!-- help start -->

```console
$ strandify -h
CLI utility to generate string art

Usage: strandify [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   Input image or blueprint json file
  [OUTPUT]  Output file, either image format, svg or json

Options:
  -i, --iterations <ITERATIONS>
          Number of iterations [default: 4000]
  -t
          Transparent background
  -c, --yarn-color <YARN_COLOR>
          Yarn color [default: "0 0 0"]
      --project-to-yarn-color
          Project image to yarn color
  -S, --peg-shape <SHAPE>
          Peg distribution shape [default: circle] [possible values: circle, square, border]
  -n, --peg-number <PEG_NUMBER>
          Number of pegs. Depending on the shape, can be slightly off [default: 288]
  -m, --peg-margin <PEG_MARGIN>
          Margin between pegs and image edge [0, 1] [default: 0.05]
  -j, --peg-jitter <PEG_JITTER>
          Add jitter to the peg position
  -s, --peg-skip-within <PEG_SKIP_WITHIN>
          Don't connect pegs within pixel distance
  -O, --yarn-opacity <YARN_OPACITY>
          Yarn opacity to use to render the image [0, 1] [default: 0.2]
  -W, --yarn-width <YARN_WIDTH>
          Yarn width to use to render the image [default: 1]
  -o, --line-opacity <LINE_OPACITY>
          Line opacity to use when computing the path, controls how much to lighten the pixels at each line pass, low values encourage more line overlap [0, 1] [default: 0.1]
  -w, --line-width <LINE_WIDTH>
          Line width to use when computing the path [default: 2]
  -b, --beam-width <BEAM_WIDTH>
          Beam search width, a value of 1 results in a purely greedy algorithm [default: 1]
  -e, --early-stop-threshold <EARLY_STOP_THRESHOLD>
          If provided, early stop pathing when consecutive path losses are greater than threshold
  -E, --early-stop-count <EARLY_STOP_COUNT>
          Number of consecutive iterations with path losses above threshold to allow [default: 100]
      --output-scale <OUTPUT_SCALE>
          Output scale [default: 1]
      --save-pegs <PEG_SAVE_FILE>
          Write pegs to file
      --load-pegs <PEG_LOAD_FILE>
          Read pegs from file
  -v, --verbose...
          Increase logging verbosity
  -q, --quiet...
          Decrease logging verbosity
  -h, --help
          Print help
```

<!-- help end -->

## Library

To use the library as a dependency in your project, add the `strandify` crate:

```sh
cargo add strandify
```

See the [docs](https://docs.rs/strandify) for usage.

## Wasm

To use the `wasm` bindings in your project, add the `strandify-wasm` `npm` package:

```sh
npm add strandify-wasm
```

I would recommend taking a look at the [demo page's source code](https://github.com/loiccoyle/strandify/tree/gh-pages) to see how to use it.

## 👓 Examples

See the [examples](https://github.com/loiccoyle/strandify/tree/main/examples) folder.
