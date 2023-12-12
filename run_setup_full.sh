docker compose -f ./docker-compose.yml -p acticxwebtest up -d
echo "Waiting for docker setup to finish..."
sleep 5
echo "Running Diesel setup"
diesel setup

echo "Finished Setup!"