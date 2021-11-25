FROM gcc
COPY . /usr/src/app

WORKDIR /usr/src/app

RUN gcc -o main -fanalyzer -Wall -Wextra -O2 main.c

CMD ["./main"]


