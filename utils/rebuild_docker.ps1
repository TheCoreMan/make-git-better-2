Set-PSDebug -Trace 2

Push-Location -Path .\scripts
cargo run --bin generate-pre-receive-hook -- --verbose ..\levels\game-config.toml .\src\bin\templates\hook.tmpl
Pop-Location

& docker rm mgbtest -f
& docker build --tag mgb:0.1 --build-arg CACHE_DATE=$(date +%Y-%m-%d:%H:%M:%S) .
& docker run --detach --name mgbtest --publish 7777:22 mgb:0.1
