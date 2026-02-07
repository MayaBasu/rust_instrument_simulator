use serde::{Serialize, Deserialize};
use crate::effects::{Effect};

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
}






