mod cebdata;
mod native_date_time_range;
mod utils;

fn main() {
    let d = cebdata::fetch_data().unwrap();
    println!("{d}");
    let po = cebdata::PowerOutages::new(d);
    // println!("{:?}", po);

    println!(
        "{:#?}",
        po.get_region("moka".to_string())
            .expect("Could not get region")
            .get_poweroutages().get(0).unwrap().date()
    );
}
