use std::fs::{File, self};
use std::io::prelude::*;
use crate::CONFIG_DIR;
use crate::CONFIG_FILE;

const RON_TEMPLATE: &str = "
   Config(
      user_name: \"minh\",
      income: 0,
   )
";

pub fn create_template_config() {
   fs::create_dir(CONFIG_DIR);
   let mut file = File::create(CONFIG_FILE).expect("Can't create config file");
   file.write_all(RON_TEMPLATE.as_bytes());
}
