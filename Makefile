db:
	sudo docker compose up -d
db-stop:
	sudo docker compose stop

back:
	cargo run -p backend

front:
	pushd frontend && dx serve && popd

minio:
	docker run    -p 9000:9000    -p 9001:9001    -v ~/minio/data:/data    -e "MINIO_ROOT_USER=minioadmin"    -e "MINIO_ROOT_PASSWORD=minioadmin"  \
	quay.io/minio/minio server /data --console-address ":9001"
