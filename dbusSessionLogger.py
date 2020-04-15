import dbus
import gobject
from dbus.mainloop.glib import DBusGMainLoop
from datetime import datetime
import signal
import sys
import argparse

IFACE_SCREENSAVER = "org.gnome.ScreenSaver"
MEMBER_SCREENSAVER = "ActiveChanged"

class SessionLockLogger:
    def __init__(self, loop, logFile):
        self.loop = loop
        self.log = logFile
        signal.signal(signal.SIGHUP, self.stop)
        signal.signal(signal.SIGINT, self.stop)
        signal.signal(signal.SIGTERM, self.stop)
        signal.signal(signal.SIGQUIT, self.stop)
        self.session_bus = dbus.SessionBus()
        self.session_bus.add_signal_receiver(self.handler, dbus_interface=IFACE_SCREENSAVER, message_keyword='message')
        self.writeLog("Start")

    def writeLog(self, command):
        time = datetime.now().strftime("%Y/%m/%d-%H:%M")
        self.log.write(time+" "+command+"\n")
        self.log.flush()

    def stop(self, signalNumber, frame):
        self.writeLog("Stop")
        self.loop.quit()

    def handler(self, status, message=None):
        if(message.get_member() == MEMBER_SCREENSAVER):
            if(status):
                self.writeLog("Lock")
            else:
                self.writeLog("Unlock")

def main(args):
    DBusGMainLoop(set_as_default=True)
    loop = gobject.MainLoop()
    sessionLogger = SessionLockLogger(loop, args.outfile)
    loop.run()

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Record locks and unlocks of screen")
    parser.add_argument('-f', default=sys.stdout, dest='outfile', type=argparse.FileType('a'), help="File to store the info")
    main(parser.parse_args())