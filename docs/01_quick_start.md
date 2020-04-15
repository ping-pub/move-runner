# Move Runner Quick Start
## 1. Download the lastest version. 

Please visit the release page: https://github.com/ping-pub/move-runner/releases

Mac OS:

```
cd /usr/local/bin
wget https://github.com/ping-pub/move-runner/releases/download/v0.0.3/move-macos-amd64 -O move
chmod 755 move
```
Linux:
```
cd /usr/local/bin
wget https://github.com/ping-pub/move-runner/releases/download/v0.0.3/move-linux-amd64 -O move
chmod 755 move
```
Windows:
```
C:\windows\explorer.exe https://github.com/ping-pub/move-runner/releases/download/v0.0.3/move-windows-amd64
```
make sure `move` in your path:
```
iMac:sample liangping$ move --help
move 0.0.1
Move local runner, allows developers to compile and run Move script/modules on local

USAGE:
    move <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    build      Build all modules and scripts in src directory
    compile    Compile script/module only
    help       Prints this message or the help of the given subcommand(s)
    new        New a move project
    run        Compile and run script
```


## 2. Create Move project
Linux/MacOS:

```
$ move new
$ ls
Move.toml	genesis.blob	src		target
```
Windows:
```
$ move.exe new
$ dir
Move.toml	genesis.blob	src		target
```

The project layout is :
```
.
├── Move.toml
├── genesis.blob
├── src
│   ├── modules
│   └── scripts
└── target

```

## 3. Create Hello World Move Script


Create a source file in `src/scripts`, named `hello_world_script.mvir`

```
main(a: u64, b: bool, c: address ) {
    // /*d: vector<u8>*/
    return;
}
```

## 4. Run Hello World Script.
Note: make sure that your current dir is project root dir
```
move run hello_world.mvir 64 true, 0x123

```
You can see output like this:
```
     Loading config from "./Move.toml"
   Preparing  Current address: 0xcf1fe4b268ee11f5eb2cfbd7279cd789
   Preparing  Data store: Empty
     Loading modules from ./src/modules
   Compiling "../src/modules/hello_world.mvir"
   Compiling "../src/scripts/hello_world_script.mvir"
     Running Script: "hello_world_script.mvir" Args: ["1", "true", "0x123"]
      Output 0 WriteSet was generated

```