# gigabyte-dbus

This program allows third-party applications to interact with my [kernel driver](https://github.com/tangalbert919/gigabyte-laptop-wmi) without the need to run as root.

An example of such application is [Aero Control Center](https://github.com/tangalbert919/AeroControlCenter).

## Installation

You must have Rust installed on your machine, along with `make`.
```sh
make
sudo make install
```

## Uninstallation

```sh
sudo make uninstall
```

## Documentation
The methods can be found in `interface.xml`. Refer to the kernel driver's documentation on how to use the methods properly.

## TODO
- [ ] Start moving functions from Aero Control Center to here
- [ ] Build example application for interaction with daemon
- [ ] Make this useful for third-party applications

### Additional info

Inspired by [asusctl](https://gitlab.com/asus-linux/asusctl)
