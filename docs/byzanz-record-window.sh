#!/bin/bash

# http://askubuntu.com/questions/107726/how-to-create-animated-gif-images-of-a-screencast#answer-201018

# 1. Run byzanz-record-window 30 -c output.gif
# 2. Go to the window (alt-tab) you want to capture. Click on it.
# 3. Wait 10 seconds (hard-coded in $DELAY), in which you prepare for recording.
# 4. After the beep (defined in the beep function), byzanz will start.
# 5. After 30 seconds (that's the meaning of 30 in step 1), byzanz ends. A beep will be broadcast again.

# Delay before starting
DELAY=10

# Sound notification to let one know when recording is about to start (and ends)
beep() {
    paplay /usr/share/sounds/KDE-Im-Irc-Event.ogg &
}

# Duration and output file
if [ $# -gt 0 ]; then
    D="--duration=$@"
else
    echo Default recording duration 10s to /tmp/recorded.gif
    D="--duration=10 /tmp/recorded.gif"
fi
XWININFO=$(xwininfo)
read X < <(awk -F: '/Absolute upper-left X/{print $2}' <<< "$XWININFO")
read Y < <(awk -F: '/Absolute upper-left Y/{print $2}' <<< "$XWININFO")
read W < <(awk -F: '/Width/{print $2}' <<< "$XWININFO")
read H < <(awk -F: '/Height/{print $2}' <<< "$XWININFO")

echo Delaying $DELAY seconds. After that, byzanz will start
for (( i=$DELAY; i>0; --i )) ; do
    echo $i
    sleep 1
done

beep
byzanz-record --verbose --delay=0 --x=$X --y=$Y --width=$W --height=$H $D
beep