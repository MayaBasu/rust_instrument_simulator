# An Instrument Simulator in Rust

An instrument simulator designed to quickly generate large images from complex instruments.

# Organization
Once you understand how rust_image_simulator breaks down
a telescope, it will be easy to implement any design you want.

## Objects

rust_image_simulator is very object-oriented. The following, in order of nesting, are the objects which it uses:

- Instrument: The entirety of the instrument. In our case, the entire UVEX instrument
- TelescopeObject: A physical object inside the telescope such as the dichroic, a mirror, or a detector.
- Effect: A specific effect that a telescope object has on the signal. For example, a mirror might have an Effect corresponding to a transmission curve which diminishes the brightness of the light. Or, a detector might have a dark current effect which adds noise to the signal


The Instrument houses a number of TelescopeObjects (one for each mirror, detector, etc.) and each TelescopeObject houses a number of effects which it applies to the light (transmission, noise, efficiency, etc.)


## Light Flow

Now that we have an instrument full of TelescopeObjects which each have certain Effects, we need to know the path that the signal travels through these objects.

1. The Instrument has an entry_point. This is the name of one of it's TelescopeObjects which is the first to receive the light. In normal uvex operation, this would be the first mirror.

2. Each TelescopeObject has a list of recipients. These recipients are other TelescopeObjects that receive it's output.
For example, after the third mirror of the optical train, I want to send light to the slit and the FUV and NUV paths of the dichroic. 
The third mirror would then have a list of three recipients.
Mirror one in the initial optical train however, only sends light to mirror 1. So the only recipient of the light from mirror 1 is mirror 2. 

3. The Instrument has a list of measurement_points. 
These are a list of the Telescope objects after which we want to save data. 
If you only want the final image you could list the detectors 
(which would be particular TelescopeObjects with the effects such as quantum efficiency, dark current, and read noise) 
or, you could add in more TelescopeObjects to this list to track the propagation of the signal through the Instrument.

Points 1 and 2 allow us to propagate the signal through the telescope. 
We start at the entry_point object, and then look at its recipient's list and send the signal to those TelescopeObjects, and so on. 
Point 3 allows us to put "probes" into the instrument to see the signal wherever we choose.

# On the generic nature of the code

The code in rust-image-simulator, although designed with UVEX in mind, is almost completely generic. 
The actual set up of which TelescopeObjects to use and which Effects to give them is left to elaborate configuration files.

By writing very simple and generic code which follows directions of YAML files, 
we put most of the onus of making sure the simulation is correct on setting up the YAML files, instead of debugging code (which, at least in theory, is very minimal).

There are function in rust_image_simulator which set up the YAML files, namely in uvex.rs. 
**However, all of these files could be written by hand, or via a python script**, allowing you to program the image simulator without touching any Rust.

The setup of the rust code is intensely generic and adaptible, making the likelihood of a user having to fiddle with it to add anything very small.

- Want your instrument to have 5 arms and a maze of optics?
- Want to add optical elements never before used by humankind?
- Want to have custom effects such as imprinting your data with a different smiley face at every frequency?

You can do all of these things **without** writing any rust code! All you need is to write some YAML files.

## Expected Workflow

To summarize, the work flow is 

1. Create a YAML: write by hand **or** use python functions **or** use rust_instrument_simulator functions
2. Load the YAML into a rust Instrument structure and run it with one function call

So yes, technically rust_image_simulator is written in rust. However, you really only need one line of rust to run any configuration of image simulator that the code can accommodate, as all customization occurs through the YAML file. 


# YAML configuration files

All the specificity of a simulated instrument comes from YAML files. Let's work our way up and look at how we encode our instrument.

## Effects


Let's create a new effect with functons in rust_instrument_simulator (instead of writing the YAML directly without rust, which we could also do) and see what it's YAML looks like:

```rust

fn main() {
    //make a new quantum efficiency object called test_qe, with corresponding data file test_qe_data.dat
    let qe = QUANTUM_EFFICIENCY.new("test_qe", "test_qe_data.dat");
    //serialize this quantum efficiency to qe_yaml
    qe.write_to_yaml("qe.yaml");
}


```

