docker compose -f ./docker-compose.yml -p acticxwebtest up -d
echo "Waiting for docker setup to finish..."
sleep 10
echo "Running Diesel setup"
diesel setup

echo "Finished Setup!"