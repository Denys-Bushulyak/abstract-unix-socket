init:
	sudo apt update -y
	sudo apt install -y socat
	dotnet tool install femto --global
	dotnet new tool-manifest
	dotnet tool install fable
	# Download and install nvm:
	curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.2/install.sh | bash

	# in lieu of restarting the shell
	\. "$HOME/.nvm/nvm.sh"

	# Download and install Node.js:
	nvm install 22

	# Verify the Node.js version:
	node -v # Should print "v22.14.0".
	nvm current # Should print "v22.14.0".

	# Verify npm version:
	npm -v # Should print "10.9.2".

server:
	cargo run --bin server
client:
	cargo run --bin client
echo:
	echo "Hello" | socat - ABSTRACT-CONNECT:hidden
build:
	cd fsharp && \
	dotnet fable --run npx vite build && \
	npx esbuild Program.fs.js --bundle --outfile=Program.js --format=esm