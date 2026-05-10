.PHONY: start-web

start-web:
	.github/workflows/scripts/build-wasm.sh
	lsof -i :8080 -t | xargs kill -9 || true
	cd web && python3 -m http.server 8080
