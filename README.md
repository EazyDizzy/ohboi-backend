# Migrations
 - Generate: `diesel migration generate <name>`
 - Run: `diesel migration run`
 
# DB Pool testing
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