In the first line, we create a new Effect of EffectType QUANTUM_EFFICIENCY. You can define any effect you like, but for simplicity, rust_image_simulator has a list of Effects which you might encounter, and we will stick to these in the examples.
If we look at QUANTUM_EFFICIENCY, we can see that it is an instance of the EffectType structure. 
```rust

pub const QUANTUM_EFFICIENCY: EffectType = EffectType {
    spatial_extent: spatial_resolution,
    spectral_extent: spectral_resolution,
    effect_action: ComponentWiseMultiplicative,
};

```

spatial_extent tells us how many spatial data points there are. Likewise for the spectral_extent.
These are set to constants which are specific for UVEX. Here they are set to 4000 and 1000:

```rust

pub const spectral_resolution:usize  = 1000;
pub const spatial_resolution:usize  = 4000;

```


That effect_action is
set to ComponentWiseMultiplication tells us that quantum efficiency multiplies the signal by the data in test_qe_data.dat component wise. If this was a read noise, this would instead
be ComponentWiseAddition. If this was a PSF, this would be convolution.
Next, we called the .write_to_yaml() method. This writes out the following YAML file:



```yaml
effect_label: test_qe
effect_type:
  spatial_extent: 4000
  spectral_extent: 1000
  effect_action: ComponentWiseMultiplicative
active: true
data_path: test_qe_data.dat
```

The active boolean determines if the effect will be used when the instrument simulator is run.
We could have instead skipped to this point and writen the YAML file in some other manner, the result will be the same.
**What you see above in this YAML is the only information which the "engine" of the image simulator is going off of**
There are no specialized quantum efficiency functions etc. outside of functions which help write the YAMLs.


## TelescopeObjects

Let's take a look at the TelescopeObject structure:

```rust 
pub struct TelescopeObject{
    pub unique_label: String,
    //This is a label specifying the exact instance of this effect.
    //For example, M1 verses M2
    pub effects: Vec<Effect>,
    //A vector of the effects that this object applies to the light
    pub recipients: Vec<String>,
    //A list of objects that receive the output light from this object
}
```

First, we have a unique name for each object. Second, we have a list of effects that this TelescopeObject applies to the signal, and finally we have a list of recipients. The recipients are the
unique names of other TelescopeObjects, see the Light Flow section above. 

An example of this in action is the following function in uvex.rs 
which initializes the first three mirrors of the telescope and gives them each a contamination and a reflectance.

You will recognize the effects being initialized as before, with CONTAMINATION and REFLECTANCE instead of 
QUANTUM_EFFICIENCY. Because these three are all EffectTypes, the initialization with the .new() function will always take 
the name of this particular instance, and a path pointing to where the data is stored. 


Then, we initialize three TelescopeObjects with a name, a vector of their effects, and a vector of the names of their recipients.


```rust
pub fn initialize_tma(contamination_data_path1:&str, reflectance_data_path1:&str,
                      contamination_data_path2:&str, reflectance_data_path2:&str,
                      contamination_data_path3:&str, reflectance_data_path3:&str
) -> (TelescopeObject, TelescopeObject, TelescopeObject){

    //contamination for each mirror in the tma (this is a single number)
    let m1_contamination = CONTAMINATION.new("m1_contamination", contamination_data_path1);
    let m2_contamination = CONTAMINATION.new("m2_contamination", contamination_data_path2);
    let m3_contamination = CONTAMINATION.new("m3_contamination", contamination_data_path3);

    //reflectivity as a function of wavelength for each mirror in the tma
    let m1_reflectivity = REFLECTANCE.new("m1_reflectivity", reflectance_data_path1);
    let m2_reflectivity = REFLECTANCE.new("m2_reflectivity", reflectance_data_path2);
    let m3_reflectivity = REFLECTANCE.new("m3_reflectivity", reflectance_data_path3);


    //now we initialize the mirror objects with these effects
    let m1 = TelescopeObject::new("m1",
                                  vec![m1_reflectivity,m1_contamination],
                                  vec!["m2"]);
    let m2 = TelescopeObject::new("m2",
                                  vec![m2_reflectivity,m2_contamination],
                                  vec!["m3"]);
    let m3 = TelescopeObject::new("m3",
                                  vec![m3_reflectivity,m3_contamination],
                                  vec![]);
    (m1,m2,m3)

}

```

Let's see what the YAML for the TMA is, with the following code, we get this YAML for mirror 1:

```yaml
unique_label: m1
effects:
- effect_label: m1_reflectivity
  effect_type:
    spatial_extent: 1
    spectral_extent: 1000
    effect_action: ComponentWiseMultiplicative
  active: true
  data_path: reflectance_data1
- effect_label: m1_contamination
  effect_type:
    spatial_extent: 1
    spectral_extent: 1
    effect_action: ComponentWiseMultiplicative
  active: true
  data_path: contamination_data1
recipients:
- m2
```

