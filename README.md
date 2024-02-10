# launch
shrcにエイリアス書くのがめんどくさい人向けのスクリプトマネージャ 

## Prerequisites
- [cargo](https://github.com/rust-lang/cargo)

## Getting Started
Build the binaly, add to PATH directory.
```
$ git clone https://github.com/TundraClimate/launch.git
$ cd launch

$ cargo build -r
$ ln -s ./target/release/launch /usr/local/bin
```

## How to use
```
$ launch --help
Usage: launch [OPTIONS] [SCRIPT]

Arguments:
  [SCRIPT]  script name

Options:
      --set <COMMAND>  Set command by name
      --rm             Remove command by name
      --show           Show info by name
  -a, --all            Run on all scripts(ignored if --add)
  -h, --help           Print help
  -V, --version        Print version
```

Run script
```
launch <script>
```

Set script
```
launch <script> --set <command>
```

Remove script
```
launch <script> --rm
```

Remove all script
```
launch --rm -a
```

Show info
```
launch <script> --show
```

Show list
```
launch --show -a
```
