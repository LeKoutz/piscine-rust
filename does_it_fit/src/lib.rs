pub mod areas_volumes;
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

use crate::areas_volumes::{circle_area, cone_volume, cube_volume, parallelepiped_volume, rectangle_area, sphere_volume, square_area, triangle_area, triangular_pyramid_volume};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: areas_volumes::GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let container_area = (x * y) as f64;
    let shape_area = match kind {
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b)
    };
    shape_area * times as f64 <= container_area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: areas_volumes::GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let container_volume = (x * y * z) as f64;
    let shape_volume = match kind {
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(triangle_area(a, b), c)
    };
    shape_volume * times as f64 <= container_volume
}
