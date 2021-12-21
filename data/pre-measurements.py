#%% Imports
import numpy as np
from numpy.ma import power
import scipy.optimize as opt
import matplotlib.pyplot as plt
import os
import json

#%% Import jsons
datapath = "./tcor-measurements/L-dependence/"
datafiles = [os.path.join(datapath, f) for f in os.listdir(datapath) if os.path.isfile(os.path.join(datapath, f)) and f.endswith(".json")]

parameters = []
for datafile in datafiles:
    with open(datafile) as f:
        jsondata = json.load(f)
        parameters.append(jsondata)


Ls = []
obs = []

def observable(lenghts):
    return np.std(lenghts, axis=1)

for parameter_set in parameters:
    if parameter_set["timespan"] == 20:
        Ls.append(parameter_set["length"])
        datafile = datapath + parameter_set["name"] + ".csv"
        lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, 20))
        obs.append(observable(lengths))

tdata = sorted(zip(Ls, obs))
sdata = zip(*tdata)
Ls = np.array(next(sdata))
obs = np.array(next(sdata))

#%% Batching
def power_fit(N, nu, A, N_c):
    return A*np.power(N - N_c, nu)

batches = 500
std_batch = []
for batch in np.split(obs, batches, axis=1):
    std_batch.append(np.mean(batch, axis=1))

Ns = Ls * 20
std_profile = np.mean(std_batch, axis=0)
std_err = yerr=np.std(std_batch, axis=0)/np.sqrt(batches - 1)
fit = opt.curve_fit(power_fit, Ns, std_profile, sigma=std_err, absolute_sigma=True)
params = fit[0]

plt.errorbar(Ns / 1000, std_profile, yerr=2*std_err, fmt='.', label="$\sigma$ of length profile")
plt.plot(Ns / 1000, power_fit(Ns, params[0], params[1], params[2]), alpha=0.6, label="$(N - N_c)^\\nu$ fit")
plt.legend()
plt.xlabel("$1000 \, N$")
plt.savefig("std-profile.pdf")
plt.show()
print(fit[0][0], np.sqrt(fit[1][0, 0]))

#%% Determine length correlations
obs = lengths
def lengthcorrelation(t: int, x: np.array):
    """Correlation with periodic boundaries"""
    dx = (x - np.mean(x))
    autocov = np.sum((dx * np.roll(dx, t)))
    return autocov/np.sum(dx*dx)

def correlation_profile(x: np.array):
    ds = np.arange(len(x) // 2)
    return np.vectorize(lambda d: lengthcorrelation(d, x))(ds)

cor_profile = np.zeros((obs.shape[0], obs.shape[1]//2))
for i in range(0, len(obs)):
    cor_profile[i] = correlation_profile(obs[i])
# %%
plt.plot(np.mean(cor_profile, axis=0))