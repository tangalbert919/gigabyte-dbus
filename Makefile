DEBUG ?= 0
ifeq ($(DEBUG),0)
	ARGS += --release
	TARGET = release
endif

all: build

clean:
	cargo clean

install:
	groupadd --system gigabyte-wmi
	usermod -aG gigabyte-wmi "$$user"
	install -Dm755 ./target/release/gigabyted /usr/bin/gigabyted
	install -Dm644 gigabyted.rules /etc/udev/rules.d/gigabyted.rules
	install -Dm644 gigabyted.service /usr/lib/systemd/system/gigabyted.service
	install -Dm644 gigabyted.conf /usr/share/dbus-1/system.d/gigabyted.conf

uninstall:
	rm -f /usr/bin/gigabyted
	rm -f /etc/udev/rules.d/gigabyted.rules
	rm -f /usr/lib/systemd/system/gigabyted.service
	rm -f /usr/share/dbus-1/system.d/gigabyted.conf
	groupdel gigabyte-wmi

build:
	cargo build $(ARGS)
	strip -s ./target/release/gigabyted

.PHONY = all clean install uninstall build
