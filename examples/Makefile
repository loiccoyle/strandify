.PHONY: examples

examples:
	cat README.md | grep -E '^stringart' | sed 's/stringart/cargo run --release --/' | sh
	cat README.md | grep -E '^magick' | sh
