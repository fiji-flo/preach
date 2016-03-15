#!/usr/bin/python

from sys import argv
import random

def main():
  gra = "{}.gra".format(argv[-1])
  n = int(argv[1])
  m = int(argv[2])
  gengnm(n,m,gra)

def gengnm(n,m,fn):
  l = [[] for i in range(n)]
  p = list(range(n))
  random.shuffle(p)

  random.seed()

  for i in range(m):
    s = random.randrange(0,n)
    t = s
    done = False
    while not done:
      t = random.randrange(0,n)
      while t == s:
        t = random.randrange(0,n)
      if s < t and p[t] not in l[p[s]]:
        l[p[s]].append(p[t])
        done = True
      elif s > t and p[s] not in l[p[t]]:
        l[p[t]].append(p[s])
        done = True

  f = open(fn, "w")
  f.write("graph_for_greach\n")
  f.write("{}\n".format(len(l)))
  for s in range(len(l)):
    f.write("{}:".format(s))
    for t in l[s]:
      f.write(" {}".format(t))
    f.write(" #\n")
  f.close()

if __name__ == "__main__":
  main()
