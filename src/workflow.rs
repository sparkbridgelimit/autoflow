use serde::{Deserialize, Serialize};

use crate::{flow::Flow, workflow_setting::WorkerSetting};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Workflow {
  pub flow: Flow,
  pub name: String,
  pub description: String,
  pub icon: String,
  pub version: String,
  pub setting: WorkerSetting,
}