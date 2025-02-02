# Chip-8 Emulator 


## Used the following guide + repositories as huge helps (I am very new to Rust)
- [Tobias V. Langhoff's guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#prerequisites)
- [aquova Rust chip-8 introduction](https://github.com/aquova/chip8-book) - **HELPED SO MUCH FOR THE GUI THANK YOU**

# *USED [RUST](https://www.rust-lang.org/tools/install) AND SDL2*
# For Windows
- You will need SDL2 as a pre-requisite
```
git clone https://github.com/microsoft/vcpkg.git
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg install sdl2
.\vcpkg integrate install
```

The above clones vcpkg which allows you to install SDL2

# How to use

## 1. Clone the repository 
```bash
$ git clone https://github.com/OkRespire/chip-8-emulator.git
```

## 2. build the program
```bash
$ cd chip-8-emulator
$ cargo build --release
```

## 3. Run the program
- On Linux and MacOS
```bash
$  ./target/release/chip-8-respire path/to/rom
```

- On Windows

```
./target/release/chip-8-respire.exe path/to/rom
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
