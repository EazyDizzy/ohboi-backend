diesel migration run
cargo build --release

./target/release/daemon producer -p ParseCategory 2>&1 | tee logs/ParseCategoryConsumer.log &
#./target/release/daemon consumer -c ParseCategory 2>&1 | tee logs/ParseCategoryConsumer.log &
#./target/release/daemon consumer -c ParsePage 2>&1 | tee logs/ParsePageConsumer.log &
#./target/release/daemon consumer -c ParseImage 2>&1 | tee logs/ParseImageConsumer.log &
# TODO auto restart panicky and died processes

#./target/release/http
# 2>&1 | tee logs/http.log &
