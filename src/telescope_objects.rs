use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Instrument{
    pub instrument_label: String,
    //label incase you want to configure several instruments and then compare them
    pub entry_point: String,
    //the 'unique_label' of the first object to receive starlight
    pub telescope_objects: Vec<TelescopeObject>,
    //A vector, in no particular order, of all the objects in the instrument
    //Objects include detectors, the dichroic (each path is a separate object), each mirror, diffraction grating etc.
    pub measurement_points: Vec<String>,
    //a list of the 'unique_label's which correspond to places where you want to save the data.
    //The data will be saved after the effects of that TelescopeObject are applied
    //If you just want the output at the detectors, then this would be the labels of the detectors.
    //However, if you want to see the intermediate evolution of the data, you can put in any object you want.
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TelescopeObject{
    pub object_name: TelescopeObjectName,
    //This gives us (the humans) an idea of the type of object being described
    //This does not affect the code, only the entries below do.
    pub unique_label: String,
    //This is a label specifying the exact instance of this effect.
    //For example, M1 verses M2
    pub effects: Vec<Effect>,
    //A vector of the effects that this object applies to the light
    pub recipients: Vec<String>,
    //A list of objects that receive the output light from this object
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneralEffect{
    pub effect_name: EffectName,
    //This tells us what type of effect
    // (QE, Read noise etc. that this struct contains)
    pub unique_label: String,
    //This is a label specifying the exact instance of this effect.
    // For example, QE specifically *for the NUV detector*
    pub active:bool,
    //Do we currently want to use this effect?
    pub file_path: String,
    //What is the path to the file which contains the data relevant to this effect?
    pub spatial_extent: usize,
    //Spatial dimension of the data.
    //If this is a QE for each pixel for one 4kx4k detector, then the spatial extent is 4000
    //If this is a spatially independent transmission curve
    // which is sampled over 1000 frequency samples then the spatial extent is 1
    pub spectral_extent: usize,
    //Spectral dimension of the data.
    //If this is spectrally independent vinietting, then the spectral extent is 1
    //If this is a spatially independent transmission curve
    // which is sampled over 1000 frequency samples then the spectral extent is 1000
    pub effect_action: EffectAction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Effect{
    pub effect_name: EffectName,
    //This tells us what type of effect
    // (QE, Read noise etc. that this struct contains)
    pub unique_label: String,
    //This is a label specifying the exact instance of this effect.
    // For example, QE specifically *for the NUV detector*
    pub active:bool,
    //Do we currently want to use this effect?
    pub file_path: String,
    //What is the path to the file which contains the data relevant to this effect?
}


#[derive(Serialize, Deserialize, Debug)]
pub enum EffectName{
    //These are human-readable names describing what effects are which.
    // !!Important!! They do not affect the code directly and are only for human understanding.
    // However, there will be a warning if you try to multiply by read noise for example.
    QuantumEfficiency,
    DarkCurrent,
    Reflectance,
    Contamination,
    Slit,
    ReadNoise,
    Vinietting,
    PointSpreadFunction,
}






#[derive(Serialize, Deserialize, Debug)]
pub enum TelescopeObjectName{
    //These are human-readable names describing components of the telescope
    // !!Important!! They do not affect the code directly and are only for human understanding.
    // However, there will be a warning if you try to add a slit to a mirror etc.
    Mirror,
    Slit,
    DispersionGrating,
    Detector,
    Dichroic,
}

