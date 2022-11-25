use std::env;
use std::fs::File;
use std::fs;
use std::io;
use std::io::BufReader;
use std::path::Path;
use std::error::Error;
use serde::{Serialize, Deserialize};

use chrono::prelude::*;
use chrono::Duration;


#[derive(Serialize, Deserialize, Debug)]
struct MPSConfig {
    mps_api_base_uri: String,
    mps_coempr: String,
    mps_cuenta: String,
    mps_access_token: String,
    mp_url_base: String,
    mp_user_id: String,
    mp_external_pos_id: String,
    mp_external_store_id: String
}

#[derive(Serialize, Deserialize, Debug)]
struct OrdenCrear {
    external_reference: String,
    total_amount: String,
    cash_out: String
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "external_reference")]
    pub external_reference: String,
    #[serde(rename = "notification_url")]
    pub notification_url: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "expiration_date")]
    pub expiration_date: String,
    #[serde(rename = "total_amount")]
    pub total_amount: f32,
    pub items: Vec<Item>,
    #[serde(rename = "cash_out")]
    pub cash_out: CashOut,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "sku_number")]
    pub sku_number: String,
    pub category: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "unit_price")]
    pub unit_price: f32,
    pub quantity: i8,
    #[serde(rename = "unit_measure")]
    pub unit_measure: String,
    #[serde(rename = "total_amount")]
    pub total_amount: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CashOut {
    pub amount: f32,
}


fn read_mpsconfig_from_file<P: AsRef<Path>>(path: P) -> Result<MPSConfig, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `MPSConfig`.
    let u = serde_json::from_reader(reader)?;

    // Return the `MPSConfig`.
    Ok(u)
}


fn read_ordencrear_from_file<P: AsRef<Path>>(path: P) -> Result<OrdenCrear, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `OrdenCrear`.
    let u = serde_json::from_reader(reader)?;

    // Return the `OrdenCrear`.
    Ok(u)
}


async fn ordenstatuser(_er: &str) -> Result<(), Box<dyn std::error::Error>> {

    if Path::new("OrdenGetStatus.json").exists(){
        fs::remove_file("OrdenGetStatus.json").expect("File delete failed");
    }

    let _config = read_mpsconfig_from_file("mpsconfig.json").unwrap();
    let _mps_api_base_uri: &str = &_config.mps_api_base_uri[..];
    let _coempr: &str = &_config.mps_coempr[..];
    let _cuenta: &str = &_config.mps_cuenta[..];
    let _mps_access_token: &str = &_config.mps_access_token[..];

    let mut http_url = String::with_capacity(250);
    http_url.push_str(_mps_api_base_uri);
    http_url.push_str("boser?&coempr=");
    http_url.push_str(_coempr);
    http_url.push_str("&cuenta=");
    http_url.push_str(_cuenta);
    http_url.push_str("&er=");
    http_url.push_str(_er);
   let client = reqwest::Client::builder().build()?;

    let res = client
        .get(http_url)
        .header("Authorization", format!("{}{}", "Bearer ", _mps_access_token))
        .send()
        .await?        
        .bytes()
        .await?;


    let mut data = res.as_ref();
    let mut f = File::create("OrdenGetStatus.json")?;
    io::copy(&mut data, &mut f)?;
    Ok(())
}

async fn ordeneliminar() -> Result<(), Box<dyn std::error::Error>>{
    let _config = read_mpsconfig_from_file("mpsconfig.json").unwrap();

    let _mp_url_base: &str = &_config.mp_url_base[..];
    let _mp_user_id: &str = &_config.mp_user_id[..];
    let _mp_external_pos_id: &str = &_config.mp_external_pos_id[..];
    let _mps_access_token: &str = &_config.mps_access_token[..];

    let mut http_url = String::with_capacity(250);
    http_url.push_str(_mp_url_base);
    http_url.push_str("/instore/qr/seller/collectors/");
    http_url.push_str(_mp_user_id);
    http_url.push_str("/pos/");
    http_url.push_str(_mp_external_pos_id);
    http_url.push_str("/orders");

    use reqwest::Client;
    let _response  = Client::new()
        .delete(http_url)
        .header("Accept", "application/json")
        .header("Authorization", format!("{}{}", "Bearer ", _mps_access_token))
        .send()
        .await?
        .text()
        .await?;

        // println!("{:#?}", _response);

    Ok(())


}

