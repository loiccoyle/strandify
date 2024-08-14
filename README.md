<p align="center"><img src="https://i.imgur.com/4jvon2p.png" width="1000"></p>
<p align="center"><b>A CLI utility to create string art.</b></p>

<p align="center">
  <a href="https://github.com/loiccoyle/strandify/actions"><img src="https://github.com/loiccoyle/strandify/actions/workflows/build.yml/badge.svg"></a>
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <img src="https://img.shields.io/badge/platform-linux%20%7C%20macOS%20%7C%20windows-informational">
</p>

## 📦 Installation

```sh
cargo install strandify
```

## 📋 Usage

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
  -C, --yarn-color <YARN_COLOR>
          Yarn color [default: "0 0 0"]
      --project-to-yarn-color
          Project image to yarn color
  -S, --peg-shape <SHAPE>
          Peg distribution shape [default: circle] [possible values: circle, square, border]
  -n, --peg-number <PEG_NUMBER>
          Number of pegs [default: 288]
  -m, --peg-margin <PEG_MARGIN>
          Margin between pegs and image edge [0, 1] [default: 0.05]
  -j, --peg-jitter <PEG_JITTER>
          Add jitter to the peg position
  -s, --peg-skip-within <PEG_SKIP_WITHIN>
          Don't connect pegs within pixel distance
  -o, --yarn-opacity <YARN_OPACITY>
          Yarn opacity to use to render the image [0, 1] [default: 0.2]
  -w, --yarn-width <YARN_WIDTH>
          Yarn width to use to render the image [default: 1]
  -l, --line-opacity <LINE_OPACITY>
          Line opacity, controls how much to lighten the pixels at each line pass, low values encourage more line overlap [0, 1] [default: 0.1]
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

## 👓 Examples

See the [examples](https://github.com/loiccoyle/strandify/tree/main/examples) folder.
