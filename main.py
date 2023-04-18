from asyncio import new_event_loop, sleep
from random import randint
from time import time
import os

from interface import GigabyteInterface

from sdbus import request_default_bus_name_async

loop = new_event_loop()

export_object = GigabyteInterface()


async def clock() -> None:
    """
    This coroutine will sleep a random time and emit
    a signal with current clock
    """
    while True:
        await sleep(randint(2, 7))  # Sleep a random time
        current_time = int(time())  # The interface we defined uses integers
        export_object.clock.emit(current_time)


async def write_to_sysfs(option, value) -> None:
    """Write to sysfs"""
    print("Not yet implemented")
    if option == 0:    # Fan mode
        print("Not yet implemented")
    elif option == 1:  # Fan speed
        print("Not yet implemented")
    elif option == 2:  # Charging mode
        print("Not yet implemented")
    elif option == 3:  # Charging limit
        print("Not yet implemented")
    else:
        print("Unsupported option")


async def startup() -> None:
    """Perform async startup actions"""
    # Acquire a known name on the bus
    # Clients will use that name to address this server
    await request_default_bus_name_async('org.gigabyte.daemon')
    # Export the object to dbus
    export_object.export_to_dbus('/')


# TODO: Only allow running as root
print(os.getuid())
loop.run_until_complete(startup())
task_clock = loop.create_task(clock())
loop.run_forever()
