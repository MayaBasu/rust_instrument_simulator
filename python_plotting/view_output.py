
import numpy as np
import matplotlib.pyplot as plt
import yaml

spectral_resolution = 4000

fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)

ax.grid(visible=False)


np.random.seed(0)
data = np.random.rand(spectral_resolution, spectral_resolution)

plt.imshow(data, cmap='viridis', interpolation='nearest')

plt.colorbar()
plt.xlabel('X-axis Label')
plt.ylabel('Y-axis Label')
plt.title('Heatmap')
plt.show()