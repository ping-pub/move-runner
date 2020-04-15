# Building Project With Move Runner

```
./move build 
```
or add `-h` or `--home` &lt;path_to_your_project_root&gt;

```
 $ move build -h sample
   Compiling move-runner v0.0.3 (/Users/liangping/workspace/move-runner)
    Finished dev [unoptimized + debuginfo] target(s) in 3m 55s
     Running `target/debug/move build -h sample`
     Loading config from "sample/Move.toml"
   Preparing  Current address: 0xcf1fe4b268ee11f5eb2cfbd7279cd789
   Preparing  Data store: Empty
     Loading modules from sample/src/scripts
   Compiling "sample/src/modules/hello_world.mvir"
     Loading scripts from sample/src/scripts
   Compiling "sample/src/scripts/test_hello_argument.mvir"
   Compiling "sample/src/scripts/test_hello_world.mvir"
Build finished.

```

You can found all compiled modules/scripts(.mv) and source code map files(.mvsm) in `target` directory.