Here the reflectivity curve is assumed to be a function of frequency (why spectral_extent = 1000) but not space (spatial_extent = 1).

And the contamination of the mirror is assumed to be a single constant. Again, if you wanted to assume contamination was spatially dependent, 
or frequency dependent, all you would have to do would be to change spatial_extent and spectral_extent in your YAML, and make sure it matches with the data in your data path.


### Instrument

Once we have Effects which are then added to TelescopeObjects, we can add these TelescopeObjects to an Instrument. The Instrument structure is as follows:

```rust
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
```

The Instrument has a name (instrument_label), in case you want to make multiple instruments and compare them. The Instrument
has an entry point which is the name of one of the TelescopeObjects listed in the telescope_objects vector. This is a 
vector of all the telescope objects which are in the Instrument (every mirror, slit, dichroic, detector etc.)
Finally, we can specify any number of these TelescopeObjects as measurement points at which we want to save data.

Here is an example where add our three mirrors from before, as well as a detector, to an instrument:

```rust
pub fn initialize_uvex(){
    let contamination_data_path = "contamination_data";
    let reflectance_data_path = "reflectance_data";
    let qe_data_path = "qe_data";
    let read_noise_data_path = "read_noise_data";

    //for simplicity, we assume all three mirrors have the same contamination and reflectance
    let (m1,m2, mut m3) = initialize_tma(
        contamination_data_path, reflectance_data_path,
        contamination_data_path, reflectance_data_path,
        contamination_data_path, reflectance_data_path);
    //initialize a detector with a quantum efficiency and a read noise
    let detector_qe = QUANTUM_EFFICIENCY.new("detector_qe", qe_data_path);
    let detector_read_noise = READ_NOISE.new("detector_read_noise", read_noise_data_path);
    let detector = TelescopeObject::new("detector",
                                        vec![detector_qe,detector_read_noise],vec![]);

    //we have to add the detector as a recipient of mirror 3
    m3.add_recipient("detector");
    //make a new instrument called "uvex" with the three mirrors and the detector, with entry point m1
    let mut uvex = Instrument::new("uvex", "m1");
    uvex.add_object(m1);
    uvex.add_object(m2);
    uvex.add_object(m3);
    uvex.add_object(detector);
    //we want to "measure" the instrument after the detector effects are applied:
    uvex.add_measurement_point("detector");
    //serialize the whole instrument to a YAML
    uvex.write_to_yaml("uvex.yaml")


}
```

And here is the output! This is getting long, so it is no longer preferable to write it by hand. However, it doesn't have to be written with rust_image_simulator functions. It could be done by serializing python objects.
```yaml
instrument_label: uvex
entry_point: m1
telescope_objects:
- unique_label: m1
  effects:
  - effect_label: m1_reflectivity
    effect_type:
      spatial_extent: 1
      spectral_extent: 1000
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: reflectance_data
  - effect_label: m1_contamination
    effect_type:
      spatial_extent: 1
      spectral_extent: 1
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: contamination_data
  recipients:
  - m2
- unique_label: m2
  effects:
  - effect_label: m2_reflectivity
    effect_type:
      spatial_extent: 1
      spectral_extent: 1000
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: reflectance_data
  - effect_label: m2_contamination
    effect_type:
      spatial_extent: 1
      spectral_extent: 1
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: contamination_data
  recipients:
  - m3
- unique_label: m3
  effects:
  - effect_label: m3_reflectivity
    effect_type:
      spatial_extent: 1
      spectral_extent: 1000
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: reflectance_data
  - effect_label: m3_contamination
    effect_type:
      spatial_extent: 1
      spectral_extent: 1
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: contamination_data
  recipients:
  - detector
- unique_label: detector
  effects:
  - effect_label: detector_qe
    effect_type:
      spatial_extent: 4000
      spectral_extent: 1000
      effect_action: ComponentWiseMultiplicative
    active: true
    data_path: qe_data
  - effect_label: detector_read_noise
    effect_type:
      spatial_extent: 4000
      spectral_extent: 1
      effect_action: ComponentWiseAddition
    active: true
    data_path: read_noise_data
  recipients: []
measurement_points:
- detector
```






# Input Data


TODO



