#include <vector>
#include <list>
#include <map>
#include <set>
#include <deque>
#include <stack>
#include <bitset>
#include <algorithm>
#include <functional>
#include <numeric>
#include <utility>
#include <sstream>
#include <iostream>
#include <iomanip>
#include <cstdio>
#include <cmath>
#include <cstdlib>
#include <cctype>
#include <string>
#include <cstring>
#include <ctime>

using namespace std;

#define FOR(i, a, b) for (int i = (a); i < (b); ++i)
#define rep(i, n) FOR(i, 0, n)

/* const double EPS = 1e-10; */
/* const double PI = acos(-1.0); */

char board[30][30];
bool visited[30][30];

int H, W;
int dx[] = {0, 0, 1, -1};
int dy[] = {1, -1, 0, 0};
void dfs(int x, int y) {
    rep(k, 4) {
        int nx = x + dx[k], ny = y + dy[k];
        if (nx < 0 || nx >= H || ny < 0 || ny >= W) continue;
        if (board[nx][ny] == '#') continue;
        if (visited[nx][ny]) continue;
        visited[nx][ny] = true;
        dfs(nx, ny);
    }
}
