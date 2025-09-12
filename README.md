# flora

A seed for your Wine prefixes. Quickly launch your favorite Windows apps and games on Linux using Wine and Proton.

## Features

* Manage Wine and Proton setups, configured in a `seed`. Each `seed` can have separate prefixes and runtimes.
* Define application entries to be launched from a `seed`.
* Generate application entries from Start Menu shortcuts.
* Automatically generate application menus for easy access to Windows applications
* Use custom Wine and Proton runtimes for a `seed`.
* Transparent configuration, everything is laid out in `toml` files

## Installation

flora depends on `wine` and `winetricks`. Proton support additionally depends on `umu-launcher`.
### From releases

Grab the latest binary from the [Releases](https://github.com/Damillora/flora/releases) page.

### Run from source

flora is built and tested against latest Rust.

```sh
git clone https://github.com/Damillora/flora.git
cd flora
cargo install --path crates/flora_cli
```

## Usage
```zsh
# Create a Wine seed
flora create wine windows_app
# Create a Proton seed
flora create proton proton_game

# Run the installer inside a seed
flora run windows_app ~/Documents/windows_app_installer.exe
# Launch winetricks for seed prefix configuration
flora tricks windows_app
# Add an app using Start Menu entries
flora start-menu create-app "Windows App"
# Generate menus 
flora generate-menu
# Run an app inside a seed
flora run -a windows_app "Windows App"
```
[![asciicast](https://asciinema.org/a/kX1eNGz3W2rYHppeyESZigOig.svg)](https://asciinema.org/a/kX1eNGz3W2rYHppeyESZigOig)

flora can also utilize additional Wine runtimes in `~/.local/share/flora/wine`, and additional Proton runtimes in `~/.local/share/flora/proton`.
Runtimes in those folder can be managed with tools like [ProtonUp-Qt](https://github.com/DavidoTek/ProtonUp-Qt).

### `flora` commands
* `seed`: Manage seeds
    * `seed list`: List all seeds
    * `seed create`: Create a seed
    * `seed set`: Set a seed's properties
    * `seed delete`: Remove a seed
    * `seed info`: Show a seed's information
* `app`: Manage apps in a seed
    * `app list`: List all apps in a seed
    * `app add`: Add an app into a seed
    * `app update`: Update an app in a seed
    * `app rename`: Rename an app in a seed
    * `app delete`: Remove an app from a seed
    * `app generate-menu`: Generate menu entries for launching apps from the application menu
* `start-menu`: Query Start Menu entries in a seed and create app entries based on them
    * `start-menu list`: List all Start Menu entries in a seed
    * `start-menu create-app`: Generate an app based on a Start Menu entry
* `config`: Launch the seed's prefix configuration, usually `winecfg`
* `tricks`: Launch winetricks for the seed's prefix 
* `run`: Run an application in a seed


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
* `[settings]`
  * `launcher_command`: Launch command for Wine or `umu-launcher`, e.g. to use Gamescope to launch apps inside a seed.
* `[[apps]`: The first `[[app]]` is the default application for the seed, and any subsequent `[[apps]]` can be launched using `flora run -a <seed> "<application_name>"`
  * `application_name`: Name of default application shown on menu
  * `application_location`: The executable to be launched when using the `run` command without arguments.
* `[wine]`
  * `wine_prefix`: Prefix used by the seed.
  * `wine_runtime`: Wine runtime used by the seed.
* `[proton]`
  * `proton_prefix`: Prefix used by the seed.
  * `proton_runtime`: Proton runtime used by the seed.
  * `game_id`: Game ID to be passed to `umu-launcher`
  * `store`: Store name to be passed to `umu-launcher`

## Contributing

flora is still in heavy development, but contributions are welcome! Feel free to file an issue or even submit a PR if you want.

## License

flora is licensed under [GNU GPL v3 or later](LICENSE).
