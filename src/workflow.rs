use crate::{flow::Flow, workflow_setting::WorkerSetting};

pub struct Workflow {
  pub ext_version: String,
  pub flow: Flow,
  pub name: String,
  pub description: String,
  pub icon: String,
  pub version: String,
  pub setting: WorkerSetting,
}