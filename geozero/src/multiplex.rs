use crate::error::Result;
use crate::feature_processor::FeatureProcessor;
use crate::geometry_processor::GeomProcessor;
use crate::property_processor::{ColumnValue, PropertyProcessor};

pub struct Multiplexer<P1: FeatureProcessor, P2: FeatureProcessor> {
    p1: P1,
    p2: P2,
}

impl<P1: FeatureProcessor, P2: FeatureProcessor> Multiplexer<P1, P2> {
    pub fn new(p1: P1, p2: P2) -> Multiplexer<P1, P2> {
        Multiplexer { p1, p2 }
    }
}

impl<P1: FeatureProcessor, P2: FeatureProcessor> FeatureProcessor for Multiplexer<P1, P2> {
    fn dataset_begin(&mut self, name: Option<&str>) -> Result<()> {
        self.p1.dataset_begin(name)?;
        self.p2.dataset_begin(name)
    }
    fn dataset_end(&mut self) -> Result<()> {
        self.p1.dataset_end()?;
        self.p2.dataset_end()
    }
    fn feature_begin(&mut self, idx: u64) -> Result<()> {
        self.p1.feature_begin(idx)?;
        self.p2.feature_begin(idx)
    }
    fn feature_end(&mut self, idx: u64) -> Result<()> {
        self.p1.feature_end(idx)?;
        self.p2.feature_end(idx)
    }
    fn properties_begin(&mut self) -> Result<()> {
        self.p1.properties_begin()?;
        self.p2.properties_begin()
    }
    fn properties_end(&mut self) -> Result<()> {
        self.p1.properties_end()?;
        self.p2.properties_end()
    }
    fn geometry_begin(&mut self) -> Result<()> {
        self.p1.geometry_begin()?;
        self.p2.geometry_begin()
    }
    fn geometry_end(&mut self) -> Result<()> {
        self.p1.geometry_end()?;
        self.p2.geometry_end()
    }
}

