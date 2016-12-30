pub mod mapping;

use serde_json;

use geojson::{ Geometry, Value };
use elastic_types::prelude::*;
use ::geo_shape_fixtures::*;

#[test]
fn can_change_geo_shape_mapping() {
    fn takes_custom_mapping(_: GeoShape<MyGeoShapeMapping>) -> bool {
        true
    }

    let point: GeoShape<DefaultGeoShapeMapping> = GeoShape::new(
        Geometry::new(
            Value::Point(vec![ 1.0, 1.0 ])
        )
    );

    assert!(takes_custom_mapping(point.remap()));
}

#[test]
fn serialise_elastic_geo_shape() {
    let shape = GeoShape::<DefaultGeoShapeMapping>::new(
        Geometry::new(
            Value::Point(vec![ 1.0, 1.0 ])
        )
    );

    let ser = serde_json::to_string(&shape).unwrap();

    assert_eq!(json_str!({
        "coordinates": [ 1.0, 1.0 ],
        "type": "Point"
    }), ser);
}

#[test]
fn deserialise_elastic_geo_shape() {
    let shape: GeoShape<DefaultGeoShapeMapping> = serde_json::from_str(&json_str!({
        "coordinates": [ 1, 1 ],
        "type": "Point"
    })).unwrap();

    assert_eq!(
        Geometry::new(
            Value::Point(vec![ 1.0, 1.0 ])),
        *shape
    );
}
