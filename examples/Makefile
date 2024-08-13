.PHONY: examples

examples:
	cat README.md | grep -E '^strandify' | sed 's/strandify/cargo run --release --/' | sh
	cat README.md | grep -E '^magick' | sh
