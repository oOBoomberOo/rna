# RNA

RNA is a library for interpreting "Loot Table Script" designed by [Minecraft Datapacks](https://discord.gg/56ySADc) community.

## Usage

```rust
let ruby = rna::interpret_file("ruby.ult").unwrap();
let draconic_ore = rna::interpret_file("draconic_ore.ult").unwrap();

let merged_loot = rna::merge(&[ruby, draconic_ore]).unwrap();
```

```rust
if rna::is_loot_table_script("path/to/loot_table/file.json.merge") {
    // Do something
}
```
