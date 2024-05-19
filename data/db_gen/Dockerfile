FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=root

# COPY A_local -to-> B_image
# NOTE: the mysql image will auto execute 
#       `.sq;, .sq;.gz, and .sh` files in the `docker-entrypoint-initdb.d` dir
COPY ./database_students.sql /docker-entrypoint-initdb.d/
