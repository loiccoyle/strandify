# Examples

This folder contains some examples of the images produced by this program and the commands used to generate them.

## Strandify

```sh
strandify strandify.jpg strandify_out.jpg -S border -O 0.2 -j 2 -o 0.3 -i 2000
```

![strandify input](./strandify.jpg)
![strandify output](./strandify_out.jpg)

## Lincoln

```sh
strandify lincoln.jpg lincoln_out.jpg -O 0.3 -o 0.1 -j 3
```

![Lincoln input](./lincoln.jpg)
![Lincoln output](./lincoln_out.jpg)

## Mona Lisa

```sh
strandify mona_lisa.jpg mona_lisa_out.jpg -S square -o 0.04 -O 0.1 -b 5

```

![Mona Lisa input](./mona_lisa.jpg)
![Mona Lisa output](./mona_lisa_out.jpg)

## Einstein

```sh
strandify einstein.jpg einstein_out.svg -W 2 -O 0.1 -o 0.07 -j 3 -i 2000 -n 200
```

![Einstein input](./einstein.jpg)
![Einstein output](./einstein_out.svg)

## Golden Gate Bridge

```sh
strandify golden_gate.jpeg golden_gate_out_red.png -S border -j 2 -c "255 0 0" --project-to-yarn-color -m 0.01 -t -O 0.1 -i 500
strandify golden_gate.jpeg golden_gate_out.jpg -S border -j 2 -m 0.01 -O 0.08 -o 0.05
magick composite golden_gate_out_red.png golden_gate_out.jpg golden_gate_composite_out.png
```

![Golden Gate input](./golden_gate.jpeg)
![Golden Gate output](./golden_gate_out.jpg)
![Golden Gate red output](./golden_gate_out_red.png)
![Golden Gate composite output](./golden_gate_composite_out.png)
