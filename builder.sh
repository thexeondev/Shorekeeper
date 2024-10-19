docker build -t camellya-builder:1.3.0-SNAPSHOT -f Dockerfile-builder .

docker build -t camellya-config-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=config-server -f Dockerfile-service .
docker build -t camellya-hotpatch-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=hotpatch-server -f Dockerfile-service .
docker build -t camellya-login-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=login-server -f Dockerfile-service .
docker build -t camellya-gateway-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=gateway-server -f Dockerfile-service .
docker build -t camellya-game-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=game-server -f Dockerfile-service .

docker rmi camellya-builder:1.3.0-SNAPSHOT

# Persistence for the application
docker volume create camellya-postgres-vol