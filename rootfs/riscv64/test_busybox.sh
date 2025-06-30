#!/busybox sh

./busybox echo "#### OS COMP TEST GROUP START busybox-musl ####"
while read line
do
    eval "./busybox $line"
    RTN=$?
    if [[ $RTN -ne 0 && "$line" != "false" ]] ;then
        echo "testcase busybox $line fail"
    else
        echo "testcase busybox $line success"
    fi
done < /busybox_cmd.txt
./busybox echo "#### OS COMP TEST GROUP END busybox-musl ####"
