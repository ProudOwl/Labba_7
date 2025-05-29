#include <iostream>
#include <string>
#include <cctype>
using namespace std;

string decodeString(const string& s) {
    string result;
    int i = 0;
    while (i < s.length()) {
        if (isdigit(s[i])) {
            int k = 0;
            while (i < s.length() && isdigit(s[i])) {
                k = k * 10 + (s[i] - '0');
                i++;
            }

            if (k < 1 || k > 300) {
                cerr << "Ошибка: число повторений должно быть от 1 до 300.\n";
                return "";
            }
            
            i++;
            
            int start = i;
            int bracketCount = 1;
            while (i < s.length() && bracketCount > 0) {
                if (s[i] == '[') bracketCount++;
                else if (s[i] == ']') bracketCount--;
                i++;
            }
            int end = i - 1; // позиция перед ']'
            
            // декодируем подстроку внутри скобок
            string decodedSubstr = decodeString(s.substr(start, end - start));
            
            // k раз декодированную подстроку к результату
            for (int j = 0; j < k; j++) {
                result += decodedSubstr;
            }
        } else if (s[i] != ']') {
            // простые символы
            result += s[i];
            i++;
        } else {
            i++;
        }
    }
    return result;
}

int main() {
    string input;
    cout << "Введите закодированную строку: ";
    cin >> input;

    // проверка длины строки
    if (input.length() < 1 || input.length() > 30) {
        cerr << "Ошибка: длина строки должна быть от 1 до 30 символов.\n";
        return 1;
    }

    // проверка символов
    for (char c : input) {
        if (!(islower(c) || isdigit(c) || c == '[' || c == ']')) {
            cerr << "Ошибка: строка содержит недопустимые символы.\n";
            return 1;
        }
    }

    string decoded = decodeString(input);
    cout << "Декодированная строка: " << decoded << endl;
    return 0;
}
