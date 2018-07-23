#!/bin/bash
if [ $# -ne 2 ]; then
    echo "Example usage: $0 800x600 i3-chrome"
    exit 1
fi
echo "Proceeding to run $2 with initial size of $1"

while :
do
    RAND=`shuf -i 1000-2000000000 -n 1`
    FILE="/tmp/.X11-unix/X$RAND"
    echo "Trying DISPLAY=:$RAND..."
    if [ ! -f $FILE ]; then
        break
    fi
done

echo "Starting Xephyr :$RAND"
Xephyr :$RAND -screen $1 -ac -resizeable -no-host-grab &

while :
do
    if [ -e $FILE ]; then
        sleep 0.3
        DISPLAY=:$RAND i3 -c $2
        break
    fi
    sleep 0.1
done