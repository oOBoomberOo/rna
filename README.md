# RNA

RNA is a library for interpreting "Loot Table Script" designed by [Minecraft Datapacks](https://discord.gg/56ySADc) community.

## Usage

```rust
let ruby = rna::interpret_file("ruby.megu").unwrap();
let draconic_ore = rna::interpret_file("draconic_ore.megu").unwrap();

let merged_loot = rna::merge(&[ruby, draconic_ore]).unwrap();
```
