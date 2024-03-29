#%% Imports
import numpy as np
import scipy.optimize as opt
import matplotlib.pyplot as plt

#%% Import single datafiles
datafile = "meas_t40_l100_n100000_r0.3_1639905425.csv"

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

def find_tcor(obs, batch_count=5, t_max=1000, resolution=200):
    # Determine the autocorrelation and error based on batching
    tcor = []
    left_over = len(obs) // batch_count
    for batch in np.array_split(obs[:batch_count*left_over], batch_count):
        # autocov = sample_autocovariance(batch, t_max)
        # tcor.append(find_correlation_time(autocov))
        ts, autocor = correlation_profile(batch, t_max=t_max, resolution=resolution)
        fit = opt.curve_fit(correlation_length_fit, ts, autocor)
        tcor.append(fit[0][0])
    return np.mean(tcor), np.std(tcor)/np.sqrt(batch_count - 1)

# #%% Lecture way of computing autocovariance
# def sample_autocovariance(x, tmax):
#     '''Compute the autocorrelation of the time series x for t = 0,1,...,tmax-1.'''
#     x_shifted = x - np.mean(x)
#     return np.array([np.dot(x_shifted[:len(x)-t],x_shifted[t:])/len(x) for t in range(tmax)])

# def find_correlation_time(autocov):
#     '''Return the index of the first entry that is smaller than autocov[0]/e.'''
#     return np.where(autocov < np.exp(-1)*autocov[0])[0][0]

# obs = np.std(lengths, axis=1)
# autocov = sample_autocovariance(obs, 2000)
# plt.plot(autocov/autocov[0])
# print(find_correlation_time(autocov))


#%% Visualise correlations
obs = np.std(lengths, axis=1)[500:]
t_max = 4000  # in sweeps
ts, autocor = correlation_profile(obs, t_max, resolution=300)
fit = opt.curve_fit(correlation_length_fit, ts, autocor)
tcor = fit[0][0]
tcor_err = np.sqrt(fit[1][0, 0])

print("t_cor: {} ± {}".format(tcor, tcor_err))
plt.plot(ts, autocor, label="Autocorrelation")
plt.plot(ts, correlation_length_fit(ts, tcor), label='Fit ($\pm \, 3\sigma$): $e^{-t/t_{cor}}$', alpha=0.6)
plt.fill_between(ts, correlation_length_fit(ts, tcor-3*tcor_err), correlation_length_fit(ts, tcor+3*tcor_err), alpha=0.3)
plt.plot(0.4 + obs_trace(obs), alpha=0.1, label="Observable trace")
plt.xlim((0, t_max//2))
plt.xlabel("Markov-chain steps")
plt.ylim(-0.19, 1.09)
plt.legend()
plt.savefig("tcor-decay.pdf")
plt.show()

#%% Test batching tcor
counts = np.arange(1, 20)
tcors = np.zeros(counts.shape)
tcors_err = np.zeros(counts.shape)
for i, count in enumerate(counts):
    tcors[i], tcors_err[i] = find_tcor(obs, batch_count=count)

plt.errorbar(counts, tcors, yerr=tcors_err, fmt=".k")
plt.xticks(counts)
plt.show()

#%% Read-in ALL parameter files from folder
import os
import json

datapath = "./tcor-measurements/L-dependence/"
datafiles = [os.path.join(datapath, f) for f in os.listdir(datapath) if os.path.isfile(os.path.join(datapath, f)) and f.endswith(".json")]

parameters = []
for datafile in datafiles:
    with open(datafile) as f:
        jsondata = json.load(f)
        parameters.append(jsondata)

#%% Import r-dependence data
rs = []
obs = []

def observable(lenghts):
    return np.std(lenghts, axis=1)

for parameter_set in parameters:
    if parameter_set["timespan"] == 20:
        rs.append(parameter_set["move_ratio"])
        datafile = datapath + parameter_set["name"] + ".csv"
        lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, 20))
        obs.append(observable(lengths))

tdata = sorted(zip(rs, obs))
sdata = zip(*tdata)
rs = np.array(next(sdata))
obs = np.array(next(sdata))

# %% Determine correlation times
M = len(obs)    
tcors = np.zeros(M)
tcors_err = np.zeros(M)

