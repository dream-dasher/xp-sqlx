# Justfile (Convenience Command Runner)


# home_dir := env_var('HOME')
local_root := justfile_directory()
invocd_from := invocation_directory()
invoc_is_root := if invocd_from == local_root { "true" } else { "false" }
## ANSI Color Codes for use with echo command
GRN := '\033[0;32m' # Green
BLU := '\033[0;34m' # Blue
PRP := '\033[0;35m' # Purple
BRN := '\033[0;33m' # Brown
CYN := '\033[0;36m' # Cyan
NC := '\033[0m'     # No Color

# Default, lists commands.
_default:
        @ just --list --unsorted

# Linting, formatting, typo checking, etc.
check:
    cargo clippy
    cargo fmt
    typos --exclude 'data/*'
    committed

# Watch a file: compile & run on changes.
watch file_to_run:
    cargo watch --quiet --clear --exec 'run --quiet --example {{file_to_run}}'

# Clean up cargo build artifacts.
[confirm]
clean:
    cargo clean

# List dependencies. (Note some dependencies needed to run this command.)
deps:
    xsv table ext_dependencies.csv
    
# Enter mysql instance "remotely" with container.
mysql port="3306":
    echo "try: 'SHOW DATABASES; USE student; SHOW TABLES; SELECT * FROM STUDENTS;'"
    mysql --host 127.0.0.1 --port {{port}} --user root --password=root

# Run the Docker compose file.
docker-comp:
    docker compose --file data/db_gen/docker-compose.yaml up --detach 
    docker image ls
    docker container ls

# Build a Docker image of a MySQL database.
docker-build image_tag="xp-sqlx-mysql-image":
    docker build --tag {{image_tag}} data/db_gen/.
    docker image ls | recolor '({{image_tag}})'
 
# Run Docker image. (Create container instance. NOT detached.)
docker-run host_port="3306" image_tag="xp-sqlx-mysql-image" container_name="xp-sqlx-mysql-container":
    docker image ls
    docker run --publish {{host_port}}:3306 --name={{container_name}} {{image_tag}}     
    docker container ls
    echo "Checking port for listening Daemon (containerized mysql server would be a positive hit)"
    nc -zv 127.0.0.1 {{host_port}}

# Clean that Docker stuff. Kills container and then force removes image. (same tag)
[confirm]
docker-destroy image_tag="xp-sqlx-mysql-image" container_name="xp-sqlx-mysql-container":
    docker image ls | recolor '({{image_tag}})'
    docker container ls | recolor '({{container_name}})'
    -docker kill {{container_name}}
    -docker container rm {{container_name}}
    -docker image rm {{image_tag}}
    docker image ls | recolor '({{image_tag}})'
    docker container ls | recolor '({{container_name}})'
