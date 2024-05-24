CFLAGS = \
    -Wall -Wextra -pedantic -Werror \
    -Wconversion -Wsign-conversion -Wswitch-default \
    -Wswitch-enum -Wno-mismatched-new-delete -Wimplicit-fallthrough \
    -UNDEBUG -Wno-sign-conversion -std=c++20

graph: graph.cpp
	${CXX} ${CFLAGS} -o $@ $<

.PHONY: clean
clean:
	rm -f graph
