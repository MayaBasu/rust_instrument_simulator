use serde::{Deserialize, Serialize};
use crate::effects::EffectAction::*;
use crate::instrument::{spatial_resolution, spectral_resolution};
/*
This file sets up the type of effects used in the configuration of the instrument.
If you want any new effect which acts as point-wise multiplication, point-wise addition, or as a convolution kernel
then all you have to do is define it below.
The spatial extent tells us how many spatial entries there are.
For example, QUANTUM_EFFICIENCY has a spatial_extent of 4000 in anticipation of a 4000X4000 grid of values for each pixel
The spectral extent is the number of wavelengths at which the data is sampled, for example, 1000.
Together, these values describe a datacube of dimension (spatial_extent,spatial_extent,spectral_extent)
If there is no spatial or spectral variation, we set spectral_extent or spatial_extent to 1 respectively.
 */

pub const QUANTUM_EFFICIENCY: EffectType = EffectType {
    spatial_extent: spatial_resolution,
    spectral_extent: spectral_resolution,
    effect_action: ComponentWiseMultiplicative,
};
pub const DARK_CURRENT: EffectType = EffectType {
    spatial_extent: spatial_resolution,
    spectral_extent: 1,
    effect_action: ComponentWiseAddition,
};
pub const REFLECTANCE: EffectType = EffectType {
    spatial_extent: 1,
    spectral_extent: spectral_resolution,
    effect_action: ComponentWiseMultiplicative,
};
pub const CONTAMINATION: EffectType = EffectType {
    spatial_extent: 1,
    spectral_extent: 1,
    effect_action: ComponentWiseMultiplicative,
};
pub const SLIT: EffectType = EffectType {
    spatial_extent: spatial_resolution,
    spectral_extent: 1,
    effect_action: Reshape,
};
pub const READ_NOISE: EffectType = EffectType {
    spatial_extent: spatial_resolution,
    spectral_extent: 1,
    effect_action: ComponentWiseAddition,
};
pub const VINIETTING: EffectType = EffectType {
    spatial_extent: spatial_resolution,
    spectral_extent: 1,
    effect_action: ComponentWiseMultiplicative,
};
pub const POINT_SPREAD_FUNCTION: EffectType = EffectType {
    spatial_extent: 10,
    spectral_extent: 1,
    effect_action: ConvolutionKernel,
};



#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum EffectAction {
    //EffectAction determines how the effects modify the data involved
    ComponentWiseMultiplicative,
    //Effects like transmission curves and quantum efficiencies should be multiplied component-wise
    ComponentWiseAddition,
    //For effects such as read noise
    ConvolutionKernel,
    //For the point spread functions and spacecraft "jitter"
    Reshape,
    //For reshaping the data, for example because of the slit mask
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct EffectType {
    //All the information describing a certain effect is packaged together into an Effect
    //The first two fields tell us how to read the data from the file, and the last tells us how to apply it
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


impl EffectType {
    pub fn new(&self, effect_label: &str, data_path: &str) -> Effect{
        /*returns an instance of an Effect of type EffectType.
        So QUANTUM_EFFICIENCY.new("best_ever_qe", "data/qe")
        returns an Effect of EffectType QUANTUM_EFFICIENCY with name "best_ever_qe" and data path "data/qe"
        This defaults to an active element. You can use Effect.turn_off() to turn off the effect so it isn't used.
         */
        Effect{
            effect_label:effect_label.to_string(),
            effect_type: self.clone(),
            active: true,
            data_path: data_path.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Effect {
    //A particular instance of an effect of type EffectType, which acts upon the incoming light with an EffectAction
    //The EffectAction is specified via the Effect Type
    pub effect_label: String,
    //unique label of this instance of the effect
    pub effect_type: EffectType,
    //The type of effect which this effect is (qe, dark current, read noise ect.)
    pub active: bool,
    //"active" toggles if this effect should be applied?
    pub data_path:String,
    //path to the data which describes this particular instance of the effect
}
impl Effect{
    pub fn turn_off(&mut self){
        //turns off an effect
        self.active = false
    }
}






