
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

<img width="400" alt="trefoil" src="https://github.com/IgorPietrzak/P-colourings-of-Knots/assets/96392306/80ca621d-9721-4f61-a4e8-315c9dce1986">



To find the number of 3-colourings we run:


```bash
./target/release/knots 2 s1_s1_s1 3

```

output:

```bash
number of 3-colourings: 9
```
