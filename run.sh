sleep 20
diesel migration run

./target/release/daemon queue_config
./target/release/daemon producer -p ParseCategory 2>&1 | tee logs/ParseCategoryProducer.log &
./target/release/daemon consumer -c ParseCategory 2>&1 | tee logs/ParseCategoryConsumer.log &
./target/release/daemon consumer -c ParsePage 2>&1 | tee logs/ParsePageConsumer.log &
./target/release/daemon consumer -c ParseImage 2>&1 | tee logs/ParseImageConsumer.log &
# TODO auto restart panicky and died processes

./target/release/http 2>&1 | tee logs/http.log
#cargo build
#
#./target/debug/daemon queue_config
#./target/debug/daemon producer -p ParseCategory 2>&1 | tee logs/ParseCategoryProducer.log &
#./target/debug/daemon consumer -c ParseCategory 2>&1 | tee logs/ParseCategoryConsumer.log &
#./target/debug/daemon consumer -c ParsePage 2>&1 | tee logs/ParsePageConsumer.log &
#./target/debug/daemon consumer -c ParseImage 2>&1 | tee logs/ParseImageConsumer.log &
## TODO auto restart panicky and died processes
#
#./target/debug/http 2>&1 | tee logs/http.log
#sleep 2000
