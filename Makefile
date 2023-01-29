PROTOAPIC_VERSION=1.13.0

init_db:
	cargo script --dep rusqlite create_tables.rs