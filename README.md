# RNA [![Build Status](https://travis-ci.com/oOBoomberOo/rna.svg?branch=master)](https://travis-ci.com/oOBoomberOo/rna) [![Discord](https://img.shields.io/discord/428791010244558850?color=blue&label=Discord&logo=discord)](https://discord.gg/56ySADc)

RNA is a library for interpreting "Loot Table Script" designed by [Minecraft Datapacks](https://discord.gg/56ySADc) community.

## Usage

```rust
let ruby = rna::interpret_file("ruby.ult", "resource").unwrap();
let draconic_ore = rna::interpret_file("draconic_ore.ult", "resource").unwrap();

let merged_loot = rna::merge(&[ruby, draconic_ore], "resource").unwrap();
```

```rust
if rna::is_loot_table_script("path/to/loot_table/file.json.merge") {
    // Do something
}
```

## Installation

By default, this library will not recognized vanilla loot table. (Totally not because I can't figure out how to do it)
You need to specify the `base_path` to tell it where to look for the loot table files.