impl<P1: FeatureProcessor, P2: FeatureProcessor> GeomProcessor for Multiplexer<P1, P2> {
    fn xy(&mut self, x: f64, y: f64, idx: usize) -> Result<()> {
        self.p1.xy(x, y, idx)?;
        self.p2.xy(x, y, idx)
    }
    fn coordinate(
        &mut self,
        x: f64,
        y: f64,
        z: Option<f64>,
        m: Option<f64>,
        t: Option<f64>,
        tm: Option<u64>,
        idx: usize,
    ) -> Result<()> {
        self.p1.coordinate(x, y, z, m, t, tm, idx)?;
        self.p2.coordinate(x, y, z, m, t, tm, idx)
    }
    fn point_begin(&mut self, idx: usize) -> Result<()> {
        self.p1.point_begin(idx)?;
        self.p2.point_begin(idx)
    }
    fn point_end(&mut self, idx: usize) -> Result<()> {
        self.p1.point_end(idx)?;
        self.p2.point_end(idx)
    }
    fn multipoint_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.multipoint_begin(size, idx)?;
        self.p2.multipoint_begin(size, idx)
    }
    fn multipoint_end(&mut self, idx: usize) -> Result<()> {
        self.p1.multipoint_end(idx)?;
        self.p2.multipoint_end(idx)
    }
    fn linestring_begin(&mut self, tagged: bool, size: usize, idx: usize) -> Result<()> {
        self.p1.linestring_begin(tagged, size, idx)?;
        self.p2.linestring_begin(tagged, size, idx)
    }
    fn linestring_end(&mut self, tagged: bool, idx: usize) -> Result<()> {
        self.p1.linestring_end(tagged, idx)?;
        self.p2.linestring_end(tagged, idx)
    }
    fn multilinestring_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.multilinestring_begin(size, idx)?;
        self.p2.multilinestring_begin(size, idx)
    }
    fn multilinestring_end(&mut self, idx: usize) -> Result<()> {
        self.p1.multilinestring_end(idx)?;
        self.p2.multilinestring_end(idx)
    }
    fn polygon_begin(&mut self, tagged: bool, size: usize, idx: usize) -> Result<()> {
        self.p1.polygon_begin(tagged, size, idx)?;
        self.p2.polygon_begin(tagged, size, idx)
    }
    fn polygon_end(&mut self, tagged: bool, idx: usize) -> Result<()> {
        self.p1.polygon_end(tagged, idx)?;
        self.p2.polygon_end(tagged, idx)
    }
    fn multipolygon_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.multipolygon_begin(size, idx)?;
        self.p2.multipolygon_begin(size, idx)
    }
    fn multipolygon_end(&mut self, idx: usize) -> Result<()> {
        self.p1.multipolygon_end(idx)?;
        self.p2.multipolygon_end(idx)
    }
    fn circularstring_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.circularstring_begin(size, idx)?;
        self.p2.circularstring_begin(size, idx)
    }
    fn circularstring_end(&mut self, idx: usize) -> Result<()> {
        self.p1.circularstring_end(idx)?;
        self.p2.circularstring_end(idx)
    }
    fn compoundcurve_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.compoundcurve_begin(size, idx)?;
        self.p2.compoundcurve_begin(size, idx)
    }
    fn compoundcurve_end(&mut self, idx: usize) -> Result<()> {
        self.p1.compoundcurve_end(idx)?;
        self.p2.compoundcurve_end(idx)
    }
    fn curvepolygon_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.curvepolygon_begin(size, idx)?;
        self.p2.curvepolygon_begin(size, idx)
    }
    fn curvepolygon_end(&mut self, idx: usize) -> Result<()> {
        self.p1.curvepolygon_end(idx)?;
        self.p2.curvepolygon_end(idx)
    }
    fn multicurve_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.multicurve_begin(size, idx)?;
        self.p2.multicurve_begin(size, idx)
    }
    fn multicurve_end(&mut self, idx: usize) -> Result<()> {
        self.p1.multicurve_end(idx)?;
        self.p2.multicurve_end(idx)
    }
    fn multisurface_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.multisurface_begin(size, idx)?;
        self.p2.multisurface_begin(size, idx)
    }
    fn multisurface_end(&mut self, idx: usize) -> Result<()> {
        self.p1.multisurface_end(idx)?;
        self.p2.multisurface_end(idx)
    }
    fn triangle_begin(&mut self, tagged: bool, size: usize, idx: usize) -> Result<()> {
        self.p1.triangle_begin(tagged, size, idx)?;
        self.p2.triangle_begin(tagged, size, idx)
    }
    fn triangle_end(&mut self, tagged: bool, idx: usize) -> Result<()> {
        self.p1.triangle_end(tagged, idx)?;
        self.p2.triangle_end(tagged, idx)
    }
    fn polyhedralsurface_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.polyhedralsurface_begin(size, idx)?;
        self.p2.polyhedralsurface_begin(size, idx)
    }
    fn polyhedralsurface_end(&mut self, idx: usize) -> Result<()> {
        self.p1.polyhedralsurface_end(idx)?;
        self.p2.polyhedralsurface_end(idx)
    }
    fn tin_begin(&mut self, size: usize, idx: usize) -> Result<()> {
        self.p1.tin_begin(size, idx)?;
        self.p2.tin_begin(size, idx)
    }
    fn tin_end(&mut self, idx: usize) -> Result<()> {
        self.p1.tin_end(idx)?;
        self.p2.tin_end(idx)
    }
}

impl<P1: FeatureProcessor, P2: FeatureProcessor> PropertyProcessor for Multiplexer<P1, P2> {
    fn property(&mut self, i: usize, colname: &str, colval: &ColumnValue) -> Result<bool> {
        self.p1
            .property(i, colname, colval)
            .and(self.p2.property(i, colname, colval))
    }
}
