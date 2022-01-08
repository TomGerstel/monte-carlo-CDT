#%% Imports
import numpy as np
import scipy.optimize as opt
import matplotlib.pyplot as plt
from matplotlib import cm

#%% Import jsons
import os
import json

datapath = "./personal/tcor-measurements/L-dependence/"
datafiles = [os.path.join(datapath, f) for f in os.listdir(datapath) if os.path.isfile(os.path.join(datapath, f)) and f.endswith(".json")]

parameters = []
for datafile in datafiles:
    with open(datafile) as f:
        jsondata = json.load(f)
        parameters.append(jsondata)


# Ts = []
Ls = []
obs = []

def correlation_profile(x: np.array, tmax=None):
    L = np.mean(x)
    dx = x - L
    if tmax is None:
        tmax = x.shape[1]//2
    cor = np.zeros((x.shape[0], tmax))
    for t in range(tmax):
        norm = 100*L**1.015
        cor[:, t] = np.sum(dx * np.roll(dx, t, axis=1), axis=1)/norm
    return cor

def observable(lenghts, bakein=500):
    # return np.mean(correlation_profile(lengths[500:]), axis=0)
    return lengths/np.mean(lengths)
    # return np.std(lengths, axis=1, ddof=1)

for parameter_set in parameters:
    if parameter_set["timespan"] == 20:
        Ls.append(parameter_set["length"])
        datafile = datapath + parameter_set["name"] + ".csv"
        lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, 20))
        obs.append(observable(lengths))

# for parameter_set in parameters:
#     if parameter_set["length"] == 200:
#         T = parameter_set["timespan"]
#         Ts.append(T)
#         datafile = datapath + parameter_set["name"] + ".csv"
#         lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, T))
#         obs.append(observable(lengths))

tdata = sorted(zip(Ls, obs))
sdata = zip(*tdata)
Ls = np.array(next(sdata))

# tdata = sorted(zip(Ts, obs))
# sdata = zip(*tdata)
# Ts = np.array(next(sdata))
obs = np.array(next(sdata))

#%%
color = cm.viridis(np.linspace(0, 1, len(obs)))
plt.figure(figsize=(10, 6))
for i, obsi in enumerate(obs):
    plt.plot(obsi, label=Ls[i], c=color[i])
plt.legend()
plt.title("$L = 200$")
# plt.xlim((-1, 50))
plt.show()

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

#%%
def lengthcorrelation(t: int, x: np.array):
    """Correlation with periodic boundaries"""
    dx = (x - np.mean(x))
    autocov = np.sum((dx * np.roll(dx, t)))
    return autocov/np.sum(dx*dx)

def correlation_profile(x: np.array):
    ds = np.arange(len(x) // 2)
    return np.vectorize(lambda d: lengthcorrelation(d, x))(ds)

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

#%% 
def correlation_profile(x: np.array, tmax=None):
    dx = x - np.mean(x)
    if tmax is None:
        tmax = x.shape[1]//2
    cor = np.zeros((x.shape[0], tmax))
    for t in range(tmax):
        norm = np.sum(dx * dx, axis=1)
        cor[:, t] = np.sum(dx * np.roll(dx, t, axis=1), axis=1)/norm
    return cor
    

#%% Save plot
from scipy.stats import poisson

plt.figure(figsize=(10, 6))
data = obs[1].flatten()
x = np.bincount(data)/len(data)
l = np.arange(len(x))
plt.bar(l, x, width=1.0)
plt.vlines(200, 0.0, 0.016, colors="r")

sig2 = np.var(data)
p = 1 - sig2/200
n = 200/p
# plt.plot(l, np.exp(-(l-200)**2/2/sig2)/np.sqrt(2*np.pi*sig2), color='r')
plt.plot(l, poisson.pmf(l, 200)*0.55, color='r')
# %%

def autocorrelation(t: int, x: np.array):
    dx = (x - np.mean(x))
    if t == 0:
        return 1.0
    elif t >= len(x):
        return 0.0
    autocov = np.sum(dx[:-t]*dx[t:])
    return autocov/np.sum((dx*dx))

def correlation_profile(x: np.array, t_max: int, resolution=100):
    ts = (np.arange(resolution) * (t_max/resolution)).astype(int)
    return ts, np.vectorize(lambda t: autocorrelation(t, x))(ts)

#%%
def dist(l, Lambda):
    return 4.0 * l * Lambda * np.exp(-2.0*(np.sqrt(Lambda)*l))

lengths = np.loadtxt("personal/meas_t500_l20_n100_r0.4_1641638785.csv", dtype=int, usecols=range(0, 500), delimiter=',').flatten()
obs = lengths[500*4:]
plt.hist(obs, bins=20, density=True)
l = np.linspace(0, 100, 500)
plt.plot(l, dist(l, (20)**(-2)))

# %%
