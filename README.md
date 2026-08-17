# lightpanel - a light CLI server panel, built fully on Rust. Unix-only.

# COMMANDS

# -- Setup --
```
sudo lpnl setup
```
Sets you up for a work with lightpanel (installs nginx, ensures all directories needed are there).


# -- Config manipulation commands --
```
sudo lpnl init
```
Initializes and generates an nginx config. You can use flag '--default' to use 'localhost' domain, '8080' port and '/var/www/localhost' as root. You can also use flags '--default <String>, 'port <u16>' and --root <PathBuf>' for adjusting your preferences right away.
Examples:
```
sudo lpnl init example.com --root /var/www/example.com --port 80
sudo lpnl init --default
```

```
sudo lpnl remove <domain>
```
Removes an nginx config inside '/etc/nginx'. You can use flag '--force' to delete nginx config, its backup in '/etc/lpnl/backups' and files from the '/var/www' if you have used it as root folder. Use it carefully.
Examples:
```
sudo remove example.com
sudo remove localhost --force
```

```
sudo lpnl add-loc <domain>
```
Lets you choose between a root or a proxy mode interactively. You can use flag '--root <PathBuf>' or '--proxy <String>' to set up from one command. You can also try '--location <String>' to select a location.
Example:
```
sudo lpnl add-loc localhost --location /api --proxy http://example.com
```

```
sudo lpnl remove-loc <domain>
```
Does the oposite of add-loc command - deletes a location. There is only '--location <String>' flag available here.
Example:
```
sudo lpnl remove-loc localhost --location /api
```
# -- Config state-control commands --
All of the state-control manipulation happen between two directories. A one directory, which nginx includes in config. And the other one isn't included. So these directories are '/etc/nginx/sites-enabled/' and 'etc/nginx/sites-disabled/'.

```
sudo lpnl enable <domain>
```
Moves a domain specified to a sites-enabled folder, enabling the config for nginx.

```
sudo lpnl disable <domain>
```
Does complete the oposite of 'enable' command, disabling the config for nginx.

```
sudo lpnl enable-all
sudo lpnl disable-all
```
These two commands are similar to their regular versions, but they move all of their contents into another directory.

# -- Config backuping --
All of the backup saves are placed in '/etc/lpnl/backups' and every config has it's own folder. It's because in future I want to add a git-like backup system.

```
sudo lpnl set-backup <domain>
```
Creates a backup from a copy of a working config file in '/etc/lpnl/backups/{domain}/domain.txt'.

```
sudo lpnl get-backup <domain>
```
Takes a backup and pastes it into a working version in '/etc/nginx/sites-enabled/{domain}.conf' or '/etc/nginx/sites-disabled/{domain}.conf' depending on if the config is enabled or not.

# -- Monitoring --

```
lpnl list
lpnl list-enabled
lpnl list-disabled
lpnl list-backups
```
All of them print out a list of configurations, depending on a category you're searching in. Regular 'list' shows all enabled, disabled, and backed up configs.

```
lpnl stats
```
Disaplays you such information as:
- Host name
- System name
- OS version
- Kernel version
- Uptime in HH:MM:SS
- CPU usage and temperature
- RAM usage
- Information about the disks (name, kind, available, total and used storage, available and free inodes)

```
lpnl short-stats
```
Displays you a short version of 'stats' shpwing you:
- Host name
- System name
- OS version
- Kernel version
- Uptime in HH:MM:SS
- CPU usage and temperature
- RAM usage

```
lpnl disk-stats
```
Shows you just an information about the disks (name, kind, available, total and used storage, available and free inodes).

# DEPENDENCIES

- clap (CLI)
- sysinfo (stats displaying, a main part of the monitoring)
- nix (inodes of the disks' displaying)
- url (domain parsing)