#include <arpa/inet.h>
// #include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <signal.h>
#include <sys/types.h>
#include <unistd.h>

#define PORT 80

char webpage[] = "HTTP/1.1 200 OK\r\n"
                 "Content-Type: text/html; charset=UTF-8\r\n\r\n"
                 "<!DOCTYPE html>\r\n"
                 "<html><head><title>WebPage</title>\r\n"
                 "<style>body { background-color: #FFFF00 }</style></head><body>hello world</body></html>\r\n";


void sigint_handler(int signo) {
    printf("Shutting down...");
    exit(0);
}

int main(int argc, char *argv[]) {
 
  // disable buffering for stdout
  setbuf(stdout, NULL);

  // setup sigint handler
  signal(SIGINT, sigint_handler);


  printf("starting\n");


  struct sockaddr_in server_addr, client_addr;
  socklen_t sin_len = sizeof(client_addr);
  int fd_server, fd_client;
  char buf[2048];
  int on = 1;

  fd_server = socket(AF_INET, SOCK_STREAM, 0);
  if (fd_server < 0) {
    perror("socket");
    exit(1);
  }
  printf("created socket\n");

  setsockopt(fd_server, SOL_SOCKET, SO_REUSEADDR, &on, sizeof(int));

  server_addr.sin_family = AF_INET;
  server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
  server_addr.sin_port = htons(PORT);

  if (bind(fd_server, (struct sockaddr *)&server_addr, sizeof(server_addr)) !=
      0) {
    perror("bind");
    exit(1);
  }

  printf("listening on port %d\n", PORT);

  if (listen(fd_server, 1024) != 0) {
    perror("listen");
    exit(1);
  }

  int count = 0;

  while (1) {
    fd_client = accept(fd_server, (struct sockaddr *)&client_addr, &sin_len);
    count += 1;

    if (fd_client < 0) {
      perror("Can't connect...\n");
      continue;
    }


    if (count % 1000 == 0) {
        char* pretty_ip = inet_ntoa(client_addr.sin_addr);
        printf("[%s] connection %d\n", pretty_ip, count);
    }



    memset(buf, 0, 2048);
    read(fd_client, buf, 2047);

    write(fd_client, webpage, sizeof(webpage) - 1);

    close(fd_client);
  }

  return 0;
}
