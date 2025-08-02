pub mod areas_volumes;
pub use areas_volumes::*;

#[inline]
pub fn area_fit(
    (container_x, container_y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (shape_a, shape_b): (usize, usize),
) -> bool {
    let container_area = (container_x * container_y) as f64;
    let total_shapes_area = match kind {
        GeometricalShapes::Square => {
            areas_volumes::square_area(shape_a) as f64 * times as f64
        }
        GeometricalShapes::Circle => {
            areas_volumes::circle_area(shape_a) * times as f64
        }
        GeometricalShapes::Rectangle => {
            areas_volumes::rectangle_area(shape_a, shape_b) as f64 * times as f64
        }
        GeometricalShapes::Triangle => {
            areas_volumes::triangle_area(shape_a, shape_b) * times as f64
        }
    };
    total_shapes_area <= container_area
}

#[inline]
pub fn volume_fit(
    (container_x, container_y, container_z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (shape_a, shape_b, shape_c): (usize, usize, usize),
) -> bool {
    let container_volume = (container_x * container_y * container_z) as f64;
    let total_shapes_volume = match kind {
        GeometricalVolumes::Cube => {
            areas_volumes::cube_volume(shape_a) as f64 * times as f64
        }
        GeometricalVolumes::Sphere => {
            areas_volumes::sphere_volume(shape_a) * times as f64
        }
        GeometricalVolumes::Cone => {
            areas_volumes::cone_volume(shape_a, shape_b) * times as f64
        }
        GeometricalVolumes::TriangularPyramid => {
            let base_area = areas_volumes::triangle_area(shape_a, shape_b);
            areas_volumes::triangular_pyramid_volume(base_area, shape_c) * times as f64
        }
        GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(shape_a, shape_b, shape_c) as f64
                * times as f64
        }
    };
    total_shapes_volume <= container_volume
}
