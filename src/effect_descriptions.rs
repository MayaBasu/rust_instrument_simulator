use serde::{Deserialize, Serialize};
use crate::effect_descriptions::EffectAction::*;
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
pub const QUANTUM_EFFICIENCY: Effect = Effect{
    spatial_extent: 4000,
    spectral_extent: 1000,
    effect_action: ComponentWiseMultiplicative,
};
pub const DARK_CURRENT: Effect = Effect{
    spatial_extent: 4000,
    spectral_extent: 1,
    effect_action: ComponentWiseAddition,
};
pub const REFLECTANCE: Effect = Effect{
    spatial_extent: 1,
    spectral_extent: 1000,
    effect_action: ComponentWiseMultiplicative,
};
pub const CONTAMINATION: Effect = Effect{
    spatial_extent: 1,
    spectral_extent: 1,
    effect_action: ComponentWiseMultiplicative,
};
pub const SLIT: Effect = Effect{
    spatial_extent: 4000,
    spectral_extent: 1,
    effect_action: Reshape,
};
pub const READ_NOISE: Effect = Effect{
    spatial_extent: 4000,
    spectral_extent: 1,
    effect_action: ComponentWiseAddition,
};
pub const VINIETTING: Effect = Effect{
    spatial_extent: 4000,
    spectral_extent: 1,
    effect_action: ComponentWiseMultiplicative,
};
pub const POINT_SPREAD_FUNCTION: Effect = Effect{
    spatial_extent: 10,
    spectral_extent: 1,
    effect_action: ConvolutionKernel,
};



#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Effect{
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







