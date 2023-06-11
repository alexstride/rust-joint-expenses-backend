mod app;
use std::fs::File;
use std::io::Read;

use app::report_generator::ReportGenerator;
use app::core_types::CategoryConfig;
use app::categories::create_ledger_categories;

const CATEGORY_CONFIG_PATH: &str = "./resources/test_config/test_categories_1.json";

fn main() {
    // TODO: load the actual categories from file

    // Read the category config from filepath. Panic if this fails.
    let mut file = File::open(CATEGORY_CONFIG_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let category_config: CategoryConfig = serde_json::from_str(&contents).unwrap();
    
    println!("{:?}", category_config);

    // If we need the category config again, this function may need to take a pointer
    // and borrow the config, rather than taking ownership and dropping it.
    let ledger_categories = create_ledger_categories(category_config);

    

    // let report_generator = ReportGenerator::new(categories);
    // println!("{:?}", report_generator.generate([]));

    // if let Ok(file_lister) = std::fs::read_dir(".") {
    //     file_lister.for_each(|entry| {
    //         println!("{:?}", entry.unwrap().path())
    //     })
    // }
}


