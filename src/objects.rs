use std::fs::File;
use serde::{Serialize, Deserialize};
use crate::effects::{Effect};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct TelescopeObject{
    pub unique_label: String,
    //This is a label specifying the exact instance of this effect.
    //For example, M1 verses M2
    pub effects: Vec<Effect>,
    //A vector of the effects that this object applies to the light
    pub recipients: Vec<String>,
    //A list of objects that receive the output light from this object
}

impl TelescopeObject {
    pub fn new(unique_label:&str, effects: Vec<Effect>, recipients: Vec<&str>) -> TelescopeObject{
        TelescopeObject{
            unique_label: unique_label.to_string(),
            effects,
            recipients: recipients.into_iter().map(|x| x.to_string()).collect(),
        }
    }
    pub fn add_recipient(&mut self, recipient_name:&str)  {
        self.recipients.push(recipient_name.to_string())
    }
    pub fn write_to_yaml(&self, file_name:&str,) {
        println!("Writing configuration data for {:?} to {:?}", self.unique_label, file_name);
        let serialized_self = serde_yaml::to_string(&self).expect("Failed to YAMLify the object");
        let mut file = File::create(file_name).expect("Couldn't create the config file");
        write!(file, "{}", serialized_self).expect("Failed to write YAML to config file");
    }

    //TODO remove recipient/effect/turn off/on effect etc
}






