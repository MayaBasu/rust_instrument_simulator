import sys
from astropy.io import fits
import matplotlib.pyplot as plt
import numpy as np

file = fits.open("/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/UVEX_FUV_PSF_1um_F001 1.fits")
data = file[0].data
print(data)
plt.imshow(data)
plt.show()

np.set_printoptions(threshold=sys.maxsize)
data = np.array(data).flatten()

print(file[0].header)
print(np.argmax(data))
print(data[1955])

