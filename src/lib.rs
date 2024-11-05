use pgrx::prelude::*;
use pgrx::pg_sys::Point;
use geohash::{encode, decode, neighbor, Direction, Coord};


::pgrx::pg_module_magic!();

#[pg_extern]
fn geohash_decode(hash_str: String) -> Point<> {
    let (c, _, _) = decode(&hash_str).unwrap();
    Point {
        x: c.x,
        y: c.y,
    }
}

#[pg_extern]
fn geohash_encode(point: Point<>) -> String {
    let c = Coord { x: point.x, y: point.y };
    // encode max supported precision is 12
    encode(c, 12).unwrap()
}

#[pg_extern]
fn geohash_encode_with_precision(point: Point<>, precision: i32) -> String {
    let c = Coord { x: point.x, y: point.y };
    encode(c, precision as usize).unwrap()
}

#[pg_extern]
fn geohash_neighbor(hash_str: String, direction: i32) -> String {
    let dir = match direction {
        0 => Direction::N,
        1 => Direction::NE,
        2 => Direction::E,
        3 => Direction::SE,
        4 => Direction::S,
        5 => Direction::SW,
        6 => Direction::W,
        7 => Direction::NW,
        _ => panic!("Invalid direction"),
    };
    neighbor(&hash_str, dir).unwrap()
}


#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use pgrx::pg_sys::Point;

    #[pg_test]
    fn test_geohash_decode_via_spi() {
        let result = Spi::get_one::<Point>("SELECT geohash_decode('ezs42')")
            .expect("SPI result should not be NULL").unwrap();
        assert!((result.x - -5.60302734375).abs() < 1e-6);
        assert!((result.y - 42.60498046875).abs() < 1e-6);
    }

    #[pg_test]
    fn test_geohash_encode_via_spi() {
        let result = Spi::get_one::<String>(
            "SELECT geohash_encode(point(-5.60302734375, 42.60498046875))"
        ).expect("SPI result should not be NULL").unwrap();
        assert!(result.starts_with(&"ezs42".to_string())); // Since we use precision 20
    }

    #[pg_test]
    fn test_geohash_encode_precision_via_spi() {
        let result = Spi::get_one::<String>(
            "SELECT geohash_encode_with_precision(point(-5.60302734375, 42.60498046875), 5)"
        ).expect("SPI result should not be NULL").unwrap();
        assert_eq!(result, "ezs42".to_string());
    }

    #[pg_test]
    fn test_geohash_neighbor_via_spi() {
        let result = Spi::get_one::<String>(
            "SELECT geohash_neighbor('ezs42', 0)"
        ).expect("SPI result should not be NULL").unwrap();
        assert_eq!(result, "ezs48".to_string());
    }

    #[pg_test]
    #[should_panic]
    fn test_geohash_invalid_direction_via_spi() {
        Spi::get_one::<String>(
            "SELECT geohash_neighbor('ezs42', 8)"
        ).expect("SPI call failed");
    }

    #[pg_test]
    #[should_panic]
    fn test_geohash_invalid_hash_via_spi() {
        Spi::get_one::<Point>(
            "SELECT geohash_decode('invalid')"
        ).expect("SPI call failed");
    }
}

/// This module is required by `cargo pgrx test` invocations.
/// It must be visible at the root of your extension crate.
#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
