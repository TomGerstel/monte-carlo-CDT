#%% Imports
import numpy as np
import scipy.optimize as opt
import matplotlib.pyplot as plt

#%% Load all parameter files from folder (these files are measurements of the std every 0.1sweep to get a good understanding of teq)
import os
import json

datapath = "personal/teq-measurements"
parameterfiles = [os.path.join(datapath, f) for f in os.listdir(datapath) if os.path.isfile(os.path.join(datapath, f)) and f.endswith(".json")]

parameters = []
for parameterfile in parameterfiles:
    with open(parameterfile) as f:
        jsondata = json.load(f)
        parameters.append(jsondata)

#%% Read in relevant data from parameters
data = []

for p in parameters:
    L = p['length']
    std = np.loadtxt(datapath + "/" + p['name'] + ".csv", delimiter=',', usecols=0)
    data.append((L, std))

tups = zip(*sorted(data, key=lambda x: x[0]))
Ls = np.array(next(tups))
stds = np.array(next(tups))

# %% Determine equilibration time by assuming the observable on average behaves like: $$ <O> (1 - \exp{- t/t_eq}) $$

def fit_function(t, O_eq, t_eq):
    return O_eq * (1.0 - np.exp(-t/t_eq))

def plot_teq(obs, pause=1, max_range=10):
    # Verification plot in sweeps (pause is the amount of sweeps between each measurement)
    ts = pause*np.arange(len(obs)) # simulation time in sweeps
    fit = opt.curve_fit(fit_function, ts, obs)
    O_eq, t_eq = fit[0]

    plt.step(ts, obs, label="Observable: $\sigma_\ell$")
    plt.plot(ts, fit_function(ts, O_eq, t_eq), c='r', alpha=0.6, label='Fit: $O_{eq}(1 - e^{-t / t_{eq}})$')
    plt.vlines(3*t_eq, 0.0, 1.4*O_eq, linestyles='dashed', colors='r', alpha=0.7) # Line at 3*t_eq

    plt.xlabel("Monte Carlo time (in sweeps)")
    plt.xlim([-0.5*t_eq, max_range*t_eq])
    plt.ylim([-0.1*O_eq, 1.5*O_eq])
    plt.legend()
    
# Use this function to create a visualisation of the thermalisation
def plot_teq_stds(index, pause=0.1, filename=None):
    # verification plot based on Ls and stds for index
    plot_teq(stds[index], pause=pause)
    if filename is not None:
        plt.savefig(filename, bbox_inches='tight')
    plt.show()

#%% Estimate t_eq (in practice one would use at least 5*t_eq to be safe)
def estimate_teq(obs, pause=0.1):
    # Estimate teq in sweep units from data by fitting
    ts = pause*np.arange(len(obs)) # simulation time in sweeps
    fit = opt.curve_fit(fit_function, ts, obs)
    return fit[0][1], np.sqrt(fit[1][1, 1])

def estimate_teq_set(obs_set, pause=0.1, max_error=0.1):
    # Estimate teq with error by averaging multiple measurement sets
    teqs = np.array([estimate_teq(obs, pause=pause) for obs in obs_set])
    # Select only reasonable t_eq data
    teqs = teqs[teqs[:, 1] / teqs[:, 0] < max_error][:, 0]
    return np.mean(teqs), np.std(teqs) / np.sqrt(len(teqs) - 1)
    
Ls_unique = np.unique(Ls)
M = len(Ls_unique)
teqs = np.zeros(M)
teqs_err = np.zeros(M)

for i, L in enumerate(Ls_unique):
    teqs[i], teqs_err[i] = estimate_teq_set(stds[Ls == L])

#%% Visualise L-dependence of 
plt.errorbar(2 * 30 * Ls_unique / 1000, teqs, yerr=teqs_err, fmt=".-")
plt.xlabel("$1000 \, N$ (number of triangles)")
plt.ylabel("$t_{eq}$ (in sweeps)")
plt.title("Thermalisation at different $L$ and $T = 30$")
plt.savefig("teq-Ldep.pdf", bbox_inches='tight')
plt.show()

# %%
