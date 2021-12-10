#%% Imports
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
lengths = np.loadtxt("lengths_t20n800r3.csv", dtype=int) 
lengths

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

# Determine t_eq (in practice one would use at least 5*t_eq to be safe)
obs = observable_lengthstd(lengths)
print(estimate_teq(obs))

# Verification plot
(t_eq, _), (O_eq, _) = estimate_teq_Omean(obs)
ts = np.arange(len(obs))
plt.step(ts, obs, label="std of lengths")
plt.plot(ts, fit_function(ts, O_eq, t_eq), c='r', label="fit", alpha=0.6)
plt.vlines(3*t_eq, 0.0, 2.5, linestyles='dashed', label="$3 \, t_{eq}$", colors='r', alpha=0.7)
plt.legend()
plt.show()

#%% Correlation time

def autocorrelation(t: int, x: np.array):
    dx = (x - np.mean(x))
    if t == 0:
        return 1.0
    autocov = np.mean(dx[:-t]*dx[t:])
    return autocov/np.mean((dx*dx))

obsm = obs[900:]
ts = np.arange(int(len(obsm)/2))
autocor = np.vectorize(lambda t: autocorrelation(t, obsm))(ts)
plt.plot(ts, autocor)
# Then the correlation time can be determined by for example a fit on: exp(-t/t_cor).

#%% Determine length correlations
x = lengths[-2]

def lengthcorrelation(t: int, x: np.array):
    """Correlation with periodic boundaries"""
    dx = (x - np.mean(x))
    autocov = np.sum((dx * np.roll(dx, t)))
    return autocov/np.sum((dx*dx))

ds = np.arange(int(len(x)/2))
autocor = np.vectorize(lambda d: autocorrelation(d, x))(ds)
plt.plot(ds, autocor)
dx = x - np.mean(x)
plt.plot(dx/np.max(np.abs(dx)))