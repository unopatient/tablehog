use tablehog::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    println!("{}", unix_offset_date_time_now_local()?);


    // let client = reqwest::Client::new();

    // let csrf_token = obtain_csrf_token(&client).await?;
    // println!("csrf_token: {}", csrf_token);

    // // let restaurant_id = 118267; // Maude
    // // let experience_id = 285395; // Maude 10th Anniversary

    // let restaurant_id = 14410;  // Osteria Mozza
    // let experience_id = 208989; // Pasta Tasting Menu
    // let seating_option = "COUNTER";
    // let slot_hash = 2635132527;
    // let experience_version = 9;
    // let dining_area_id = 1;

    // let party_size = 2;
    // // let date = time::macros::date!(2024-05-06);
    // // let time_ = time::macros::time!(18:30);
    // let date_time = time::macros::datetime!(2024-05-06 20:00);
    // let forward_minutes = 180;
    // let backwards_minutes= 0;
    // let forward_days = 30;

    // let experience_availability_response = fetch_experience_availability(
    //     &client, 
    //     restaurant_id, 
    //     experience_id, 
    //     party_size, 
    //     &date_time, 
    //     forward_minutes, 
    //     backwards_minutes, 
    //     forward_days,
    // ).await?;

    // println!("experience_availability_response: \n{:#?}", experience_availability_response);

    // let experience_availability_response_json = experience_availability_response.json::<Value>().await?;

    // println!("response: {}", experience_availability_response_json);

    // let slot_lock_response = lock_book_details_experience_slot(
    //     &client, 
    //     restaurant_id, 
    //     seating_option, 
    //     &date_time, 
    //     party_size, 
    //     slot_hash, 
    //     experience_id, 
    //     experience_version, 
    //     dining_area_id
    // ).await?;

    // println!("slot_lock_response: \n{:#?}", slot_lock_response);

    // let slot_lock_response_json = slot_lock_response.json::<Value>().await?;

    // println!("response: {}", slot_lock_response_json);

    Ok(())
}
