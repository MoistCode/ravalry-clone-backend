# ravalry-clone-backend

# Resources
* [UML Diagram](https://drawsql.app/sm/diagrams/ravalry#)

# Features

## Patterns
* User-owned
* Has highlight image and a subset of images
* Can be favorited
* Can be tagged with categories


# Docker
https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd

# Instructions
## Building Docker image
1. Navigate to `ravalry-clone-backend/`
2. Ensure port binding is set to "0.0.0.0:8080" (127.0.0.1:8080 is used for dev)
3. Run `docker build --tag moistcode/ravalry-clone:<IMAGE_VERSION> .` (ex. `docker build --tag moistcode/ravalry-clone:0.0.1 .`)

## Uploading Docker image to DockerHub
1. `docker push moistcode/ravalry-clone:<IMAGE_VERSION>` (ex. `docker push moistcode/ravalry-clone:0.0.1`)

## Running image locally
1. Pull from repo (DockerHub) using `docker pull moistcode/ravalry-clone:<IMAGE_VERSION>` (ex. `docker pull moistcode/ravalry-clone:0.0.1`). Can check for the different version/tags by going to https://hub.docker.com/r/moistcode/ravalry-clone/tags.
1. Run `docker run -e DATABASE_URL=test.db -p 8080:8080 --name <CONTAINER_NAME> moistcode/ravalry-clone:<IMAGE_VERSION>` (ex. `docker run -e DATABASE_URL=test.db -p 8080:8080 --name ravalry-clone ravalry-clone:0.0.1`).

