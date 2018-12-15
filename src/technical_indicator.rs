use crate::util::TechnicalIndicator as UtilIndicator;
use reqwest::Url;
use serde_derive::Deserialize;
use std::collections::HashMap;

type DataType = Option<HashMap<String, HashMap<String, HashMap<String, String>>>>;

/// Struct for indicator
#[derive(Deserialize)]
pub struct Indicator {
    #[serde(rename = "Error Message")]
    error_message: Option<String>,
    #[serde(rename = "Information")]
    information: Option<String>,
    #[serde(rename = "Meta Data")]
    metadata: Option<HashMap<String, String>>,
    #[serde(flatten)]
    data: DataType,
}

impl Indicator {
    /// Return out meta data in hash form
    pub fn meta_data(&self) -> Result<HashMap<String, String>, String> {
        if let Some(meta_data) = &self.metadata {
            Ok(meta_data.to_owned())
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }

    /// Return data as a vector inside result
    pub fn data(&self) -> Result<Vec<DataCollector>, String> {
        let data = self.data.to_owned();
        if data.is_some() {
            let mut vector = Vec::new();
            for hash in data.unwrap().values() {
                for time in hash.keys() {
                    let mut data_collector = DataCollector::default();
                    data_collector.time = time.to_string();
                    let hash_values = hash.get(time).unwrap().to_owned();
                    for (key, value) in hash_values.iter() {
                        let value_f64 = value.trim().parse::<f64>().unwrap();
                        data_collector.values.insert(key.to_string(), value_f64);
                    }
                    vector.push(data_collector);
                }
            }
            Ok(vector)
        } else if let Some(error) = &self.error_message {
            Err(format!("Error Message : {}", error))
        } else {
            Err(format!(
                "Information : {}",
                self.information.clone().unwrap()
            ))
        }
    }
}

/// Struct for storing a data values
#[derive(Default)]
pub struct DataCollector {
    time: String,
    values: HashMap<String, f64>,
}

impl DataCollector {
    /// Return out a time
    pub fn time(&self) -> String {
        self.time.to_string()
    }

    /// Return values for Data
    pub fn values(&self) -> HashMap<String, f64> {
        self.values.clone()
    }
}

/// Create url for reqwest
pub(crate) fn create_url(
    function: &str,
    symbol: &str,
    interval: &str,
    apikey: &str,
    series_type: Option<&str>,
    time_period: Option<&str>,
    temporary_value: Vec<UtilIndicator>,
) -> Url {
    let mut created_link = format!(
        "https://www.alphavantage.co/query?function={}&symbol={}&interval={}&apikey={}",
        function, symbol, interval, apikey
    );
    if series_type.is_some() {
        created_link.push_str(format!("&series_type={}", series_type.unwrap()).as_str());
    }
    if time_period.is_some() {
        created_link.push_str(format!("&time_period={}", time_period.unwrap()).as_str());
    }
    for values in temporary_value {
        match values {
            UtilIndicator::Acceleration(val) => {
                created_link.push_str(format!("&Acceleration={}", val).as_str())
            }
            UtilIndicator::Fastdmatype(val) => {
                created_link.push_str(format!("&fastdmatype={}", val).as_str())
            }
            UtilIndicator::Fastdperiod(val) => {
                created_link.push_str(format!("&fastdperiod={}", val).as_str())
            }
            UtilIndicator::Fastkperiod(val) => {
                created_link.push_str(format!("&fastkperiod={}", val).as_str())
            }
            UtilIndicator::Fastlimit(val) => {
                created_link.push_str(format!("&fastlimit={}", val).as_str())
            }
            UtilIndicator::Fastmatype(val) => {
                created_link.push_str(format!("&fastmatype={}", val).as_str())
            }
            UtilIndicator::Fastperiod(val) => {
                created_link.push_str(format!("&fastperiod={}", val).as_str())
            }
            UtilIndicator::Matype(val) => {
                created_link.push_str(format!("&matype={}", val).as_str())
            }
            UtilIndicator::Maximum(val) => {
                created_link.push_str(format!("&maximum={}", val).as_str())
            }
            UtilIndicator::Nbdevdn(val) => {
                created_link.push_str(format!("&nbdevdn={}", val).as_str())
            }
            UtilIndicator::Nbdevup(val) => {
                created_link.push_str(format!("&nbdevup={}", val).as_str())
            }
            UtilIndicator::Signalmatype(val) => {
                created_link.push_str(format!("&signalmatype={}", val).as_str())
            }
            UtilIndicator::Signalperiod(val) => {
                created_link.push_str(format!("&signalperiod={}", val).as_str())
            }
            UtilIndicator::Slowdmatype(val) => {
                created_link.push_str(format!("&slowdmatype={}", val).as_str())
            }
            UtilIndicator::Slowdperiod(val) => {
                created_link.push_str(format!("&slowdperiod={}", val).as_str())
            }
            UtilIndicator::Slowkmatype(val) => {
                created_link.push_str(format!("&slowkmatype={}", val).as_str())
            }
            UtilIndicator::Slowkperiod(val) => {
                created_link.push_str(format!("&slowkperiod={}", val).as_str())
            }
            UtilIndicator::Slowlimit(val) => {
                created_link.push_str(format!("&slowlimit={}", val).as_str())
            }
            UtilIndicator::Slowmatype(val) => {
                created_link.push_str(format!("&slowmatype={}", val).as_str())
            }
            UtilIndicator::Slowperiod(val) => {
                created_link.push_str(format!("&slowperiod={}", val).as_str())
            }
            UtilIndicator::Timeperiod1(val) => {
                created_link.push_str(format!("&timeperiod1={}", val).as_str())
            }
            UtilIndicator::Timeperiod2(val) => {
                created_link.push_str(format!("&timeperiod2={}", val).as_str())
            }
            UtilIndicator::Timeperiod3(val) => {
                created_link.push_str(format!("&timeperiod3={}", val).as_str())
            }
        }
    }
    created_link.parse().unwrap()
}
