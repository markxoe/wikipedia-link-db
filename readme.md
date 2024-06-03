# Wikipedia Link DB

Primary goal of this project: Build a program that can quickly return the shortest path between 2 Wikipedia pages via page links

## Implementation details

- Reading the Wikipedia dump files `-page.sql`, `-redirect.sql` and `-pagelinks.sql` using one file-reading thread (line-by-line) and at least one parsing thread (work is spread with a queue) which uses a regex that parses the sql insert statements
- Remapping the page-, redirect- and link-lists into hashmaps that can be (de-)serialized in cbor format
- Shortest-path search using BFS

## Results

I've used the `derive-db` command with the `dewiki-20240501` version of wikipedia on a 12-core Apple Silicon M2 Pro machine (while using 10 threads):

- Took 67 seconds
- Used about 2.9 GB RAM max (htop)
- Resulted in a 970.46 MB serialized db

Using the database using the `interactive` command

- Loading took 6 seconds
- Uses 1.9 GB RAM (htop, before querying)
- A few test queries:
  - `Seekröten` > `Linux`: 6ms (Path length: 3 (including end page, excluding start page))
  - `Briefmarke` > `Linux Torvalds`: 53ms (Path length: 3)
  - `Wasserrakete` > `Punktierter Stumpfzangenläufer`: 2152ms (Path length: 6)

## Usage

1. Build the project, ideally in release mode (use `cargo build -r` or just use `cargo run -r --` to run)

2. Download needed files from one of the Wikipedia dump mirrors [dumps.wikimedia.org/mirrors.html](https://dumps.wikimedia.org/mirrors.html):

   1. Choose a mirror
   2. Depending on the mirror go to `{language}wiki/{date of dump}`
   3. Download `-page.sql.gz`, `-pagelinks.sql.gz` and `-redirect.sql.gz`
   4. Extract `.gz` files

3. Derive Database from the downloaded files: `./target/release/wikipedia-link-db derive-db -p {file prefix}-page.sql -r {file prefix}-redirect.sql -l {file prefix}-pagelinks.sql -o output.db -t {number of threads to use}`

4. Use the `interactive` command to interactively query paths from the db: `./target/release/wikipedia-link-db interactive -d output.db`
