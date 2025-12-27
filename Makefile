.PHONY: dev build clean dmg

dev:
	@echo "Stopping any existing processes on port 5173..."
	lsof -ti:5173 | xargs kill -9 2>/dev/null || true
	@echo "Starting dev server..."
	cd ui && npm run dev &
	cd src-tauri && cargo tauri dev

build:
	cd ui && npm run build
	cd src-tauri && cargo tauri build

dmg:
	@echo "Creating DMG..."
	./scripts/create-dmg.sh

sign-dmg:
	@echo "Creating signed DMG..."
	./scripts/create-dmg.sh --sign

clean:
	rm -rf ui/dist
	rm -rf src-tauri/target
