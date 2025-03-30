init:
	sudo apt update -y
	sudo apt install -y socat
server:
	cd rust && cargo run --bin server
rust-client:
	cd rust && cargo run --bin client
net-client:
	cd fsharp && dotnet run
socat:
	echo -n "Hello" | socat - ABSTRACT-CONNECT:hidden