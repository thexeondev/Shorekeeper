docker build -t wicked-waifus-builder:1.3.0-SNAPSHOT -f Dockerfile-builder .

docker build -t wicked-waifus-config-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=config-server -f Dockerfile-service .
docker build -t wicked-waifus-hotpatch-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=hotpatch-server -f Dockerfile-service .
docker build -t wicked-waifus-login-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=login-server -f Dockerfile-service .
docker build -t wicked-waifus-gateway-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=gateway-server -f Dockerfile-service .
docker build -t wicked-waifus-game-server:1.3.0-SNAPSHOT --build-arg MICROSERVICE=game-server -f Dockerfile-service .

docker rmi wicked-waifus-builder:1.3.0-SNAPSHOT

: Persistence for the application
: docker volume create wicked-waifus-postgres-vol