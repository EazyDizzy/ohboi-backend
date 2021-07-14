### Migrations

- Generate: `diesel migration generate <name>`
- Run: `diesel migration run`

### DB Pool testing

```
SELECT  pid         as process_id,
        usename     as username,
        datname     as database_name,
        client_addr as client_address,
        application_name,
        backend_start,
        state,
        state_change
FROM pg_stat_activity;
```

### To clear the docker cache mount:

```
docker builder prune --filter type=exec.cachemount
```

### TO clear docker dead things

```
docker rm $(docker ps -qa --no-trunc --filter "status=exited")
docker volume ls -qf "dangling=true" | xargs docker volume rm
docker rmi $(docker images --filter "dangling=true" -q --no-trunc)
```

### To check image security

install https://aquasecurity.github.io/trivy/v0.18.3/

```
trivy ohboi_backend_ohboi_backend > trivy_security.txt
```

### To start parse job manually

```
docker exec -ti ohboi bash
/app/daemon producer -p PullExchangeRates
/app/daemon producer -p ParseCategory
```

### To clean database

1) Revert all migrations:

```
for i in {1..7}; do bin/diesel migration revert; done
```

2) Run all migrations

```
bin/diesel migration run
```

### To switch between channels/versions

https://github.com/rust-lang/rust/blob/master/RELEASES.md

```
rustup install nightly-2021-03-25
rustup default nightly-2021-02-11
rustup default stable-2021-03-25
```

### To run tests with coverage & println enabled

https://doc.rust-lang.org/nightly/unstable-book/compiler-flags/instrument-coverage.html

```
RUSTFLAGS="-Z instrument-coverage" LLVM_PROFILE_FILE="coverage/ohboi-%m.profraw" cargo test -- --nocapture
cargo profdata -- merge -sparse coverage/ohboi-*.profraw -o coverage/ohboi.profdata
bash coverage.sh 
```

### To check known issues

```
cargo clippy
```
