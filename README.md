# pg_geohash

Geohashing functionality for PostgreSQL, Greenplum and Cloudberry

## Usage

```sql
-- First create the extension
CREATE EXTENSION pg_geohash;

-- Encode latitude/longitude to geohash (default precision is 12)
SELECT geohash_encode(point(-5.60302734375, 42.60498046875));
-- Returns: ezs42...

-- Encode with specific precision
SELECT geohash_encode_with_precision(point(-5.60302734375, 42.60498046875), 5);
-- Returns: ezs42

-- Decode geohash to lat/lon point
SELECT geohash_decode('ezs42');
-- Returns: (-5.60302734375,42.60498046875)

-- Find neighboring geohash
-- direction: 0=North, 1=NorthEast, 2=East, 3=SouthEast, 4=South, 5=SouthWest, 6=West, 7=NorthWest
SELECT geohash_neighbor('ezs42', 0);  -- Northern neighbor
-- Returns: ezs48

SELECT geohash_neighbor('ezs42', 2);  -- Eastern neighbor
-- Returns: ezs43
```

## Installation

Assuming that rust toolchain is already istalled:

```sh
# install pgrx
cargo install --locked cargo-pgrx
cargo pgrx init
# build and install pg_geohash
git clone https://github.com/yihong0618/pg_geohash.git
cd pg_geohash
cargo pgrx run

## source pg_config then
cargo pgrx install 
```

## Kudos

- https://github.com/georust/geohash
- https://github.com/pgcentralfoundation/pgrx
- https://github.com/sunng87/node-geohash