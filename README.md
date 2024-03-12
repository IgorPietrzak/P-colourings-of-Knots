
# Usage:

## Clone repository:
```bash
git clone https://github.com/IgorPietrzak/P-colouings-of-braid-knots.git
```

## Build executable:
```bash
cd Knots && cargo build --release

```

## Run program:

```bash
./target/release/knots <n> <braid_representation> <p>

```
 ### Arguments:

 n: Number of strands in the n-braid.

 
 braid_representation: Sequence of crossings in the braid e.g. s1_s2_-s1.

 
 p: Number of colours.

 ## Examples:

 2-braid with s1 applied 3 times:

<img width="400" alt="image" src="https://github.com/IgorPietrzak/P-colouings-of-braid-knots/assets/96392306/c46c30fd-6e58-440c-a7a7-e3c036293ed4">


To find the number of 3-colourings we run:


```bash
./target/release/knots 2 s1_s1_s1 3

```

output:

```bash
number of 3-colourings: 9
```
