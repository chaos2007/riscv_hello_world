ENV = . .venv/bin/activate
all: kflash
	cargo build --release
	$(ENV); kflash -t -s -p /dev/ttyUSB1 -B goE target/riscv64gc-unknown-none-elf/release/riscv_hello_world

.venv:
	python3 -m venv .venv

kflash: .venv
	$(ENV); pip install kflash
