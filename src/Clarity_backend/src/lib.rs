use candid::{CandidType, Deserialize};
use ic_cdk_macros::*;
mod classification;
use classification::
Classification;
#[derive(CandidType, Deserialize)]
struct AirQualityData {    
    pm25: f64,    
    pm10: f64,
}

#[derive(CandidType, Deserialize)]
struct VerificationResult {    
    status: String,    
    message: String,
}

#[init]
fn init() {    
    ic_cdk::println!("Air Quality Classification Canister initialized");
}
    
#[update]
fn verify_air_quality(data: AirQualityData) -> VerificationResult {    
    let classifier = Classification::new();    
    let status = classifier.classify(data.pm25, data.pm10);        
    VerificationResult {        
        status,        
        message: format!("PM2.5: {}, PM10: {}", data.pm25, data.pm10)    
    }
}

#[query]
fn get_version() -> String {    
    "Air Quality Classifier v1.0".to_string()
}
#[query]
fn get_thresholds() -> String {
    "PM2.5: Good ≤ 50, Moderate ≤ 100, Poor > 100\nPM10: Good ≤ 100, Moderate ≤ 150, Poor > 150".to_string()
}
