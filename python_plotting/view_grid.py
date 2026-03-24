import numpy as np
import matplotlib.pyplot as plt


with open("/Users/mayabasu/RustroverProjects/rust_instrument_simulator/points") as f:
    data = f.read()[0:-1].split(",")
print(data)
elemets = []
for d in data:
    elemets.append(d)

x = []
y = []
colors = []
for (i,e) in enumerate(elemets):
    if i % 3 ==0:
        print(float(e[2:]))
        x.append(float(e[2:]))
    if i % 3 ==1:
        y.append(float(e))
    if i % 3 ==2:
        colors.append(e[2:-2])

print(colors)
for i in range(len(x)):

    plt.scatter(x[i],y[i],color = colors[i])
with open("/Users/mayabasu/RustroverProjects/rust_instrument_simulator/interp_points") as f:
    data = f.read()[0:-1].split(",")
print(data)
elemets = []
for d in data:
    elemets.append(d)

x = []
y = []
colors = []
for (i,e) in enumerate(elemets):
    if i % 3 ==0:
        print(float(e[2:]))
        x.append(float(e[2:]))
    if i % 3 ==1:
        y.append(float(e))
    if i % 3 ==2:
        colors.append(e[2:-2])

print(colors)
for i in range(len(x)):

    plt.scatter(x[i],y[i],color = colors[i])

plt.show()


