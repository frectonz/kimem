use std::future::Future;
use std::time::Duration;

use crate::EyreResult;

pub async fn with_retry<T, A, Fut>(mut action: A) -> EyreResult<T>
where
    A: FnMut() -> Fut,
    Fut: Future<Output = EyreResult<T>>,
{
    const MAX_ATTEMPTS: u32 = 3;
    const BASE_DELAY: Duration = Duration::from_millis(200);

    for attempt in 1..=MAX_ATTEMPTS {
        match action().await {
            Ok(value) => return Ok(value),
            Err(e) if attempt == MAX_ATTEMPTS => return Err(e),
            Err(_) => {
                let duration = BASE_DELAY * 2u32.pow(attempt - 1);
                println!("Retrying in {}ms", duration.as_millis());
                tokio::time::sleep(duration).await
            }
        }
    }

    unreachable!("loop returns on the final attempt")
}
