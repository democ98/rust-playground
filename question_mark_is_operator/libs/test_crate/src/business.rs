use crate::Result;
use anyhow::anyhow;

pub fn function_b(ok: bool) -> Result<()> {
    if !ok {
        // We can get anyhow::Error type by using anyhow! macro, Over here we have to use into() to convert it to Result type.
        return Err(anyhow!("function b error").into());
    } else {
        // But here we can use the question mark operator to convert anyhow::Error to Result type directly.
        crate::function_a()?;
    }
    Ok(())
}

pub fn function_c(ok: bool) -> Result<()> {
    if !ok {
        let error = anyhow!("function c error");
        return Err(error.into());
    } else {
        match crate::function_a() {
            Ok(_) => {}
            Err(error) => {
                return {
                    // Little strange, When we use match statement to get the error concrete object,but still need use into() to convert it to Result type for return.
                    Err(error.into())
                };
            }
        }
    }
    Ok(())
}
