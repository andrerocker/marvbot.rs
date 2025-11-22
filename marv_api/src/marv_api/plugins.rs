use async_trait::async_trait;
use std::io::Error;

pub type DynamicPlugin = Box<dyn Plugin>;
pub type DynamicPluginVec = Vec<DynamicPlugin>;

#[async_trait]
pub trait Plugin: Sync + Send {
    fn name(&self) -> String;
    fn is_enabled(&self, message: &String) -> bool;
    async fn perform(&self, message: &String) -> Result<Vec<String>, Error>;
}
