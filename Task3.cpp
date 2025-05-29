#include <iostream>
#include <vector>
#include <climits>
using namespace std;

bool calculate(vector<int>& nums, int pos, int current, int target) {
    if (pos == nums.size()) {
        return current == target;
    }
    
    // сложение
    if (calculate(nums, pos + 1, current + nums[pos], target)) {
        return true;
    }
    
    // умножение
    if (calculate(nums, pos + 1, current * nums[pos], target)) {
        return true;
    }
    return false;
}

int main() {
    int N, S;
    cout << "Введите N: ";
    cin >> N;
    
    if (N == 0) {
        cout << "Ошибка" << endl;
        return 0;
    }
    
    vector<int> numbers(N);
    cout << "Введите " << N << " двузначных чисел(-ла): ";
    for (int i = 0; i < N; ++i) {
        cin >> numbers[i];
    }
    
    cout << "Введите число S: ";
    cin >> S;
    
    if (calculate(numbers, 1, numbers[0], S)) {
        cout << "Да, можно получить число " << S << " из данных чисел" << endl;
    } else {
        cout << "Нет, нельзя получить число " << S << " из данных чисел" << endl;
    }
    return 0;
}