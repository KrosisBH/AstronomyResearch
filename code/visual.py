from csv import reader
from vpython import *
import numpy as np

with open('xyz.csv', 'r') as read_obj:
    csv_reader = reader(read_obj)
    list_of_rows = list(csv_reader)


xyz = np.array(list_of_rows).astype(float).tolist()
print(len(xyz))

#gonna start with 100:

for (x,y,z) in xyz[0:100]:
    sphere(pos=vector(x,y,z))