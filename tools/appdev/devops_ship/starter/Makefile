.PHONY: test run image up down load

test:            ## run the unit tests
	python -m unittest -v

run:             ## run the API locally
	python main.py

image:           ## build the container image
	docker build -t ticket-api:dev .

up:              ## start the API with Prometheus and Grafana
	docker compose up -d --build

down:
	docker compose down

load:            ## send some traffic so the dashboards have something to show
	for i in $$(seq 1 50); do \
	  curl -s -X POST localhost:8080/tickets -d '{"title":"Load test","priority":"low"}' > /dev/null; \
	  curl -s localhost:8080/tickets > /dev/null; curl -s localhost:8080/nope > /dev/null; \
	done
