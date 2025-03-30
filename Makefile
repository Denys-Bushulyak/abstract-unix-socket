init:
	sudo apt update -y
	sudo apt install -y socat
server_rust:
	cd rust && cargo run --bin server
client_rust:
	cd rust && cargo run --bin client
fshart_client:
	cd fsharp && dotnet run
socat_client:
	echo "Hello" | socat - ABSTRACT-CONNECT:hidden