docker build -t shorekeeper-builder:1.3.0-SNAPSHOT -f Dockerfile-builder .

docker build -t shorekeeper-config-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=config-server -f Dockerfile-service .
docker build -t shorekeeper-hotpatch-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=hotpatch-server -f Dockerfile-service .
docker build -t shorekeeper-login-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=login-server -f Dockerfile-service .
docker build -t shorekeeper-gateway-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=gateway-server -f Dockerfile-service .
docker build -t shorekeeper-game-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=game-server -f Dockerfile-service .

docker rmi shorekeeper-builder:1.3.0-SNAPSHOT

: Persistence for the application
docker volume create shorekeeper-postgres-vol