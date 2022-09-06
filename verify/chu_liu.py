# Chu-Liu/Edmonds' Algorithm
# 最小全域有向木を再帰的に求める
# V: 頂点数, es: 辺集合, r: 根となる頂点番号
# 辺１つ(s->tのコストw)は(s, t, w)のlistを持つ
# 頂点は0..V-1

# cf: <https://tjkendev.github.io/procon-library/python/graph/chu-liu-edmonds.html>

def m(a):
    return ['rabcde'[e] for e in a]

def solve(V, es, r):
    mins = [(10**18, -1)]*V
    for s, t, w in es:
        mins[t] = min(mins[t], (w, s))
    mins[r] = (-1, -1)

    group = [0]*V
    comp = [0]*V
    cnt = 0

    used = [0]*V
    for v in range(V):
        if not used[v]:
            chain = []
            cur = v
            while cur!=-1 and not used[cur]:
                chain.append(cur)
                used[cur] = 1
                cur = mins[cur][1]
            if cur!=-1:
                cycle = 0
                for e in chain:
                    group[e] = cnt
                    if e==cur:
                        cycle = 1
                        comp[cnt] = 1
                    if not cycle:
                        cnt += 1
                if cycle:
                    cnt += 1
            else:
                for e in chain:
                    group[e] = cnt
                    cnt += 1
            print(f'chain={m(chain)}')
    print(f'group={group}')
    print(f'comp={comp}')
    print(f'{cnt=}')

    if cnt == V:
        return sum(map(lambda x:x[0], mins)) + 1

    res = sum(mins[v][0] for v in range(V) if v != r and comp[group[v]])

    n_es = []
    for s, t, w in es:
        gs = group[s]; gt = group[t]
        if gs == gt:
            continue

        if comp[gt]:
            n_es.append((gs, gt, w - mins[t][0]))
        else:
            n_es.append((gs, gt, w))

    print(f'solve({cnt}, {n_es}, {group[r]})')
    return res + solve(cnt, n_es, group[r])

n = 4
edges = [
    (0, 1, 1),
    (1, 2, 1),
    (2, 0, 1),
    (0, 3, 1),
]

# [r: 0, a: 1, b: 2, c: 3, d: 4, e: 5]
n = 6
edges = [
    (0, 3, 8),
    (0, 1, 2),
    (0, 2, 10),
    (1, 3, 2),
    (2, 1, 1),
    (2, 4, 1),
    (3, 2, 8),
    (3, 4, 3),
    (4, 5, 1),
    (5, 2, 2),
]
res = solve(n, edges, 0)
print(res)