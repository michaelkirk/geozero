use gdal::vector::Geometry;
use gdal_sys::{self, OGRwkbGeometryType};
use geozero::error::{GeozeroError, Result};
use geozero::GeomProcessor;

/// Process GDAL/OGR geometry.
pub fn process_geom<P: GeomProcessor>(geo: &Geometry, processor: &mut P) -> Result<()> {
    process_geom_n(geo, 0, processor)
}

fn process_geom_n<P: GeomProcessor>(geo: &Geometry, idx: usize, processor: &mut P) -> Result<()> {
    match type2d(geo.geometry_type()) {
        OGRwkbGeometryType::wkbPoint => {
            processor.point_begin(idx)?;
            process_point(geo, 0, processor)?;
            processor.point_end(idx)?;
        }
        OGRwkbGeometryType::wkbMultiPoint => {
            let n_pts = geo.geometry_count();
            processor.multipoint_begin(n_pts, idx)?;
            for i in 0..n_pts {
                let pt = unsafe { geo._get_geometry(i) };
                if type2d(pt.geometry_type()) != OGRwkbGeometryType::wkbPoint {
                    return Err(GeozeroError::GeometryFormat);
                }
                process_point(&pt, i, processor)?;
            }
            processor.multipoint_end(idx)?;
        }
        OGRwkbGeometryType::wkbLineString => {
            process_linestring(geo, true, idx, processor)?;
        }
        OGRwkbGeometryType::wkbMultiLineString => {
            let n_lines = geo.geometry_count();
            processor.multilinestring_begin(n_lines, idx)?;
            for i in 0..n_lines {
                let line = unsafe { geo._get_geometry(i) };
                if type2d(line.geometry_type()) != OGRwkbGeometryType::wkbLineString {
                    return Err(GeozeroError::GeometryFormat);
                }
                process_linestring(&line, false, i, processor)?;
            }
            processor.multilinestring_end(idx)?;
        }
        OGRwkbGeometryType::wkbPolygon => {
            process_polygon(geo, true, idx, processor)?;
        }
        OGRwkbGeometryType::wkbMultiPolygon => {
            let n_polys = geo.geometry_count();
            processor.multipolygon_begin(n_polys, idx)?;
            for i in 0..n_polys {
                let poly = unsafe { geo._get_geometry(i) };
                if type2d(poly.geometry_type()) != OGRwkbGeometryType::wkbPolygon {
                    return Err(GeozeroError::GeometryFormat);
                }
                process_polygon(&poly, false, i, processor)?;
            }
            processor.multipolygon_end(idx)?;
        }
        OGRwkbGeometryType::wkbGeometryCollection => {
            let n_geoms = geo.geometry_count();
            processor.geometrycollection_begin(n_geoms, idx)?;
            for i in 0..n_geoms {
                let g = unsafe { geo._get_geometry(i) };
                process_geom_n(&g, i, processor)?;
            }
            processor.geometrycollection_end(idx)?;
        }
        _ => return Err(GeozeroError::GeometryFormat),
    }
    Ok(())
}

fn type2d(wkb_type: OGRwkbGeometryType::Type) -> OGRwkbGeometryType::Type {
    match wkb_type {
        OGRwkbGeometryType::wkbPoint | OGRwkbGeometryType::wkbPoint25D => {
            OGRwkbGeometryType::wkbPoint
        }
        OGRwkbGeometryType::wkbMultiPoint | OGRwkbGeometryType::wkbMultiPoint25D => {
            OGRwkbGeometryType::wkbMultiPoint
        }
        OGRwkbGeometryType::wkbLineString | OGRwkbGeometryType::wkbLineString25D => {
            OGRwkbGeometryType::wkbLineString
        }
        OGRwkbGeometryType::wkbMultiLineString | OGRwkbGeometryType::wkbMultiLineString25D => {
            OGRwkbGeometryType::wkbMultiLineString
        }
        OGRwkbGeometryType::wkbPolygon | OGRwkbGeometryType::wkbPolygon25D => {
            OGRwkbGeometryType::wkbPolygon
        }
        OGRwkbGeometryType::wkbMultiPolygon | OGRwkbGeometryType::wkbMultiPolygon25D => {
            OGRwkbGeometryType::wkbMultiPolygon
        }
        other => other,
    }
}

fn process_point<P: GeomProcessor>(geo: &Geometry, idx: usize, processor: &mut P) -> Result<()> {
    let multi = processor.dimensions().z;
    let (x, y, z) = geo.get_point(0);
    if multi {
        processor.coordinate(x, y, Some(z), None, None, None, idx)?;
    } else {
        processor.xy(x, y, idx)?;
    }
    Ok(())
}

fn process_linestring<P: GeomProcessor>(
    geo: &Geometry,
    tagged: bool,
    idx: usize,
    processor: &mut P,
) -> Result<()> {
    let length = unsafe { gdal_sys::OGR_G_GetPointCount(geo.c_geometry()) } as usize;
    processor.linestring_begin(tagged, length, idx)?;
    let multi = processor.dimensions().z;
    for i in 0..length {
        let (x, y, z) = geo.get_point(i as i32);
        if multi {
            processor.coordinate(x, y, Some(z), None, None, None, i)?;
        } else {
            processor.xy(x, y, i)?;
        }
    }
    processor.linestring_end(tagged, idx)
}

fn process_polygon<P: GeomProcessor>(
    geo: &Geometry,
    tagged: bool,
    idx: usize,
    processor: &mut P,
) -> Result<()> {
    let ring_count = geo.geometry_count();
    processor.polygon_begin(tagged, ring_count, idx)?;
    for i in 0..ring_count {
        let ring = unsafe { geo._get_geometry(i) };
        if type2d(ring.geometry_type()) != OGRwkbGeometryType::wkbLineString {
            return Err(GeozeroError::GeometryFormat);
        }
        process_linestring(&ring, false, i, processor)?;
    }
    processor.polygon_end(tagged, idx)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::wkt_writer::WktWriter;

    #[test]
    fn point() {
        let wkt = "POINT(1 1)";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn multipoint() {
        let wkt = "MULTIPOINT(1 1,2 2)";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn line() {
        let wkt = "LINESTRING(1 1,2 2)";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn line_3d() {
        let wkt = "LINESTRING(1 1 10,2 2 20)";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        let mut writer = WktWriter::new(&mut wkt_data);
        writer.dims.z = true;
        assert!(process_geom(&geo, &mut writer).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn multiline() {
        let wkt = "MULTILINESTRING((1 1,2 2),(3 3,4 4))";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn polygon() {
        let wkt = "POLYGON((0 0,0 3,3 3,3 0,0 0),(0.2 0.2,0.2 2,2 2,2 0.2,0.2 0.2))";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn multipolygon() {
        let wkt = "MULTIPOLYGON(((0 0,0 1,1 1,1 0,0 0)))";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }

    #[test]
    fn geometry_collection() {
        let wkt = "GEOMETRYCOLLECTION(POINT(1 1),LINESTRING(1 1,2 2))";
        let geo = Geometry::from_wkt(wkt).unwrap();

        let mut wkt_data: Vec<u8> = Vec::new();
        assert!(process_geom(&geo, &mut WktWriter::new(&mut wkt_data)).is_ok());

        assert_eq!(std::str::from_utf8(&wkt_data).unwrap(), wkt);
    }
}
