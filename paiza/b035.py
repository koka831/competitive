from operator import attrgetter
# (k, v) のソート手法の確立

class Score(object):

    def __init__(self, name, last, cur):
        self.name = name
        self.last = last
        self.cur = cur
        self.cur_rank = 0
        self.last_rank = None

    def add_dist(self, distance):
        self.cur += distance

    def __repr__(self):
        return '{} {} {}'.format(self.name, self.cur_rank, self.last_rank)

    def set_cur_rank(self, rank):
        self.cur_rank = rank

    def set_last_rank(self, rank):
        self.last_rank = rank


if __name__ == '__main__':
    n, m, t = map(int, input().split())
    members = {}
    last_rank = {}
    for i in range(n):
        a, p = input().split()
        members[a] = Score(a, int(p), 0)
        last_rank[a] = int(p)

    for i in range(m):
        _, w, x = input().split()
        s = members.get(w)
        s.add_dist(int(x))

    cur_list = []
    last_list = []
    for k, v in members.items():
        cur_list.append(v)
        last_list.append(v)

    cur_list = sorted(cur_list, key=attrgetter('name'))
    last_list = sorted(last_list, key=attrgetter('name'))

    cur_list = sorted(cur_list, reverse=True, key=attrgetter('cur'))
    last_list = sorted(last_list, reverse=True, key=attrgetter('last'))

    for i in range(len(cur_list)):
        s = cur_list[i]
        s.set_cur_rank(i)

    for i in range(len(last_list)):
        s = last_list[i]
        s.set_last_rank(i)


    for i in range(t):
        s = cur_list[i]
        if s.last_rank > t:
            print('{} {} new'.format(s.name, s.cur))
        elif s.last_rank == s.cur_rank:
            print('{} {} same'.format(s.name, s.cur))
        elif s.last_rank > s.cur_rank:
            print('{} {} up'.format(s.name, s.cur))
        elif s.last_rank < s.cur_rank:
            print('{} {} down'.format(s.name, s.cur))
