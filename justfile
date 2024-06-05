# Justfile (Convenience Command Runner)


# local vars
IMAGE_AND_TAG := 'mysql:8.4.0'
CONT := 'xp-sqlx-mysql-container'
HOST_PORT := '3306'

# rust vars
RUST_LOG:= 'debug'
RUST_BACKTRACE:= '1'
RUSTFLAGS:='--cfg tokio_unstable'

# home_dir := env_var('HOME')
local_root := justfile_directory()
invocd_from := invocation_directory()
invoc_is_root := if invocd_from == local_root { "true" } else { "false" }
froze_sha_regex := 'FROZE_[a-fA-F0-9]{64}_FROZE-'
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
    -cargo clippy
    -cargo fmt
    -typos --exclude 'data/*'
    -committed
    -cargo sqlx prepare --check

# Watch a file: compile & run on changes.
watch file_to_run:
    cargo watch --quiet --clear --exec 'run --quiet --example {{file_to_run}}'

# Initialize repository.
init: && deps  docker-comp  gen-env
    cargo build
    cargo doc

# Clean up cargo build artifacts.
[confirm]
clean:
    cargo clean

# List dependencies. (This command has dependencies.)
deps:
    @echo "{{CYN}}List of external dependencies for this command runner:"
    xsv table ext_dependencies.csv

hyperf reps:
    @echo "{{PRP}}NOTE{{NC}}: we only care about 'recopy' and 'vstruct'; 'direct' does not do DF creation, 'all' was quick substitution bench framework"
    @echo "Release:"
    hyperfine --warmup 3 'target/release/transpose_implementations recopy {{reps}}'
    hyperfine --warmup 3 'target/release/transpose_implementations v-struct {{reps}}'
    @echo "Debug (for compiler insights, mostly):"
    hyperfine --warmup 3 'target/debug/transpose_implementations recopy {{reps}}'
    hyperfine --warmup 3 'target/debug/transpose_implementations v-struct {{reps}}'

# generate a `.env` file to place your database path in.
gen-env:
    @echo "{{CYN}}The {{GRN}}.env DATABASE_URL value{{CYN}}will populate your database path when needed.  Please edit the file to manually specify."
    @echo {{ if path_exists(".env") == "true" { `echo "\(.env file already exists\)"` } else { `cp 'template.env' '.env'; echo "\(.env file created\)"`} }}


# Pull local schema to dir to allow off-line building.
sqlx-prep:
    cargo install sqlx-cli
    @echo "Database path required to pull schema."
    cargo sqlx prepare -- --all-targets --all-features

# Enter MySQL instance "remotely" with container.
mysql:
    @echo "try: 'SHOW DATABASES; USE university; SHOW TABLES; SELECT * FROM STUDENTS;'"
    mysql --host 127.0.0.1 --port {{HOST_PORT}} --user root --password=root

# Run the Docker compose file.
docker-comp:
    docker compose --file data/db_gen/docker-compose.yaml up --detach
    docker image ls
    docker container ls

# Build a Docker image of a MySQL database.
docker-build:
    docker build --tag {{IMAGE_AND_TAG}} data/db_gen/.
    docker image ls | recolor '({{IMAGE_AND_TAG}})'

# Run Docker image.
docker-run:
    docker image ls
    docker run --publish {{HOST_PORT}}:3306 --name={{CONT}} {{IMAGE_AND_TAG}}
    docker container ls
    echo "Checking port for listening Daemon (containerized mysql server would be a positive hit)"
    nc -zv 127.0.0.1 {{HOST_PORT}}

# Remove Docker image & container.
[confirm]
docker-destroy:
    docker image ls | recolor '({{IMAGE_AND_TAG}})'
    @echo "{{GRN}}^^--------- images pre destroy ---------^^"
    docker container ls | recolor '({{CONT}})'
    @echo "{{GRN}}^^--------- containers pre destroy ---------^^"
    -docker kill {{CONT}}
    -docker container rm {{CONT}}
    -docker image rm {{IMAGE_AND_TAG}}
    @echo "{{PRP}}vv--------- images post destroy ---------vv"
    docker image ls | recolor '({{IMAGE_AND_TAG}})'
    @echo "{{PRP}}vv--------- containers post destroy ---------vv"
    docker container ls | recolor '({{CONT}})'

# ######################################################################## #

# Freeze! For your safety.
freeze FILE:
	mv -iv {{FILE}} FROZE_{{sha256(FILE)}}_FROZE-{{FILE}} | rg {{FILE}}

# Unfreeze a file. (removes 'FROZE...FROZE-' tag from filename)
thaw FILE:
	echo {{FILE}} | sd '{{froze_sha_regex}}' '' | xargs mv -iv {{FILE}}

# Find local file(s) through the ice.
arctic_recon ICELESS_NAME:
	fd --max-depth 1 '{{froze_sha_regex}}{{ICELESS_NAME}}' | rg {{ICELESS_NAME}}


# ######################################################################## #

# Speak Funny to Me!
_uu:
	echo {{uuid()}}

# Say my name.
_sha FILE:
	echo {{sha256_file(FILE)}}

# Example function for syntax reference
_example_file_exists_test file:
    echo {{ if path_exists(file) == "true" { "hello" } else { "goodbye" } }}
