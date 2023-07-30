

struct Point3D(f32,f32,f32);






fn main() {
    let point1 = Point3D(1.0,2.0,1.0);

    let point2 = Point3D(2.0,2.0,2.0);

    let distance = calculate_distance(point1,point2);


    println!("Distance is {}",distance);
}


fn calculate_distance(
    point1: Point3D,
    point2: Point3D
) -> f32 {

    let dx = point1.0 - point2.0;
    let dy = point1.1 - point2.1;
    let dz = point1.2 - point2.2;

    (dx*dx + dy*dy + dz*dz).sqrt()
}