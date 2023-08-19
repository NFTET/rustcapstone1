use crate::location::{Country,Continent};
use chrono::NaiveDate;
#[derive(Debug)]
pub struct Transaction{
    transaction_id:u32,
    client_id:u32,
    asset_name:String,
    country:Country,
    continent:Continent,
    amount:f64,
    days_under_management:i64

}
impl Transaction{
    pub fn from_csv_line(line:&str)-> Result<Transaction,String>{
        let field: Vec<&str>=line.split(',').collect();
        if (field.len()!=7){
            return Err("Incorrect number of fields".to_owned());
        }
        let transaction_id=field[0].parse::<u32>().unwrap();
        let client_id=field[1].parse::<u32>().unwrap();
        let asset_name=field[2].to_uppercase();
        let transaction_start_date = NaiveDate::parse_from_str(field[3],"%Y-%m-%d").unwrap();
        let transaction_end_date = NaiveDate::parse_from_str(field[4],"%Y-%m-%d").unwrap();
        let country=field[5].parse::<Country>()?;
        let amount=field[6].parse::<f64>().unwrap();

        let days_under_management=(transaction_start_date - transaction_end_date).num_days();
        let continent=country.country_to_continent();
        let transaction=Transaction{
            transaction_id:transaction_id,
            client_id:client_id,
            asset_name:asset_name,
            country:country,
            continent:continent,
            amount:amount,
            days_under_management:days_under_management
        };
        Ok(transaction)
        }
}