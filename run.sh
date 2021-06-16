#sleep 20
#cd /app && diesel migration run

/app/target/release/daemon
/app/target/release/daemon queue_config
/app/target/release/daemon producer -p ParseCategory 2>&1 | tee /app/logs/ParseCategoryProducer.log &
/app/target/release/daemon consumer -c ParseCategory 2>&1 | tee /app/logs/ParseCategoryConsumer.log &
/app/target/release/daemon consumer -c ParsePage 2>&1 | tee /app/logs/ParsePageConsumer.log &
/app/target/release/daemon consumer -c ParseImage 2>&1 | tee /app/logs/ParseImageConsumer.log &
# TODO auto restart panicky and died processes

/app/target/release/http 2>&1 | tee /app/logs/http.log
echo "scripts launched"
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
sleep infinity
