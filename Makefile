db:
	sudo docker compose up -d
db-stop:
	sudo docker compose stop

back:
	cargo run -p backend

front:
	pushd frontend && dx serve && popd
