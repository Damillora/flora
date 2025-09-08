# flora

A seed for your Wine prefixes. Quickly launch your favorite Windows apps and games on Linux using Wine and Proton.

## Features

* Manage Wine and Proton prefixes 
* Launch Proton using `umu-launcher`
* Create `.desktop` entries for easy access
* Transparent configuration, everything is laid out in `toml` files

## Installation

flora depends on `wine` and `winetricks`. Proton support additionally depends on `umu-launcher`.

flora can also utilize additional Wine runtimes in `~/.local/share/flora/wine`, and additional Proton runtimes in `~/.local/share/flora/proton`.
Runtimes in those folder can be managed with tools like [ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt).

### From releases

Grab the latest binary from the [Releases](https://github.com/Damillora/flora) page.

### Run from source

flora is built and tested against latest Rust.

```sh
git clone https://git.nanao.moe/Damillora/flora.git
cd flora
cargo install --path crates/flora_cli
```

## Configuration

flora is configured using the file `flora.toml`, located in the `$HOME/.local/share/flora` folder. 

This file will be automatically generated with defaults when `flora` is run for the first time.

* `[wine]`
  * `wine_prefix_location`: Location where Wine prefixes are installed. Default is `$HOME/.local/share/flora/prefixes`.
  * `default_wine_prefix`: Default Wine prefix used by seeds. Default is `$HOME/.local/share/flora/prefixes/default`.
  * `default_wine_runtime`: Default Proton runtime used by seeds. Default is system wine (`/usr/bin/wine`).
* `[proton]`
  * `proton_prefix_location`: Location where Wine prefixes are installed. Default is `$HOME/.local/share/flora/prefixes`.
  * `default_proton_prefix`: Default Wine prefix used by seeds. Default is `$HOME/.local/share/flora/prefixes/proton`.
  * `default_proton_runtime`: Default Proton runtime used by seeds. Default is empty.

Each application is configured in `.toml` files, located in `$HOME/.local/share/flora/seeds` folder.
* `pretty_name`: Name of application shown on menu
* `executable_location`: The executable to be launched when using the `run` command and from the `.desktop` entry.
* `[wine]`
  * `wine_prefix`: Prefix used by the seed.
  * `wine_runtime`: Wine runtime used by the seed.
* `[proton]`
  * `proton_prefix`: Prefix used by the seed.
  * `proton_runtime`: Proton runtime used by the seed.
  * `game_id`: Game ID to be passed to `umu-launcher`
  * `store`: Store name to be passed to `umu-launcher`

## CLI 

See [flora_cli README](crates/flora_cli/README.md) for documentation.

## Contributing

flora is still in heavy development, but contributions are welcome! Feel free to file an issue or even submit a PR if you want.

## License

flora is licensed under [GNU GPL v3 or later](LICENSE).
