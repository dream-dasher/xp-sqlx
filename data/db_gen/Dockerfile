FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=root

# COPY A_local -to-> B_image
COPY ./database_students.sql /docker-entrypoint-initdb.d/
