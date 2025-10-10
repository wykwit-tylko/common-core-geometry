use pyo3::prelude::*;

mod primitives;
mod svg;

use primitives::*;

#[pymodule]
fn common_core_geometry(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPoint3D>()?;
    m.add_class::<PyVector3D>()?;
    m.add_class::<PySphere>()?;
    m.add_class::<PyRay>()?;
    m.add_class::<PyTriangle>()?;
    m.add_class::<PyPlane>()?;
    m.add_class::<PyAABB>()?;
    m.add_class::<PyLineSegment>()?;

    svg::register_svg_module(m)?;

    Ok(())
}
