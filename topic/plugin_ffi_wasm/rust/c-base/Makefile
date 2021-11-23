#https://www.cprogramming.com/tutorial/shared-libraries-linux-gcc.html

base_dir = rust-call-c
c_dir = shared-c

all:
	@gcc -c -Wall -Werror -fpic $(base_dir)/$(c_dir)/echo.c
	@gcc -shared -o libecho.so echo.o