mod app;
mod integrations;
use std::fs::File;
use std::io::Read;

use app::reports::ReportGenerator;
use app::categories::CategoryConfig;
use integrations::transaction_data::{ TransactionJsonDataLoader };

const CATEGORY_CONFIG_PATH: &str = "./resources/test_config/test_categories_1.json";

fn main() {
    // TODO: load the actual categories from file

    // Read the category config from filepath. Panic if this fails.
    let mut file = File::open(CATEGORY_CONFIG_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse the JSON into a CategoryConfig
    let category_config: CategoryConfig = serde_json::from_str(&contents).unwrap();
    
    println!("{:?}", category_config);

    // Create a report generator loaded with the categories from the config
    // NOTE: this struct exists for the time when we are initializing once (i.e. on startup) 
    // and then potentially generating multiple reports
    let report_generator = ReportGenerator::new(category_config);

    // Load some transaction data

    // Parse transaction data into a flat list of relevant transactions for the ledgers
    let transaction_data = TransactionJsonDataLoader::new().load();

    // Generate the report
    let report = report_generator.generate(Vec::new());

    // Print the report
    // TODO: Work out how to make this integrate with Lambda properly
    println!("{:?}", report);
}


