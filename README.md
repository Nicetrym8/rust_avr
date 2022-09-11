# Rust AVR

Simple

## Build instructions

Install Rust nightly.

Then run:

```
cargo build --target avr-atmega328p.json -Z build-std=core --release
```

The final ELF executable file will then be available at `target/avr-atmega328p/release/rust_avr.elf`.
