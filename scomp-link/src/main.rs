use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{

    open::that("localhost:3000")?;
    
    Ok(())

}

