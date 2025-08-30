use ureq;
use serde_json::Value;

fn main() {
    let url = "https://api.flightradar24.com/common/v1/airport.json?code=icn&plugin[]=&plugin-setting[schedule][mode]=&plugin-setting[schedule][timestamp]=1756564415&page=1&limit=5&fleet=&token=";
    
    let response = ureq::get(url)
        .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0")
        .call();
    
    match response {
        Ok(resp) => {
            match resp.into_json::<Value>() {
                Ok(json) => {
                    println!("Full API Response:");
                    if let Ok(pretty) = serde_json::to_string_pretty(&json) {
                        // Print first 3000 characters to see structure
                        println!("{}", &pretty[..std::cmp::min(3000, pretty.len())]);
                    }
                    
                    // Try to navigate to the departure data
                    if let Some(result) = json.get("result") {
                        println!("\nFound 'result' field");
                        if let Some(response) = result.get("response") {
                            println!("Found 'response' field");
                            if let Some(airport) = response.get("airport") {
                                println!("Found 'airport' field");
                                if let Some(plugin_data) = airport.get("pluginData") {
                                    println!("Found 'pluginData' field");
                                    if let Some(schedule) = plugin_data.get("schedule") {
                                        println!("Found 'schedule' field");
                                        if let Some(departures) = schedule.get("departures") {
                                            println!("Found 'departures' field");
                                            if let Some(data) = departures.get("data") {
                                                println!("Found 'data' field with {} items", data.as_array().map_or(0, |a| a.len()));
                                                if let Some(arr) = data.as_array() {
                                                    if let Some(first_item) = arr.first() {
                                                        println!("\nFirst flight item structure:");
                                                        if let Ok(pretty) = serde_json::to_string_pretty(first_item) {
                                                            println!("{}", &pretty[..std::cmp::min(1000, pretty.len())]);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("JSON parse error: {}", e),
            }
        }
        Err(e) => println!("HTTP error: {}", e),
    }
}
