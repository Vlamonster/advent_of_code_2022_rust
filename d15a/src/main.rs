use regex::Regex;
use std::collections::HashSet;

fn main() {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    let regex = Regex::new(r"[-\d]+").unwrap();

    for reading in include_str!("input.txt").lines() {
        let mut matches = regex
            .find_iter(reading)
            .map(|coordinate| coordinate.as_str().parse::<isize>().unwrap());
        sensors.push((matches.next().unwrap(), matches.next().unwrap()));
        beacons.push((matches.next().unwrap(), matches.next().unwrap()));
    }

    let mut checked_tiles = HashSet::new();

    for (sensor, beacon) in sensors.iter().zip(&beacons) {
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let offset = distance - (sensor.1 - 2000000).abs();
        for x in sensor.0 - offset..=sensor.0 + offset {
            checked_tiles.insert((x, 2000000));
        }
    }

    for beacon in &beacons {
        checked_tiles.remove(beacon);
    }

    print!("{:?}", checked_tiles.len());
}