async fn ordencrear() -> Result<(), Box<dyn std::error::Error>>{

    let _oc = read_ordencrear_from_file("ordencrear.json").unwrap();

    let _external_reference: &str = &_oc.external_reference[..];
    let _total_amount: &str = &_oc.total_amount[..];
    let _cash_out: &str = &_oc.cash_out[..];

    let _ca: f32 = _cash_out.parse::<f32>().unwrap();
    let _ta: f32 = _total_amount.parse::<f32>().unwrap();

    let _config = read_mpsconfig_from_file("mpsconfig.json").unwrap();
    let _coempr: &str = &_config.mps_coempr[..];
    let _cuenta: &str = &_config.mps_cuenta[..];
    let _mps_api_base_uri: &str = &_config.mps_api_base_uri[..];
    let _mp_url_base: &str = &_config.mp_url_base[..];
    let _mp_external_store_id: &str = &_config.mp_external_store_id[..];
    let _mp_external_pos_id: &str = &_config.mp_external_pos_id[..];
    let _mp_user_id: &str = &_config.mp_user_id[..];
    let _mps_access_token: &str = &_config.mps_access_token[..];

    let mut notification_url = String::with_capacity(250);
    notification_url.push_str(_mps_api_base_uri);
    notification_url.push_str("ipn?coempr=");
    notification_url.push_str(_coempr);
    notification_url.push_str("&cuenta=");
    notification_url.push_str(_cuenta);
    notification_url.push_str("&docid=");
    notification_url.push_str(_external_reference);


    let cash_out = CashOut{amount : _ca};

    let item = Item {
        sku_number : "SKU123456".to_string(),
        category : "FOOD".to_string(),
        title : "Item1".to_string(),
        description : "Item1 Mercado Pago".to_string(),
        unit_price : _ta,
        quantity : 1,
        unit_measure :	"unit".to_string(),
        total_amount :	_ta
    };

    let mut vec: Vec<Item> = Vec::with_capacity(1);
    vec.push(item);

    let exp_date: DateTime<Local> = Local::now() + Duration::hours(1);
    
    let _orden = Root{
        external_reference : _external_reference.to_string(),
        notification_url : notification_url.to_string(),
        title : "Orden Crear".to_string(),
        description : "Orden Crear".to_string(),
        expiration_date : exp_date.format("%Y-%m-%dT%H:%M:%S.000-04:00").to_string(),
        total_amount : _ta + _ca,
        items : vec,
        cash_out : cash_out
    };

    // let json = serde_json::to_string_pretty(&_orden).unwrap();

    std::fs::write("oc.json",serde_json::to_string_pretty(&_orden).unwrap(),).unwrap();

    let mut http_url = String::with_capacity(250);
    http_url.push_str(_mp_url_base);
    http_url.push_str("/instore/qr/seller/collectors/");
    http_url.push_str(_mp_user_id);
    http_url.push_str("/stores/");
    http_url.push_str(_mp_external_store_id);
    http_url.push_str("/pos/");
    http_url.push_str(_mp_external_pos_id);
    http_url.push_str("/orders");

    // println!("{}", json);
    
    // println!("{}", http_url);


    use reqwest::Client;
    let _response  = Client::new()
        .put(http_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("{}{}", "Bearer ", _mps_access_token))
        .body(serde_json::to_string_pretty(&_orden).unwrap())
        .send()
        .await?
        .text()
        .await?;



    std::fs::write("response.json",_response,).unwrap();

    // println!("{:#?}", _response);

    Ok(())
}

async fn ordenbuscar(_filto: &str) -> Result<(), Box<dyn std::error::Error>>{

    let _config = read_mpsconfig_from_file("mpsconfig.json").unwrap();
    let _mp_url_base: &str = &_config.mp_url_base[..];
    let _mps_access_token: &str = &_config.mps_access_token[..];

    let mut http_url = String::with_capacity(250);
    http_url.push_str(_mp_url_base);
    if _filto.len()==0{
        http_url.push_str("/merchant_orders/search");
    }
    else{
        http_url.push_str("/merchant_orders/search/?");
        http_url.push_str(_filto);
    }

    // println!("{:#?}", http_url);

    use reqwest::Client;
    let _response = Client::new()
        .get(http_url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("{}{}", "Bearer ", _mps_access_token))
        .send()
        .await?
        .text()
        .await?;


    std::fs::write("ordenbuscar.json",&_response,).unwrap();

    // println!("{:#?}", _response);




    Ok(())

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    
    let mut _p1: &str = "";
    let mut _p2: &str = "";

    match args.len() {
         2 => {
            _p1 = &args[1];
         }
         3 => {
            _p1 = &args[1];
            _p2 = &args[2];
         }
         _=> todo!()
    } 



    if _p1=="ORDENSTATUSER"{
        let _state = ordenstatuser(_p2).await;
    }

    if _p1=="ORDENELIMINAR"{
        let _state = ordeneliminar().await;
    }

    if _p1=="ORDENCREAR"{
        let _state = ordencrear().await;
    }
    
    if _p1=="ORDENBUSCAR"{
        let _state = ordenbuscar(_p2).await;
    }
    
    
    Ok(())
}
