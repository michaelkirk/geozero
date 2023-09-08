use flatgeobuf::FgbReader;
use geozero::error::Result;
use geozero::ProcessToJson;
use seek_bufread::BufReader;
use std::fs::File;

#[test]
fn fgb_to_geojson() -> Result<()> {
    let mut filein = BufReader::new(File::open("tests/data/countries.fgb")?);
    let mut fgb = FgbReader::open(&mut filein)?.select_bbox(8.8, 47.2, 9.5, 55.3)?;
    let json = fgb.to_json()?;
    assert_eq!(
        &json[0..215],
        r#"{
"type": "FeatureCollection",
"name": "countries",
"features": [{"type": "Feature", "properties": {"id": "DNK", "name": "Denmark"}, "geometry": {"type": "MultiPolygon", "coordinates": [[[[12.690006,55.609991],[12.0"#
    );
    Ok(())
}
