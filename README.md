<a href="https://github.com/EazyDizzy/ohboi-backend/actions/workflows/ci.yml">
  <img src="https://github.com/EazyDizzy/ohboi-backend/actions/workflows/ci.yml/badge.svg/"/>
</a>

## Docker

### Inside

To enter container

```
docker exec -ti ohboi bash
```

- Migrations
    - Generate: `diesel migration generate <name>`
    - Revert: `for i in {1..8}; do bin/diesel migration revert; done`
    - Run: `bin/diesel migration run`

- To start parse job manually
    - to pull exchange rates `/app/daemon producer -p PullExchangeRates`
    - to parse everything `/app/daemon producer -p ParseCategory`

### Outside

- To clear the docker cache mount:

```
docker builder prune --filter type=exec.cachemount
```

- To clear docker dead things

```
docker rm $(docker ps -qa --no-trunc --filter "status=exited")
docker volume ls -qf "dangling=true" | xargs docker volume rm
docker rmi $(docker images --filter "dangling=true" -q --no-trunc)
```

- To check image security

install https://aquasecurity.github.io/trivy/v0.18.3/

```
trivy ohboi_backend_ohboi_backend > trivy_security.txt
```

## Development

### To switch between channels/versions

https://github.com/rust-lang/rust/blob/master/RELEASES.md

```
rustup install nightly-2020-12-31
rustup default nightly-2020-12-31
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