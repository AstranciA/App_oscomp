#!/bin/sh
echo "#### OS COMP TEST GROUP START basic-glibc ####"
tests="
brk
chdir
clone
close
dup2
dup
execve
exit
fork
fstat
getcwd
getdents
getpid
getppid
gettimeofday
mkdir_
mmap
mount
munmap
openat
open
pipe
read
times
umount
uname
unlink
wait
waitpid
write
yield
"
for i in $tests
do
	echo "Testing $i :"
	./$i
done
echo "#### OS COMP TEST GROUP END basic-glibc ####"
