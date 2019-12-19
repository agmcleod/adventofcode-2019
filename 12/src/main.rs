use std::cmp;

#[derive(Copy, Clone)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    fn manhatten_abs(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Copy, Clone)]
struct Planet {
    position: Vector3,
    velocity: Vector3,
}

impl Planet {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Planet {
            position: Vector3 { x, y, z },
            velocity: Vector3 { x: 0, y: 0, z: 0 },
        }
    }
}

fn get_velocity(target_axis: i32, other_axis: i32) -> i32 {
    if target_axis > other_axis {
        -1
    } else {
        1
    }
}

fn coord_sets_are_equal(coords: &[(i32, i32); 4], coords2: &[(i32, i32); 4]) -> bool {
    coords[0].0 == coords2[0].0
        && coords[1].0 == coords2[1].0
        && coords[2].0 == coords2[2].0
        && coords[3].0 == coords2[3].0
        && coords[0].1 == 0
        && coords[1].1 == 0
        && coords[2].1 == 0
        && coords[3].1 == 0
}

fn find_differential(mut coords: [(i32, i32); 4]) -> i32 {
    let coords_origin = coords.clone();
    let mut differential = 0;
    loop {
        if differential > 0 && coord_sets_are_equal(&coords, &coords_origin) {
            break;
        }

        let last_step = coords.clone();
        for index in 0..coords.len() {
            let target = coords.get_mut(index).unwrap();
            for (i, other) in last_step.iter().enumerate() {
                // only update velocity via gravity for other planets
                if i != index {
                    if other.0 != target.0 {
                        target.1 += get_velocity(target.0, other.0);
                    }
                }
            }
        }

        for coord in &mut coords {
            coord.0 += coord.1;
        }

        differential += 1;
    }

    differential
}

fn get_lcm(n1: u64, n2: u64, n3: u64) -> u64 {
    let increment_by = cmp::min(cmp::min(n1, n2), n3);
    let mut multiple = increment_by;

    loop {
        if multiple % n1 == 0 && multiple % n2 == 0 && multiple % n3 == 0 {
            break;
        }

        multiple += increment_by;
    }

    multiple
}

fn main() {
    let mut planets = vec![
        Planet::new(4, 12, 13),
        Planet::new(-9, 14, -3),
        Planet::new(-7, -1, 2),
        Planet::new(-11, 17, -1),
    ];

    for _ in 0..1000 {
        // make a copy here so when we update velocity below, it doesnt affect the rest of the planets
        let reference_planets = planets.clone();
        for index in 0..planets.len() {
            let target_planet = planets.get_mut(index).unwrap();
            for (i, other) in reference_planets.iter().enumerate() {
                // only update velocity via gravity for other planets
                if i != index {
                    if other.position.x != target_planet.position.x {
                        target_planet.velocity.x +=
                            get_velocity(target_planet.position.x, other.position.x);
                    }
                    if other.position.y != target_planet.position.y {
                        target_planet.velocity.y +=
                            get_velocity(target_planet.position.y, other.position.y);
                    }
                    if other.position.z != target_planet.position.z {
                        target_planet.velocity.z +=
                            get_velocity(target_planet.position.z, other.position.z);
                    }
                }
            }
        }

        for planet in &mut planets {
            planet.position.x += planet.velocity.x;
            planet.position.y += planet.velocity.y;
            planet.position.z += planet.velocity.z;
        }
    }

    println!(
        "part one {}",
        planets.iter().fold(0, |sum, planet| {
            sum + (planet.position.manhatten_abs() * planet.velocity.manhatten_abs())
        })
    );

    // part two
    let x_coords = [(4, 0), (-9, 0), (-7, 0), (-11, 0)];
    let y_coords = [(12, 0), (14, 0), (-1, 0), (17, 0)];
    let z_coords = [(13, 0), (-3, 0), (2, 0), (-1, 0)];

    let x_differential = find_differential(x_coords);
    let y_differential = find_differential(y_coords);
    let z_differential = find_differential(z_coords);

    println!("{} {} {}", x_differential, y_differential, z_differential);

    let lcm = get_lcm(
        x_differential as u64,
        y_differential as u64,
        z_differential as u64,
    );

    println!("part two {}", lcm);
}
