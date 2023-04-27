use ignition::IgnitionDriver;



fn main() {
    let mut ignition = IgnitionDriver::from_env();

    let re = ignition.fetch_tracks(None).unwrap();
    println!("{:#?}", re);
    
}