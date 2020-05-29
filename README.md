# make-git-better-2

Git CTF 🚩 but good this time.

## Build

### How to build the challenge Docker

#### Create the hook script

In the `scripts` directory...

```powershell powershell
cargo run --bin generate-pre-receive-hook -- --verbose ..\levels\game-config.toml .\src\bin\templates\hook.tmpl
```

```sh sh
cargo run --bin generate-pre-receive-hook -- --verbose ../levels/game-config.toml src/bin/templates/hook.tmpl
```

#### Build and run docker image

In the root directory.

##### Build docker

```sh
docker build --tag mgb:0.1 --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S) .
```

##### Run docker

```sh
docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
```

Useful oneliner:

```sh sh
docker rm -f mgbtest && docker build --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S) --tag mgb:0.1 . && docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
```

##### Connect to the running instance

```sh
ssh player@localhost -p 7777
```

### How to build the web content

#### Build the level browser

```sh
cargo run --bin generate-levels-graph -- -v ../levels/game-config.toml src/bin/templates/graph.tmpl
```

```sh
make-git-better-scripts 0.1.0
A script to generate a levels graph from a game config.

USAGE:
    generate-levels-graph [FLAGS] <game-config-path> <template-path> [output-path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show more information about the actions taken

ARGS:
    <game-config-path>    Path to game config file to read
    <template-path>       Path to the graph template file to read
    <output-path>         Path to output file (creates if doesn't exist) [default: output/levelgraph.html]
```

## Test

### Unit tests

```sh
cd scripts
cargo test
```

### Test levels

- [ ] TODO @ShayNehmad

## Develop

### Add a new stage

```powershell powershell
cargo run --bin generate-new-level -- ..\levels\game-config.toml .\src\bin\templates\level_checker.tmpl .\src\bin\templates\level_test.tmpl .\src\bin\templates\level_page.tmpl .\src\bin\resources\words_alpha.txt ..\levels\ -v
```

```sh
A script to generate a new level.

USAGE:
    generate-new-level.exe [FLAGS] <game-config-path> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Show more information about the actions taken

ARGS:
    <game-config-path>         Path to game config file to read
    <checker-template-path>    Path to the chekcer template file [default: templates/level_checker.tmpl]
    <test-template-path>       Path to the test template file [default: templates/level_test.tmpl]
    <page-template-path>       Path to the page template file [default: templates/level_page.tmpl]
    <words-path>               Path to a file with english words separated by newline [default:
                               resources/words_alpha.txt]
    <levels-directory>         Levels directory [default: ../levels/]
```
