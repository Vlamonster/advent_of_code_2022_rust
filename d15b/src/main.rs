use regex::Regex;

struct Sensor {
    coords: (isize, isize),
    radius: isize,
}

fn main() {
    let regex = Regex::new(r"[-\d]+").unwrap();
    let mut sensors = Vec::new();

    for reading in include_str!("input.txt").lines() {
        let mut matches = regex
            .find_iter(reading)
            .map(|coordinate| coordinate.as_str().parse::<isize>().unwrap());

        let coords = (matches.next().unwrap(), matches.next().unwrap());
        let radius =
            (coords.0 - matches.next().unwrap()).abs() + (coords.1 - matches.next().unwrap()).abs();

        sensors.push(Sensor { coords, radius });
    }

    for sensor in &sensors {
        let border_radius = sensor.radius + 1;
        for x in -border_radius..=border_radius {
            'outer: for y in [-(border_radius - x).abs(), (border_radius - x).abs()] {
                let (nx, ny) = (sensor.coords.0 + x, sensor.coords.1 + y);
                if !(0..=4000000).contains(&nx) || !(0..=4000000).contains(&ny) {
                    continue;
                }
                for sensor in &sensors {
                    if (nx - sensor.coords.0).abs() + (ny - sensor.coords.1).abs() <= sensor.radius
                    {
                        continue 'outer;
                    }
                }
                print!("{}", 4000000 * nx + ny);
                return;
            }
        }
    }
}
