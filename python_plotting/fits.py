import sys
from astropy.io import fits
import matplotlib.pyplot as plt
import numpy as np


for n in [35]:
    path = "/Users/mayabasu/RustroverProjects/image_simulator_outline/data/demo/demo_psf/FUV PSF/UVEX_FUV_PSF_1um_F"+ f'{n:03}'+".fits"

    file = fits.open(path)
    data = file[0].data


    header = file[0].header
    print(header)
    print(header["XPOS"])


    #np.set_printoptions(threshold=sys.maxsize)
    #data = np.array(data).flatten()


    #print(np.argmax(data))
    #print(data[1955])

    plt.imshow(data)
    plt.show()

