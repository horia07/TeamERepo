#include <arpa/inet.h>
#include <errno.h>
#include <fcntl.h>
#include <netdb.h>
#include <netinet/in.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/epoll.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#define _GNU_SOURCE 
#define PORT 80
#define MAXEVENTS 1024

char webpage[2048] =
    "HTTP/1.1 200 OK\r\n"
    "Content-Type: text/html; charset=UTF-8\r\n\r\n"
    "<!DOCTYPE html>\r\n"
    "<html><head><title>WebPage</title>\r\n"
    "<style>body { background-color: #FFFF00 }</style></head><body><h1>Hello world</h1></body></html>\r\n";

void sigint_handler(int signo) {
  printf("Shutting down...");
  exit(0);
}

void handle_connection(int fd) {}

int main(int argc, char *argv[]) {

  // disable buffering for stdout
  setbuf(stdout, NULL);

  // setup sigint handler
  signal(SIGINT, sigint_handler);

  printf("starting\n");

  struct sockaddr_in server_addr, client_addr;
  socklen_t sin_len = sizeof(client_addr);
  int fd_server, fd_client, s;
  struct epoll_event event;
  struct epoll_event *events;
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


  int flags =
      fcntl(fd_server, F_GETFL, 0); // change socket fd to be non-blocking
  flags |= O_NONBLOCK;
  fcntl(fd_server, F_SETFL, flags);

  if (listen(fd_server, 1024) != 0) {
    perror("listen");
    exit(1);
  }

  int efd = epoll_create1(0); // create epoll instance

  if (efd == -1) {
    perror("epoll_create");
    abort();
  }

  event.data.fd = fd_server;
  event.events =
      EPOLLIN |
      EPOLLET; // just interested in read's events using edge triggered mode

  s = epoll_ctl(efd, EPOLL_CTL_ADD, fd_server,
                &event); // Add server socket FD to epoll's watched list
  if (s == -1) {
    perror("epoll_ctl");
    abort();
  }

  events = (struct epoll_event *)calloc(MAXEVENTS, sizeof(event));

  int count = 0;

  while (1) {
    int nfds = epoll_wait(efd, events, MAXEVENTS, -1);

    for (int n = 0; n < nfds; n++) {
      if (events[n].data.fd == fd_server) {

	while (1) {
            int conn_sock =
                accept4(fd_server, (struct sockaddr *)&client_addr, &sin_len, SOCK_NONBLOCK | SOCK_CLOEXEC);

            if (conn_sock == -1) {
	        if (errno == EAGAIN || (errno == EWOULDBLOCK)) {
		    break;
		} else {
                    perror("accept");
                    exit(1);
		}
            }

            event.events = EPOLLIN | EPOLLRDHUP | EPOLLET;
            event.data.fd = conn_sock;

            if (epoll_ctl(efd, EPOLL_CTL_ADD, conn_sock, &event) == -1) {
                perror("epoll_ctl: add conn_sock");
                exit(1);
            }
	}
      } else {
        int fd = events[n].data.fd;

	if (count % 1000 == 0) {
            char* pretty_ip = inet_ntoa(client_addr.sin_addr);
            printf("[%s] connection %d\n", pretty_ip, count);
	}

        memset(buf, 0, 2048);
        int r = read(fd, buf, 2047);
        write(fd, webpage, strlen(webpage));

	// epoll_ctl(efd, EPOLL_CTL_DEL, fd, &events[n]);
        close(fd);
	count += 1;
      }
    }
  }

  free(events);
  close(efd);
  close(fd_server);

  return 0;
}
