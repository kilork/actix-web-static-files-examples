# actix-web-static-files Examples

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

A collection of [actix-web-static-files](https://github.com/kilork/actix-web-static-files) examples.

## Build with local static-files version

```
cargo update --config 'patch.crates-io.static-files.path="../../static-files-rs/static-files"'
cargo build --config 'patch.crates-io.static-files.path="../../static-files-rs/static-files"'
```

## Examples

1. [resource-dir](resource-dir) - basic example using `resource_dir` function.
1. [npm-resource-dir](npm-resource-dir) - basic example using `npm_resource_dir` function.
1. [generate-resources-mapping](generate-resources-mapping) - uses `generate_resources_mapping` function
to show possibility to have own resource generation function definition.
1. [webpack](webpack) - minimal WebPack example with change detection.
1. [yarn-webpack](yarn-webpack) - [webpack](webpack) example using `yarn` package manager.