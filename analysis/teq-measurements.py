#%% Imports
import os
import json
import numpy as np
import scipy.optimize as opt
import matplotlib.pyplot as plt

#%%[markdown]
# Autocorrelation as defined in the Monte Carlo Technique lectures
#
# **Autocorrelation** is defined as:
# $$\rho(t)=\operatorname{Corr}(f(X_i),f(X_{i+t})) := \frac{\mathbb{E}\Big[(f(X_i)-\mathbb{E}[f(X)])(f(X_{i+t})-\mathbb{E}[f(X)])\Big]}{\operatorname{Var}[f(X)]}.$$
# We can estimate the latter via the **sample autocovariance**,
# $$ \bar{\gamma}(t) = \frac{1}{n} \sum_{i=1}^{n-t}(f(X_i) - \overline{f(X)}_n)(f(X_{i+t}) - \overline{f(X)}_n),$$
# from which one obtains the **sample autocorrelation** as $$\bar{\rho}(t) = \frac{\bar{\gamma}(t)}{\bar{\gamma}(0)}$$
# by normalizing $\bar{\gamma}(t)$ by the sample variance $\bar{\gamma}(0)$.
# Note that there is a $1/n$ factor in front of the sum in $\bar{\gamma}(t)$, while one may instead expect an $1/(n-t)$ since there are $n-t$ terms. The reason is that this choice is often found to be more stable numerically. 

#%% Import data
datapath = "personal/teq-measurements"
datafiles = [os.path.join(datapath, f) for f in os.listdir(datapath) if os.path.isfile(os.path.join(datapath, f)) and f.endswith(".json")]

parameters = []
for datafile in datafiles:
    with open(datafile) as f:
        jsondata = json.load(f)
        parameters.append(jsondata)

#%% Create N and Data lists (for std), can take a while for large datafiles
M = len(parameters)
Ns = []
stds = []

for i, p in enumerate(parameters):
    Ns.append(2*p['length']*p['timespan'])
    stds.append(np.loadtxt(datapath + "/" + p['name'] + ".csv", delimiter=',', usecols=0))

tups = zip(*sorted(zip(Ns, stds), key=lambda x: x[0]))
Ns = np.array(next(tups))
stds = np.array(next(tups))

%store Ns
%store stds

# %% Determine equilibration time
# Determine t_eq by assuming the observable on average behaves like:
# $$ <O> (1 - \exp{- t/t_eq}) $$

def observable_lengthstd(lengths):
    return np.std(lengths, axis=1)

def fit_function(t, O_eq, t_eq):
    return O_eq * (1.0 - np.exp(-t/t_eq))

def estimate_teq(obs):
    fit = opt.curve_fit(fit_function, np.arange(len(obs)), obs)
    return fit[0][1], np.sqrt(fit[1][1, 1])

def estimate_teq_Omean(obs):
    fit = opt.curve_fit(fit_function, np.arange(len(obs)), obs)
    return (  ( fit[0][1], np.sqrt(fit[1][1, 1]) ),
        ( fit[0][0], np.sqrt(fit[1][0, 0]) )  )

def plot_teq(obs, sweep=100, max_range=10):
    # Verification plot in sweeps (a sweep is N: the amount of triangles)
    (t_eq, _), (O_eq, _) = estimate_teq_Omean(obs)
    ts = np.arange(len(obs))
    plt.step(ts/sweep, obs, label="std of lengths")
    plt.plot(ts/sweep, fit_function(ts, O_eq, t_eq), c='r', alpha=0.6)
    plt.vlines(3*t_eq/sweep, 0.0, 1.2*O_eq, linestyles='dashed', colors='r', alpha=0.7)
    plt.xlabel("Monte Carlo time (in sweeps)")
    plt.xlim([-0.2*t_eq/sweep, max_range*t_eq/sweep])
    plt.plot()

def plot_teq_stds(index, filename=None):
    # verification plot based on Ns and stds for index
    plot_teq(stds[index], sweep=10)
    if filename is not None:
        plt.savefig(filename)
    plt.show()

#%% Estimate t_eq
# Determine t_eq (in practice one would use at least 5*t_eq to be safe)
teqs = np.zeros(len(Ns)//10)
teqs_err = np.zeros(len(Ns)//10)

for i in range(len(Ns)//10):
    teq_batch = np.zeros(10)
    for j in range(0, 10):
        obs = stds[10*i+j]
        teq_batch[j], teq_err = estimate_teq(obs)
        if teq_err/teq_batch[j] > 0.1:
            print("The error in t_eq for index: {} is larger than 10%".format(10*i+j))
    teqs[i], teqs_err[i] = np.mean(teq_batch), np.std(teq_batch)/np.sqrt(10 - 1)

%store teqs
%store teqs_err
#%% Look at the N-dependance of t_eq
%store -r Ns
%store -r stds
%store -r teqs
%store -r teqs_err

plt.errorbar(np.mean(np.split(Ns, len(Ns)//10), axis=1).astype(int) / 1000, teqs/10, yerr=teqs_err/10, fmt=".-")
plt.ylim(-10, 100)
plt.xlabel("$1000 \, N$ (number of triangles)")
plt.ylabel("$t_{eq}$ (in sweeps)")
plt.savefig("teq-plot.pdf")
plt.show()


#%% Single file import
stds = np.loadtxt("teq-measurements/meas_t30_l20_n1000_r0.4_1640107872.csv", delimiter=',', dtype=float, usecols=0)
# %%
