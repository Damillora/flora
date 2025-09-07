# flora

A seed for your Wine prefixes. Quickly launch your favorite Windows apps and games on Linux.

Proton support using `umu-launcher` is currently planned.

## Features

* Manage Wine prefixes 
* Create `.desktop` entries for easy access
* Transparent configuration, everything is laid out in `toml` files

## Installation

flora depends on `wine` and `winetricks`.

flora can also utilize additional Wine installations in `~/.local/share/flora/wine`.
Wine installations in this folder can be managed with tools like [ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt).

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

This file will be automatically generated with defaults when `flora-cli` is run for the first time.

* `[wine]`
  * `wine_prefix_location`: Location where Wine prefixes are installed. Default is `$HOME/.local/share/flora/prefixes`.
  * `default_wine_prefix`: Default Wine prefix used for running applications. Default is `$HOME/.local/share/flora/prefixes/default`.
  * `default_wine_runner`: Default Wine installation used for running applications. Default is system wine (`/usr/bin/wine`).

Each application is configured in `.toml` files, located in `$HOME/.local/share/flora/apps` folder.
* `type`: Choose between `Wine` for running apps in Wine or `Proton` for running apps in Proton using umu-launcher.
* `executable_location`: The executable to be launched when using the `run` command and from the `.desktop` entry.
* `wine_prefix` (Wine): Prefix where the app is installed.
* `wine_runner` (Wine): Wine runtime used to run the app.

## CLI 

See [flora_cli README](crates/flora_cli/README.md) for documentation.

## Contributing

flora is still in heavy development, but contributions are welcome! Feel free to file an issue or even submit a PR if you want.

## License

flora is licensed under [GNU GPL v3 or later](LICENSE).
