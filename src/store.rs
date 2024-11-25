use std::fs;
use std::path::Path;
use std::error::Error;

// Inside the Store implementation
impl Store {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        // Create storage directory if it doesn't exist
        fs::create_dir_all("storage")?;
        
        let store_path = Path::new("storage").join("store.json");
        
        // Rest of initialization code using store_path...
        if store_path.exists() {
            let contents = fs::read_to_string(&store_path)?;
            Ok(serde_json::from_str(&contents)?)
        } else {
            let store = Store::default();
            store.save()?;
            Ok(store)
        }
    }

    fn save(&self) -> Result<(), Box<dyn Error>> {
        let store_path = Path::new("storage").join("store.json");
        let contents = serde_json::to_string_pretty(self)?;
        fs::write(store_path, contents)?;
        Ok(())
    }
} 