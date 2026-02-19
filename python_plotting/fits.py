from astropy.io import fits
import matplotlib.pyplot as plt

file = fits.open("/Users/mayabasu/RustroverProjects/image_simulator_outline/python_plotting/UVEX_FUV_PSF_1um_F001 1.fits")
print(file.info())
print(file[0].header)
data= file[0].data
print(data)
plt.imshow(data)
plt.show()