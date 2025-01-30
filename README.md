# Chip-8 Emulator 


## Used the following guide + repositories as huge helps (I am very new to Rust)
- [Tobias V. Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#prerequisites)
- [aquova Rust chip-8 introduction](https://github.com/aquova/chip8-book) - **HELPED SO MUCH FOR THE GUI THANK YOU**

# *USED RUST AND SDL2*


# How to use

## 1. Clone the repository 
```bash
$ git clone https://github.com/OkRespire/chip-8-emulator.git
```

## 2. build the program
```bash
$ cd chip-8-emulator
$ cargo build
```

## 3. Run the program
```bash
$  ./target/debug/chip-8-emulator path/to/rom
```

## 4. Enjoy!
This should be it for the installation and usage. Some roms may be slower, and some may not work at all since they are not supported. 
Hopefully I will be able to implement super chip-8 and Xo-chip support, as well as a more pretty GUI.


# Controls 
- This uses the standard chip-8 keyboard layout for the modern keyboard as shown below

```
1 |	2 |	3 |	4
Q |	W |	E |	R
A |	S |	D |	F
Z |	X |	C |	V

```
