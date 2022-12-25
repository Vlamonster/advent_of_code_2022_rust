use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

struct Sensor {
    sensor: (isize, isize),
    beacon: (isize, isize),
    radius: isize,
}

fn get_sensors(input: &str) -> Vec<Sensor> {
    let regex = Regex::new(r"[-\d]+").unwrap();
    let mut sensors = Vec::new();

    for line in input.lines() {
        let mut matches = regex
            .find_iter(line)
            .map(|num| num.as_str().parse().unwrap());

        let sensor = matches.next_tuple::<(isize, isize)>().unwrap();
        let beacon = matches.next_tuple::<(isize, isize)>().unwrap();
        let radius = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

        sensors.push(Sensor {
            sensor,
            beacon,
            radius,
        });
    }

    sensors
}

pub fn p1(input: &str) -> String {
    let sensors = get_sensors(input);
    let mut ranges = Vec::new();
    let mut beacons_on_strip = HashSet::new();
    for sensor in &sensors {
        let offset = sensor.radius - (2000000 - sensor.sensor.1).abs();
        ranges.push((sensor.sensor.0 - offset, sensor.sensor.0 + offset));
        if sensor.beacon.1 == 2000000 {
            beacons_on_strip.insert(sensor.beacon.0);
        }
    }
    let mut last = isize::MIN;
    let mut total = -(beacons_on_strip.len() as isize);
    for &(start, end) in ranges.iter().sorted() {
        total += (end - last.max(start) + 1).max(0);
        last = last.max(end + 1);
    }
    total.to_string()
}

pub fn p2(input: &str) -> String {
    let sensors = get_sensors(input);
    for sensor in &sensors {
        let border_radius = sensor.radius + 1;
        for x in -border_radius..=border_radius {
            'outer: for y in [-(border_radius - x).abs(), (border_radius - x).abs()] {
                let (nx, ny) = (sensor.sensor.0 + x, sensor.sensor.1 + y);
                if !(0..=4000000).contains(&nx) || !(0..=4000000).contains(&ny) {
                    continue;
                }
                for sensor in &sensors {
                    if (nx - sensor.sensor.0).abs() + (ny - sensor.sensor.1).abs() <= sensor.radius
                    {
                        continue 'outer;
                    }
                }
                return format!("{}", 4000000 * nx + ny);
            }
        }
    }
    unreachable!();
}

#[test]
fn p1_real() {
    assert_eq!(p1(include_str!("../../inputs/d15.txt")), "5832528");
}

#[test]
fn p2_real() {
    assert_eq!(p2(include_str!("../../inputs/d15.txt")), "13360899249595");
}
