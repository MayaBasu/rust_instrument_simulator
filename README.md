# An Instrument Simulator in Rust

An instrument simulator designed to quickly generate large images from complex instruments.



# Configuring the Details

The details of the uvex instrument include the specification of data files or values to be used, as well as if a given particular effect (such as a quantum efficiency or transmission curve) should be turned on or off.

To generate a template details YAML, we can use the details default function:


```rust
fn main() {
    //Generate a default details file and write it to "configuration/details.yaml"
    let details = UVEX_Details::default("configuration/details.yaml");
}
```


You should get


```yaml


tma_details:
  spacecraft_pointing:
  - on
  - 1.0
  tma_m1_reflectance: #do not edit this
  - on  #edit this to turn "on" or "off" the effect
  - data/tma/tma_m1_reflectance #edit this to change the path
  tma_m1_contamination:
  - on
  - 2.0
#...clipped...
  image_plane_qe:
  - on
  - data/spectrograph/image_plane_qe
  image_plane_dead_pixels:
  - on
  - data/spectrograph/image_plane_dead_pixels
  image_plane_read_noise:
  - on
  - data/spectrograph/image_plane_read_noise
  image_plane_dark_current:
  - on
  - data/spectrograph/image_plane_dark_current



```

The YAML file has four sections, tma_details, fuv_details, nuv_details, and spectrograph_details. 
Each of these lists all the relevant effects. The names of these effects are important and should remain unchanged unless you will also be modifying the configuration of the uvex instrument.


Once you have a details file saved which has benn edited to include all the appropriate data paths, load the details file into the uvex instrument:

```rust

fn main() {
    //initialize the uvex instrument with the details in "configuration/details.yaml"
    let uvex = uvex::initialize_uvex("configuration/uvex");
}

```


# Sources

We can create single point source by specifying a position in the view area (values for x and y between 0 and 1) as well as a spectrum and an overall brightness factor which is between 0 and 1.
 ```rust

fn main() {
    let spectrum: [f64;spectral_resolution] = [0.1,0.13,/* Insert your spectrum here */0.2];
    let source_x = 0.2;
    let source_y = 0.3;
    let luminosity = 0.6;
    let point_source = point_source::new(source_x,source_y, spectrum,luminosity);
}

```
The UVEX instrument takes a source_list, which is a group of point_sources. We can either add custom point_sources to an empty source_list, or we can initialize a random source list to look at.


```rust
fn main() {
    
    //make three point sources: point_source1, point_source2, and point_source3
    //make a source list containing the first two sources
    let mut curated_source_list = source_list::new_from(vec![point_source1,point_source2]);
    //we can also add sources to an existing list
    curated_source_list.add_source(point_source3);
    
    //alternatively, we can generate a random star field of sources which all share a spectrum but which 
    
    
    let random_source_list = source_list::new_random_point_source_field(number_of_point_sources, //how many points
                                                                  min_brightness, //minimum brightness of the random range
                                                                  max_brightness, //maximum brightness of the random range
                                                                  min_x,//minimum x position of the random range
                                                                  max_x,//maximum x position of the random range
                                                                  min_y,//minimum y position of the random range
                                                                  max_y,//maximum x position of the random range
                                                                  spectrum); //shared spectrum
    
}


```


Once you have a source_list you are ready to feed it into the uvex_instrument and get images.

