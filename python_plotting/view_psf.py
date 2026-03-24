import numpy as np
import matplotlib.pyplot as plt


with open("/Users/mayabasu/RustroverProjects/rust_instrument_simulator/fuv") as f:
 data = f.read()[1:-1].split(",")

elemets = []
for d in data:
 elemets.append(float(d))
data_arr = np.array(elemets)
trim = 20
shaped = data_arr.reshape((64-2*trim)*18,(64-2*trim)*18)

plt.imshow(shaped)
plt.title(f"FUV PSF files, trimmed by {trim} pixels")
plt.savefig("fuv_psf")
plt.show()



