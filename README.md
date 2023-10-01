# printscp

A just for fun project. I use SSH and SCP a lot and wanted a way to quickly print out SCP notation file paths for different network interfaces.
Cross-platform Linux/MacOS/Windows.
## Usage Examples 

### Help / Usage Prompt
```shell
printscp -h
```
Output
```
Usage: printscp [options] [file_path]
Options:
  -x  Filter out loopback addresses.
  -4  Print all ipv4 addresses.
  -6  Print all ipv6 addresses.
  -h  --help Print this help.
```

### Basic
```shell
cd ~
printscp
```
Output Example:
```
NixHost:/home/doggo
192.168.1.10:/home/doggo
```

### Providing A Relative Path
```shell
cd ~
printscp documents/stuff.tgz
```
Output Example:
```
NixHost:/home/doggo/documents/stuff.tgz
192.168.1.10:/home/doggo/documents/stuff.tgz
```
