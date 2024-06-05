h, w = map(int, input().split())
ss = [input() for _ in range(h)]
tt = [input() for _ in range(h)]

ss = sorted(zip(*ss))
tt = sorted(zip(*tt))

print("Yes" if ss == tt else "No")
