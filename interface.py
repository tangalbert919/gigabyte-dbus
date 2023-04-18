from sdbus import (DbusInterfaceCommonAsync, dbus_method_async,
                   dbus_property_async, dbus_signal_async)


# This is file only contains interface definition for easy import
# in server and client files

class GigabyteInterface(
    DbusInterfaceCommonAsync,
    interface_name='org.gigabyte.interface'
):
    @dbus_method_async(
        input_signature='s',
        result_signature='s',
    )
    async def upper(self, string: str) -> str:
        return string.upper()

    @dbus_property_async(
        property_signature='s',
    )
    def hello_world(self) -> str:
        return 'Hello, World!'

    @dbus_signal_async(
        signal_signature='i'
    )
    def clock(self) -> int:
        raise NotImplementedError

    @dbus_method_async(input_signature='i', result_signature='i')
    async def write_to_sysfs(self, option: int, value: int) -> int:
        raise NotImplementedError
