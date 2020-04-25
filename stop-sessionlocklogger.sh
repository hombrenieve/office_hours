#!/bin/bash

dbus-send --session --type=signal --dest=org.sessionLockLogger /org/sessionLockLogger org.sessionLockLogger.Stop