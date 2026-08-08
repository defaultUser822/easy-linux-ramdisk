run:
	cargo run --profile dev-sccache
build:
	cargo build --profile dev-sccache
fix-perms $file:
    sudo setcap 'cap_sys_admin=ep' {{file}}
