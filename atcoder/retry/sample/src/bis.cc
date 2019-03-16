#include<iostream>
#include<vector>

using namespace std;

vector<int> a = {1, 14, 32, 51, 51, 243, 419};

bool isOk(int idx, int key) {
    if (a[idx] >= key) return true;
    return false;
}

int binary_search(int key) {
    int ng = -1;
    int ok = (int) a.size();

    while (abs(ok - ng) > 1) {
        int mid = (ok + ng) / 2;

        if (isOk(mid, key)) ok = mid;
        else ng = mid;
    }
    return ok;
}


int main() {
    string ans;
    cin >> ans;
    int left = 20, right = 30;
    while (right - left > 1) {
        int mid = left + (right - left) / 2;
        cin >> ans;
        if (ans == "yes") left = mid;
        else right = mid;
    }

    cout << left << endl;
}
