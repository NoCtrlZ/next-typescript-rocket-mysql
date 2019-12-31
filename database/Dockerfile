FROM mysql:8.0.14

ENV MYSQL_DATABASE db

ENV MYSQL_ALLOW_EMPTY_PASSWORD yes

RUN echo default-authentication-plugin=mysql_native_password >> /etc/mysql/my.cnf &&\
    echo character-set-server = utf8 >> /etc/mysql/my.cnf

COPY ./seed ./docker-entrypoint-initdb.d
