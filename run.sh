sleep 20
./bin/diesel migration run

./daemon queue_config
./daemon characteristic_enum_sync
(
  env
  echo "0 0 * * *  /app/daemon producer -p PullExchangeRates 2>&1 | tee /app/logs/PullExchangeRatesProducer.log &"
  echo "10 0 * * *  /app/daemon producer -p ParseCategory 2>&1 | tee /app/logs/ParseCategoryProducer.log &"
) | crontab -

./daemon consumer -c ParseCategory 2>&1 | tee /app/logs/ParseCategoryConsumer.log &
./daemon consumer -c ParsePage 2>&1 | tee /app/logs/ParsePageConsumer.log &
./daemon consumer -c ParseImage 2>&1 | tee /app/logs/ParseImageConsumer.log &
./daemon consumer -c PullExchangeRates 2>&1 | tee /app/logs/PullExchangeRatesConsumer.log &
# TODO auto restart panicky and died processes

cron start
./http 2>&1 | tee /app/logs/http.log
echo "scripts launched"
