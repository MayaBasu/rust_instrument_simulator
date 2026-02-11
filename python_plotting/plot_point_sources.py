import numpy as np
import matplotlib.pyplot as plt
import yaml

spectral_resolution = 2
show_bins = False

with open('../sources', 'r') as file:
    source_list = yaml.safe_load(file)
sources = source_list.get('sources')


fig = plt.figure()
ax = fig.add_subplot(1, 1, 1)
pixels = np.linspace(0, 1, spectral_resolution+1)
ax.set_xticks(pixels)
ax.set_yticks(pixels)
ax.set_xlim([0, 1])
ax.set_ylim([0, 1])
ax.grid(visible=True)

for source in sources:
    print(source)
    source_x = source.get('source_x')
    source_y = source.get('source_y')
    source_bin = source.get('bin')
    if show_bins:
        ax.scatter(source_x, source_y, label="bin:" + str(source_bin))
    else:
        ax.scatter(source_x, source_y)


plt.legend()
plt.show()



