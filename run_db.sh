if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <username> <password>"
    exit 1
fi

user=$1
pass=$2

docker build --build-arg SecretUser=$user --build-arg SecretPass=$pass -t postgres-actix-test:latest -f Dockerfile.db .
docker run -d --rm -p 5432:5432 --name postgres-container postgres-actix-test:latest
echo "Waiting for docker setup to finish..."
sleep 5
echo "Running Diesel setup"
diesel setup

echo "Finished Setup!"
