# Move-runner

[![Build Status][build-image]][build-link]
![Apache2][license-image]
![MSRV][rustc-image]
![Maintenance Status: Experimental][maintenance-image]

Move runner is a dev-tool that manage your Move project and allows you to run Move script/modules without running Libra blockchain.

# Usage

## 1. Download the lastest version. 

Please visit the release page: https://github.com/ping-pub/move-runner/releases

Mac OS:

```
cd /usr/local/bin
wget https://github.com/ping-pub/move-runner/releases/download/v0.0.1-RC1/move-macos-amd64
mv move-macos-amd64 move
```
Linux:
```
cd /usr/local/bin
wget https://github.com/ping-pub/move-runner/releases/download/v0.0.1-RC1/move-linux-amd64
mv move-linux-amd64 move
```
Windows:
```
C:\windows\explorer.exe https://github.com/ping-pub/move-runner/releases/download/v0.0.1-RC1/move-windows-amd64
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

## 3. Build project

```
./move build 
```
or add --home 'path to your workspace'

```
$ ./move build --home /Users/liangping/workspace/move-runner/sample
Build { home: "/Users/liangping/workspace/move-runner/sample" }
loaded config from "/Users/liangping/workspace/move-runner/sample/Move.toml"

 Compiling with address: 0xcf1fe4b268ee11f5eb2cfbd7279cd789

loading modules in /Users/liangping/workspace/move-runner/sample/src/modules/**/*.mvir
Compiling: "/Users/liangping/workspace/move-runner/sample/src/modules/hello_world.mvir"

loading scripts in /Users/liangping/workspace/move-runner/sample/src/scripts/**/*.mvir
Compiling: "/Users/liangping/workspace/move-runner/sample/src/scripts/test_hello_argument.mvir"
Compiling: "/Users/liangping/workspace/move-runner/sample/src/scripts/test_hello_world.mvir"
```

## 4. Execute Move script.
```
$ ./move run src/scripts/test_hello_world.mvir 
Run { home: ".", source_path: "src/scripts/test_hello_world.mvir", args: [] }
loaded config from "./Move.toml"

 Compiling with address: 0xcf1fe4b268ee11f5eb2cfbd7279cd789

loading modules in ./src/modules/**/*.mvir
Compiling: "src/modules/hello_world.mvir"
Compiling: "src/scripts/test_hello_world.mvir"
output from move vm: Ok(())

```
You can run `./move run test_hello_world.mvir` which will load source_code in script directory.

Execute script with argument:
```
$ ./move run test_hello_argument.mvir 1 true 0x0 
Run { home: ".", source_path: "test_hello_argument.mvir", args: ["1", "true", "0x0"] }
{U8Vector: 0x0101}
loaded config from "./Move.toml"

 Compiling with address: 0xcf1fe4b268ee11f5eb2cfbd7279cd789

loading modules in ./src/modules/**/*.mvir
Compiling: "src/modules/hello_world.mvir"
Compiling: "./src/scripts/test_hello_argument.mvir"
output from move vm: Ok(())
```

You can find sample project: https://github.com/ping-pub/move-runner/tree/master/sample

## License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

[build-image]: https://github.com/ping-pub/move-runner/workflows/Rust/badge.svg?branch=master&event=push
[build-link]: https://github.com/ping-pub/move-runner/actions
[license-image]:https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/ping-pub/move-runner/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.39+-blue.svg
[maintenance-image]: https://img.shields.io/badge/maintenance-experimental-blue.svg
