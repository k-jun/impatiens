# .PHONY: impatiens
impatiens-help:
	./target/debug/main -h

impatiens:
	./target/debug/main -d

test:
	echo hello
	