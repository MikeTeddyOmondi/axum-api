# Default
default:
  just --list

# Run dev mode
run-dev-mode:
  cargo watch -q -c -w ./src -x run  

# Build Docker Image
build-image:
  docker build -t ranckosolutionsinc/axum-todo-api:v1.0.0 . 

# Run Docker Container
run-container:
  docker run -d -p 5050:5050 --restart always -e DATABASE_URL=sqlite.db --name axum-todo-api ranckosolutionsinc/axum-todo-api:v1.0.0 

# Docker compose 
run-compose:
  docker compose up -d

# Docker compose down
run-compose-down:
  docker compose down
