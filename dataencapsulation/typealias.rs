type Kilometers = u32;
type Meters = u32;

fn distance_in_meters(kilometers: Kilometers) -> Meters {
    kilometers * 1000
}

fn main() {
    let dist_km: Kilometers = 5;
    let dist_m: Meters = distance_in_meters(dist_km);

    println!("Distance in meters: {}", dist_m);
}