# Rust Voxlap Test

Showcase and sample application for [rust-voxlap](https://github.com/bbodi/rust-voxlap), which is Voxlap binding for Rust.

# Overview

Voxlap is a voxel engine by Ken Silverman: http://advsys.net/ken/voxlap.htm

This demo demonstrates how to use Voxlap engine in Rust through [rust-voxlap](https://github.com/bbodi/rust-voxlap) binding.

I tried to pack a lot of showcase feature in this demo, so the scene is a little bit chaotic.

# Requirements
This repository contains all the necessary library files for Windows, and Cargo (Rust package manager) handles all the dependencies, so you don't need to care a lot about the following requirements.

## SDL2

[rust-voxlap](https://github.com/bbodi/rust-voxlap) is independent from any system dependencies, but the demo uses [Rust-SDL2](https://github.com/AngryLawyer/rust-sdl2) for
- creating window
- drawing the scene calculated by Voxlap to the screen (including 2D texts and images)
- handling input
- TODO: handling sound

# Installation

Use the Rust Nightly installer and cargo to run the demo:

```
git clone https://github.com/bbodi/rust-voxlap-test.git
cargo run
```

## Voxlap libraries
This repository contains all the necessary library files for Windows, but if you need to compile your own files, here is a little help:  
 The voxlap.lib was compiled based on [this](https://github.com/davidsiaw/voxlaptest) repository with Visual Studio 2010.
I find this repo the easiest to compile, but you can use other sources too.  
**But be aware**, this repo contains two commits, but you only need the first one!  
*(The second commit
tries to raise the map size from 1024x1024 to 2048x2048, but the modifications are incomplete,
and you cannot load .vxl files correctly with this second commit.)*  
After you checked it out and reversed to the first commit, you need to create a .lib from the voxlap project in Visual Studio.

# Images
![Empty room with a green voxel](http://i.imgur.com/IiuaEem.png)

![](http://i.imgur.com/HlkqC5r.png)

![](http://i.imgur.com/ftnAucZ.png)

![](http://i.imgur.com/z3xrYzw.png)

![](http://i.imgur.com/LCHAKoo.png)

![](http://i.imgur.com/oBHcIpl.png)

![](http://i.imgur.com/Fh5JCZg.png)

![](http://i.imgur.com/8qydDDf.png)

![](http://i.imgur.com/tKaj7xP.png)

![](http://i.imgur.com/OxmsQ4C.png)

![](http://i.imgur.com/CtLZPEJ.png)
