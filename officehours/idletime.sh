#!/bin/bash

running=1

function stopIt()
{
    echo "Stopping"
    running=0
}

idleloop() {
    touch /tmp/.{,last_}input
    cmd='stat --printf="%s"'
    idletime=10
    a=2
    t=0
    while [[ $running == 1 ]]
    do
        timeout 1 xinput test-xi2 --root > /tmp/.input
        
        if [[ `eval $cmd /tmp/.input` == `eval $cmd /tmp/.last_input` ]]
        then
            let t++ # increases $t by 1
        else
            t=0     # resets $t
        fi

        mv /tmp/.{,last_}input -f

        if [ $t -ge $idletime ] && [[ $a == "2" ]]
        then
            echo "user has gone idle"
            a=1
        fi
        if [ $t -lt $idletime ] && [[ $a == "1" ]]
        then
            echo "user has come back from idle"
            a=2
        fi
    done
    rm -f /tmp/.{,last_}input
}

trap stopIt SIGINT

idleloop