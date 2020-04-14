import dbus
import dbus.service
import gobject
from dbus.mainloop.glib import DBusGMainLoop
from datetime import datetime
import os

FILENAME = os.environ['HOME']+"/.sessionLock.log"

OPATH_SERVICE = "/org/sessionLogger"
IFACE_SERVICE = "org.sessionLogger"
BUS_NAME_SERVICE = "org.sessionLogger"

IFACE_SCREENSAVER = "org.gnome.ScreenSaver"
MEMBER_SCREENSAVER = "ActiveChanged"

class SessionLockLogger(dbus.service.Object):
    def __init__(self, loop, logFile):
        self.loop = loop
        self.log = logFile
        self.session_bus = dbus.SessionBus()
        self.session_bus.request_name(BUS_NAME_SERVICE)
        bus_name = dbus.service.BusName(BUS_NAME_SERVICE, bus=self.session_bus)
        dbus.service.Object.__init__(self, bus_name, OPATH_SERVICE)
        self.session_bus.add_signal_receiver(self.stop, "Stop")
        self.session_bus.add_signal_receiver(self.handler, dbus_interface=IFACE_SCREENSAVER, message_keyword='message')
        self.writeLog("Start")
    def writeLog(self, command):
        self.log.write(command+" "+str(datetime.now())+"\n")
        self.log.flush()

    def stop(self):
        self.writeLog("Stop")
        self.loop.quit()

    def handler(self, status, message=None):
        if(message.get_member() == MEMBER_SCREENSAVER):
            if(status):
                self.writeLog("Lock")
            else:
                self.writeLog("Unlock")

def main():
    DBusGMainLoop(set_as_default=True)
    loop = gobject.MainLoop()
    with open(FILENAME, "a") as logFile:
        sessionLogger = SessionLockLogger(loop, logFile)
        loop.run()

if __name__ == '__main__':
    main()