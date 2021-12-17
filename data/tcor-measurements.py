#%% Imports
import numpy as np
import scipy.optimize as opt
import matplotlib.pyplot as plt

#%% Import data
datafile = "meas_t40_l200_n1000_r0.3_1639763413.csv"
t_max = 40
lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, t_max))

#%% Correlation time
def autocorrelation(t: int, x: np.array):
    dx = (x - np.mean(x))
    if t == 0:
        return 1.0
    autocov = np.sum(dx[:-t]*dx[t:])
    return autocov/np.sum((dx*dx))

def correlation_profile(x: np.array, t_max: int, resolution=100):
    ts = (np.arange(resolution) * (t_max/resolution)).astype(int)
    return ts, np.vectorize(lambda t: autocorrelation(t, x))(ts)

def obs_trace(x):
    dx = x - np.mean(x)
    return dx/np.max(np.abs(dx))
    
def correlation_length_fit(t, tcor):
    return np.exp(-t/tcor)



stds = np.std(lengths, axis=1)
t_max = 800  # in sweeps
ts, autocor = correlation_profile(stds, t_max, resolution=300)

fit = opt.curve_fit(correlation_length_fit, ts, autocor)
tcor = fit[0][0]
tcor_err = np.sqrt(fit[1][0, 0])

print("t_cor: {} ± {}".format(tcor, tcor_err))
plt.plot(ts, autocor, label="Autocorrelation")
plt.plot(ts, correlation_length_fit(ts, tcor), label='Fit ($\pm \, 3\sigma$): $e^{-t/t_{cor}}$')
plt.fill_between(ts, correlation_length_fit(ts, tcor-3*tcor_err), correlation_length_fit(ts, tcor+3*tcor_err), alpha=0.3)
plt.plot(obs_trace(stds), alpha=0.1, label="Observable trace")
plt.xlim((0, t_max))
plt.legend()
plt.show()

#%% Read-in all parameter files from folder
import os
import json

datapath = "./"
datafiles = [os.path.join(datapath, f) for f in os.listdir(datapath) if os.path.isfile(os.path.join(datapath, f)) and f.endswith(".json")]

parameters = []
for datafile in datafiles:
    with open(datafile) as f:
        jsondata = json.load(f)
        parameters.append(jsondata)

#%% Import data based on wanted parameters
Ls = []
obs = []

def observable(lenghts):
    np.std(lenghts, axis=1)

for parameter_set in parameters:
    if parameter_set["timespan"] == 20:
        Ls.append(parameter_set["length"])
        datafile = datapath + parameter_set["name"] + ".csv"
        lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, 20))
        obs.append(observable(lengths))
# %%
