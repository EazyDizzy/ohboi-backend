sleep 5
diesel migration run

cargo run --bin daemon producer 1 -p CrawlerCategory
cargo run --bin daemon consumer 1 -c CrawlerCategory
# TODO auto restart panicky and died processes
cargo run --bin http
