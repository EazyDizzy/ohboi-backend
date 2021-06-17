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