for i in range(0, M):
    tcors[i], tcors_err[i] = find_tcor(obs[i], batch_count=5)

# Visualise
plt.errorbar(rs, tcors, yerr=tcors_err)
plt.xlabel("$r$ (move-ratio)")
plt.ylabel("$t_{cor}$ (MC cor. time in sweeps)")
plt.title("Correlation time at $T = 20$ and $L = 80$ ($N = 3200$)")
plt.savefig("tcor_r_t20_l80.pdf")
plt.show()


#%% Import L dependence data
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
tcors = np.zeros(M)
tcors_err = np.zeros(M)

# Parameters
for i in range(0, M):
    tcors[i], tcors_err[i] = find_tcor(obs[i], batch_count=5)

# %% Visualise correlation time profiles
tcormean = np.mean(tcors)
print(tcormean)
plt.errorbar(Ls, tcors, yerr=tcors_err)
plt.hlines(tcormean, min(Ls), max(Ls), alpha=0.2)
plt.xticks(np.arange(max(Ls)//20)*20 + 20)
plt.xlabel("L (average space length: N/T)")
plt.ylabel("$t_{cor}$ (MC cor. time in sweeps)")
plt.title("Correlation time at $T = 20$ and $r = 0.3$")
plt.savefig("tcor_t20_r0.3.pdf")
plt.show()

#%% Import T dependence data
Ts = []
obs = []

def observable(lenghts):
    return np.std(lenghts, axis=1)

for parameter_set in parameters:
    if parameter_set["length"] == 200:
        T = parameter_set["timespan"]
        Ts.append(T)
        datafile = datapath + parameter_set["name"] + ".csv"
        lengths = np.loadtxt(datafile, delimiter=',', dtype=int, usecols=range(0, T))
        obs.append(observable(lengths))

tdata = sorted(zip(Ts, obs))
sdata = zip(*tdata)
Ts = np.array(next(sdata))
obs = np.array(next(sdata))

# %% Determine correlation times
M = len(obs)
tcors = np.zeros(M)
tcors_err = np.zeros(M)

# Parameters
for i in range(0, M):
    tcors[i], tcors_err[i] = find_tcor(obs[i], batch_count=10)


# %% Visualise correlation time profiles
# tcormean = np.mean(tcors)
# print(tcormean)
fit = opt.curve_fit(lambda x, a, b: a*x + b, Ts, tcors, sigma=tcors_err)
plt.errorbar(Ts, tcors, yerr=tcors_err)
plt.plot(Ts, fit[0][0]*Ts + fit[0][1], alpha=0.3)
# plt.hlines(tcormean, min(Ls), max(Ls), alpha=0.2)
# plt.xticks(np.arange(max(Ls)//20)*20 + 20)
plt.xlabel("T (timespan: $N = T \cdot L$)")
plt.ylabel("$t_{cor}$ (MC cor. time in sweeps)")
plt.title("Correlation time at $L = 200$ and $r = 0.4$")
print("slope: {} ± {}".format(fit[0][0], np.sqrt(fit[1][0, 0])))
plt.savefig("tcor_t_l200_r0.4.pdf")
plt.show()

# %% Analyse std
def power_fit(N, nu, N_c):
    return np.power(N - N_c, nu)


batches = 100
data = np.array([np.mean(batch, axis=1) for batch in np.split(obs, batches, axis=1)]) # Batched std
obs_mean = np.mean(data, axis=0)
obs_err = np.std(data, axis=0)/np.sqrt(batches - 1)
plt.errorbar(Ls, obs_mean, yerr=obs_err)
# plt.show()
# %%
Ls_ext = np.arange(200)
fit = opt.curve_fit(power_fit, Ls, obs_mean, sigma=obs_err, absolute_sigma=True)
plt.plot(Ls_ext, power_fit(Ls_ext, nu=fit[0][0], N_c=fit[0][1]), alpha=0.3)
plt.errorbar(Ls, obs_mean, yerr=obs_err)
plt.title("nu = {:.4}".format(fit[0][0]))
plt.xlabel("$L$")
plt.ylabel("std")
plt.savefig("critical_exp")
plt.show()
# %%
