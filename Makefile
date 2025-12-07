.PHONY: up down build

up:
	docker-compose up --build -d

down:
	docker-compose down

build:
	docker-compose build

logs:
	docker-compose logs -f

