# Move Runner

[![Build Status][build-image]][build-link]
![Apache2][license-image]
![MSRV][rustc-image]
![Maintenance Status: Experimental][maintenance-image]

Are you looking for a tool to manage your Move project?

* Move Runner is a NPM/Cargo like tool that help you to manage your Move project.
* Move Runner is also a simulator that allows you to run Move script/modules on local.
* Move Runner make Move developing very easy, No blockchain, No module publish, No pre-compile.

Here is layout of Move project.

```
.
├── Move.toml
├── genesis.blob
├── src
│   ├── modules
│   └── scripts
└── target
```
* Modules in `src/modules` directory are compiled and loaded/published.
* Scripts in `src/scripts` directory can be executed directly.
* All compiled modules/scripts and source code maps will be generated into `target` directory. and you can publish them to your blockchain.

## Tutorials

* [Quick Start](docs/01_quick_start.md)
* [Integrate With IDE](docs/02_integrate_with_ide.md)
* [Build Project](docs/03_building_project.md)
* [Custom Move Project & Transaction](docs/04_custom_your_project.md)
* [Initial States From genesis](docs/05_initial_states_from_genesis.md)


## Samples

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
