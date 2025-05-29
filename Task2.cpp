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
    
    string decoded = decodeString(input);
    cout << "Декодированная строка: " << decoded << endl;
    return 0;
}