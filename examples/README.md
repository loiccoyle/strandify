# Examples

This folder contains some examples of the images produced by this program and the commands used to generate them.

## Lincoln

```sh
strandify lincoln.jpg lincoln_out.jpg -o 0.3 -l 0.2 -j 3
```

![Lincoln input](./lincoln.jpg)
![Lincoln output](./lincoln_out.jpg)

## Mona Lisa

```sh
strandify mona_lisa.jpg mona_lisa_out.jpg -o 0.15 -l 0.08 -S square
```

![Mona Lisa input](./mona_lisa.jpg)
![Mona Lisa output](./mona_lisa_out.jpg)

## Einstein

```sh
strandify einstein.jpg einstein_out.svg -w 2 -o 0.1 -l 0.1 -j 3 -i 2000 -n 200
```

![Einstein input](./einstein.jpg)
![Einstein output](./einstein_out.svg)

## Golden Gate Bridge

```sh
strandify golden_gate.jpeg golden_gate_out_red.png -S border -j 2 -C "255 0 0" --project-to-yarn-color -m 0.01 -n 500 -t -w 0.4 -i 1000 -o 0.3 -l 0.1
strandify golden_gate.jpeg golden_gate_out.jpg -S border -j 2 -m 0.01 -n 500 -w 0.3 -i 3000 -l 0.1
magick composite golden_gate_out_red.png golden_gate_out.jpg golden_gate_composite_out.png
```

![Golden Gate input](./golden_gate.jpeg)
![Golden Gate output](./golden_gate_out.jpg)
![Golden Gate red output](./golden_gate_out_red.png)
![Golden Gate composite output](./golden_gate_composite_out.png)
