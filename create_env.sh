if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <username> <password>"
    exit 1
fi

echo "SECRET_USER="$1 >./.env
echo "SECRET_PASS="$2 >>./.env
echo "DATABASE_URL=postgres://\${SECRET_USER}:\${SECRET_PASS}@127.0.0.1:5432/inventory">>./.env


