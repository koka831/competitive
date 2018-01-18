

n_engineer = int(input())
names = input().split()
n = int(input())
eng = {}

for k in names:
    eng[k] = 0

for i in range(n):
    s = input().split()
    eng[s[0]] += int(s[1])


for (k, v) in sorted(eng.items(), key=lambda k: k[1], reverse=True):
    print(k)
