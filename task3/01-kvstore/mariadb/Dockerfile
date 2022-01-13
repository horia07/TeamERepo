FROM praszkowski/docker-mariadb-myrocks

COPY crTable.sql /docker-entrypoint-initdb.d/

ENV MYSQL_ROOT_PASSWORD admin
ENV MYSQL_DATABASE userdb
ENV MYSQL_USER user
ENV MYSQL_PASSWORD user

EXPOSE 3306

