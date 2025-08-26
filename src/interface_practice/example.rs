use crate::interface_practice::{
    traits::*,
    implementations::*,
    pipeline::*,
};

pub fn example_usage() {
    let source = DatabaseSource::new(
        "postgresql://localhost:5432/mydb".to_string(),
        "users".to_string(),
    );
    
    let processor = JsonProcessor::new();
    
    let storage = FileStorage::new("/tmp/processed_data".to_string());
    
    let logger = Box::new(ConsoleLogger::new(LogLevel::Info));
    
    let mut pipeline = create_processing_pipeline(source, processor, storage, logger);
    
    match pipeline.run() {
        Ok(()) => println!("Pipeline executed successfully"),
        Err(e) => println!("Pipeline failed: {}", e),
    }
}

pub fn demonstrate_trait_flow() {
    println!("=== Data Processing Pipeline Demo ===");
    
    let source = DatabaseSource::new("mock_db".to_string(), "table".to_string());
    let processor = JsonProcessor::new();
    let mut storage = FileStorage::new("/tmp".to_string());
    let logger = ConsoleLogger::new(LogLevel::Debug);
    
    println!("1. Checking data source availability...");
    if source.is_available() {
        println!("   ✓ Source is available");
    }
    
    println!("2. Fetching data from source...");
    match source.fetch_data() {
        Ok(data) => {
            println!("   ✓ Fetched {} items", data.len());
            
            println!("3. Validating input data...");
            if processor.validate_input(&data) {
                println!("   ✓ Input validation passed");
                
                println!("4. Processing data...");
                match processor.process(data) {
                    Ok(processed) => {
                        println!("   ✓ Data processed successfully");
                        
                        println!("5. Saving to storage...");
                        match storage.save(processed) {
                            Ok(()) => println!("   ✓ Data saved successfully"),
                            Err(e) => println!("   ✗ Save failed: {}", e),
                        }
                    }
                    Err(e) => println!("   ✗ Processing failed: {}", e),
                }
            }
        }
        Err(e) => println!("   ✗ Fetch failed: {}", e),
    }
    
    logger.log(LogLevel::Info, "Pipeline demonstration completed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_creation() {
        let source = DatabaseSource::new("test".to_string(), "test".to_string());
        let processor = JsonProcessor::new();
        let storage = FileStorage::new("test".to_string());
        let logger = Box::new(ConsoleLogger::new(LogLevel::Info));
        
        let _pipeline = create_processing_pipeline(source, processor, storage, logger);
    }
}