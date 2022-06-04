use {
    crate::Error,
    library::package::{Metadata, Spec},
};

pub struct Status {
    pub metadata: Metadata,
    pub spec: Spec,
    pub served: bool,
}
pub fn inform(result: Result<Status, Error>, peer: &str) {
    use log::info;

    match result {
        Ok(Status {
            metadata,
            spec,
            served,
        }) => {
            if served {
                info!("served {} with {}/{}", peer, metadata, spec);
            } else {
                info!(
                    "by {} requested {}/{} is not in repository",
                    peer, metadata, spec
                );
            }
        }
        Err(e) => info!("serving {} failed because: {}", peer, e),
    }
}
