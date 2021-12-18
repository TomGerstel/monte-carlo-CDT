#%% Imports
import numpy as np
import scipy.optimize as opt
import matplotlib.pyplot as plt

#%% Import data
# datafile = "meas_t40_l100_n2000_r0.3_1639847040.csv"

# datafile = "meas_t40_l100_n5000_r0.3_1639847975.csv"
# datafile = "meas_t40_l100_n5000_r0.3_1639848013.csv"
# datafile = "meas_t40_l100_n5000_r0.3_1639847059.csv"
# datafile = "meas_t40_l100_n5000_r0.3_1639848253.csv"

# datafile = "meas_t40_l100_n10000_r0.3_1639844562.csv"
# datafile = "meas_t40_l100_n10000_r0.3_1639848337.csv"
# datafile = "meas_t40_l100_n10000_r0.3_1639848412.csv"
datafile = "meas_t40_l100_n10000_r0.3_1639848532.csv"

t_max = 40
lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, t_max))

#%% Correlation time functions
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

def obs_trace(x):
    dx = x - np.mean(x)
    return dx/np.max(np.abs(dx))
    
def correlation_length_fit(t, tcor):
    return np.exp(-t/tcor)

#%% 
def sample_autocovariance(x, tmax):
    '''Compute the autocorrelation of the time series x for t = 0,1,...,tmax-1.'''
    x_shifted = x - np.mean(x)
    return np.array([np.dot(x_shifted[:len(x)-t],x_shifted[t:])/len(x) for t in range(tmax)])

def find_correlation_time(autocov):
    '''Return the index of the first entry that is smaller than autocov[0]/e.'''
    return np.where(autocov < np.exp(-1)*autocov[0])[0][0]

obs = np.std(lengths, axis=1)
autocov = sample_autocovariance(obs, 1000)
plt.plot(autocov/autocov[0])
print(find_correlation_time(autocov))


#%% Visualise correlations
obs = np.std(lengths, axis=1)[:500]
# obs = np.split(stds, 4)[3]
t_max = 500  # in sweeps
ts, autocor = correlation_profile(obs, t_max, resolution=300)
fit = opt.curve_fit(correlation_length_fit, ts, autocor)
tcor = fit[0][0]
tcor_err = np.sqrt(fit[1][0, 0])

print("t_cor: {} ± {}".format(tcor, tcor_err))
plt.plot(ts, autocor, label="Autocorrelation")
plt.plot(ts, correlation_length_fit(ts, tcor), label='Fit ($\pm \, 3\sigma$): $e^{-t/t_{cor}}$')
plt.fill_between(ts, correlation_length_fit(ts, tcor-3*tcor_err), correlation_length_fit(ts, tcor+3*tcor_err), alpha=0.3)
plt.plot(obs_trace(obs), alpha=0.1, label="Observable trace")
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
# %% Determine correlation times
M = len(obs)
tcors = []
tcors_err = []

# Parameters
t_max = 800  # in sweeps
parts = 4

for i in range(0, M):
    tcor = []
    for part in np.split(obs[i], parts):
        ts, autocor = correlation_profile(part, t_max//parts, resolution=300)
        fit = opt.curve_fit(correlation_length_fit, ts, autocor)
        tcor.append(fit[0][0])
    tcors.append(np.mean(tcor))
    tcors_err.append(np.std(tcor)/np.sqrt(parts))
# %% Visualise correlation time profiles
tcormean = np.mean(tcors)
print(tcormean)
plt.errorbar(Ls, tcors, yerr=tcors_err)
plt.hlines(tcormean, min(Ls), max(Ls), alpha=0.2)
plt.show()

# %%
