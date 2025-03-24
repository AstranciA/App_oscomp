## Build


1. fetch the AstracE source code:
```sh
make AX_SOURCE=git:http://github.com/AstranciA/AstrancE.git AX_ROOT=.AstrancE fetch_ax
```
or locally:
```sh
make AX_SOURCE=file:/path/to/AstrancE/ AX_ROOT=.AstrancE fetch_ax
```

(Don't forget the trailing slash in the local path.)

`AX_SOURCE` by default is set to `git:http://github.com/AstranciA/AstrancE.git`, which is the official AstracE repository.
`AX_ROOT` by default is set to `.AstrancE`, which is the directory where AstracE will be downloaded and built.

2. build testcases: 

```sh
make ARCH=riscv64 AX_ROOT=.AstrancE testcase
```

read Makefile for more details. Binary files will be automatically made into
disk.img under `$(AX_ROOT)$` dir.

## Run

shortest way: 
```sh
make ARCH=riscv64 run
```

read AstrancE doc for more details.
