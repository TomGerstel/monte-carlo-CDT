import math
# Let op: ik weet niet of dit klopt en/of nuttig is
# find the number of labeled triangulations given N triangles and T timeslices:


def entropy(N, T, x=[], S=0):
    t = len(x)
    L_free = N // 2 - T - sum(x)
    if t < T - 1:
        for l in range(L_free + 1):
            x_t = l
            S += entropy(N, T, x + [x_t], S)
    elif t == T - 1:
        x_t = L_free
        return internal_entropy(x + [x_t])
    return S


def internal_entropy(x):
    S = 1
    for i in range(len(x)):
        N_up = x[i] + 1
        N_down = x[(i + 1) % len(x)] + 1
        S *= math.factorial(N_up + N_down) ** 2 // (math.factorial(N_up)
                                                    * math.factorial(N_down))
    return S


print(entropy(20, 4))
