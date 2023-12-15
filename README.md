# guishi

"guishi" is a tool use a series of encryption algorithms to encrypt shellcode

## How it works

The key generated is different every time
AES -> RC4 -> UUID encode output

## How to use

```bash
guishi -h
```

```bash
guishi -f $shellcode_path
```

![run](guishi.gif)

## RUN

Then copy the output code to main.c in the clib folder

```bash
make

wine ./enc.x64.exe
```

![run](enc.gif)

## Install

```bash
cargo build --release
```

## Summary

This project is just a simple shellcode encryption, it should be said to be a demon.
