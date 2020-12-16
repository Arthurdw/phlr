# PHLR
###### Pre-Hash Loader/Renderer

## Installation

Installing phlr is very easy and just requires this one command to be executed.
The whole installation process will start automatically.

```bash
$ sudo curl --proto '=https' -L --tlsv1.2 -sSf https://phlr.arthurdw.com | sh
```

### Check if installation was successfull

To check if your installation was successfull just execute any phlr command.
For example:

```bash
$ phlr --version
Pre-Hash Loader/Renderer (phlr) 0.1.0
```

## Uninstall

You can easily uninstall phlr by executing the following command.

```bash
$ sudo apt remove phlr
```

## Updating

You can easily update by just running the update command. The script will do everything by itself.

```bash
$ phlr update
```


## Basic usage

Using phlr is extremly easy, for example the following command would create an output file in a directory called "foo" which contains the file hashed (sha256) file content.

```bash
$ cat foo.txt
foo
bar

$ phlr generate foo.txt 
 info: Logging max level: CRITICAL
 info: Creating destination directory: "foo"

$ cat foo/foo-out.txt 
foo;2c26b46b68ffc68ff99b453c1d30413413422d706483bfa0f98a5e886266e7ae
bar;fcde2b2edba56bf408601fb721fe9b5c338d10ee429ea04fae5511b68fbf8fb9
```
