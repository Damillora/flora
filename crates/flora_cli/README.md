# flora_cli
`flora` is the command line interface to Flora.

## Usage
* `seed`: Manage seeds
    * `seed list`: Lists all seeds
    * `seed create`: Creates a seed
    * `seed set`: Sets a seed's properties
    * `seed delete`: Removes a seed
    * `seed info`: Show a seed's information
* `app`: Manage apps in a seed
    * `app list`: Lists all apps in a seed
    * `app add`: Adds an app into a seed
    * `app update`: Updates an app in a seed
    * `app rename`: Renames an app in a seed
    * `app delete`: Removes an app from a seed
    * `app start-menu`: Generates an app entry from a Start Menu shortcut
* `config`: Launches the seed's prefix configuration, usually `winecfg`.
* `tricks`: Launches winetricks for the seed's prefix 
* `run`: Runs an application in a seed
* `generate-menu`: Generates menu entries for each app entries in a seed for launching from the application menu
