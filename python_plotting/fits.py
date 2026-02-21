import sys

from astropy.io import fits
import matplotlib.pyplot as plt
import numpy as np

file = fits.open("/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/UVEX_FUV_PSF_1um_F001 1.fits")


data= file[0].data
print(file[0].header)

plt.imshow(data)

np.set_printoptions(threshold=sys.maxsize)
print(data)

data = np.array(data).flatten()
print(np.argmax(data))
print(data[1955])



with open("fits_data", "wb") as binary_file:
    for pixel in data:
        data_bytes = pixel.tobytes()
        binary_file.write(data_bytes)


plt.show()