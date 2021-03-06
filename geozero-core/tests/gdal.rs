#[cfg(feature = "gdal-lib")]
mod gdal_examples {

    use gdal::vector::Dataset;
    use geozero_core::gdal::process_geom;
    use geozero_core::svg::SvgWriter;
    use std::path::Path;

    #[test]
    fn ogr_to_svg() -> Result<(), gdal::errors::Error> {
        let mut dataset = Dataset::open(Path::new("tests/data/places.json"))?;
        let layer = dataset.layer(0)?;
        let mut out: Vec<u8> = Vec::new();
        for feature in layer.features() {
            let geom = feature.geometry();
            assert!(process_geom(&geom, &mut SvgWriter::new(&mut out, true)).is_ok());
        }
        assert_eq!(
            &std::str::from_utf8(&out).unwrap()[..53],
            r#"<path d="M 32.533299524864844 -0.583299105614628 Z"/>"#
        );
        Ok(())
    }
}
