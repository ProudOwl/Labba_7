#include <iostream>
#include <vector>
#include <cmath>
#include <utility>
#include <algorithm>

using namespace std;

// Рекурсивная функция для построения кривой Коха
void kochCurve(vector<pair<double, double>>& points, 
               double x1, double y1, 
               double x2, double y2, 
               int depth) {
    if (depth == 0) {
        points.emplace_back(x1, y1);
        return;
    }
    
    double dx = x2 - x1;
    double dy = y2 - y1;
    
    // Точки для разделения отрезка на 3 части
    double xA = x1 + dx / 3;
    double yA = y1 + dy / 3;
    double xB = x1 + 2 * dx / 3;
    double yB = y1 + 2 * dy / 3;
    
    // Точка вершины равностороннего треугольника
    double xC = (x1 + x2) / 2 - (y2 - y1) * sqrt(3) / 6;
    double yC = (y1 + y2) / 2 + (x2 - x1) * sqrt(3) / 6;
    
    // Рекурсивно строим 4 отрезка
    kochCurve(points, x1, y1, xA, yA, depth - 1);
    kochCurve(points, xA, yA, xC, yC, depth - 1);
    kochCurve(points, xC, yC, xB, yB, depth - 1);
    kochCurve(points, xB, yB, x2, y2, depth - 1);
}

// Функция для рисования линии между двумя точками
void drawLine(vector<string>& canvas, int x1, int y1, int x2, int y2) {
    int dx = abs(x2 - x1);
    int dy = abs(y2 - y1);
    int sx = x1 < x2 ? 1 : -1;
    int sy = y1 < y2 ? 1 : -1;
    int err = dx - dy;
    
    while (true) {
        if (x1 >= 0 && x1 < canvas[0].size() && y1 >= 0 && y1 < canvas.size()) {
            canvas[y1][x1] = '*';
        }
        
        if (x1 == x2 && y1 == y2) break;
        
        int e2 = 2 * err;
        if (e2 > -dy) {
            err -= dy;
            x1 += sx;
        }
        if (e2 < dx) {
            err += dx;
            y1 += sy;
        }
    }
}

int main() {
    setlocale(LC_ALL, "rus");
    int depth;
    cout << "Введите глубину рекурсии для кривой Коха: ";
    cin >> depth;
    
    // Начальные точки (ориентированы для лучшего отображения)
    double x1 = 0, y1 = 0;
    double x2 = 1, y2 = 0;
    
    vector<pair<double, double>> points;
    kochCurve(points, x1, y1, x2, y2, depth);
    points.emplace_back(x2, y2); // Добавляем последнюю точку
    
    // Находим минимальные и максимальные координаты
    double minX = points[0].first, maxX = points[0].first;
    double minY = points[0].second, maxY = points[0].second;
    
    for (const auto& point : points) {
        minX = min(minX, point.first);
        maxX = max(maxX, point.first);
        minY = min(minY, point.second);
        maxY = max(maxY, point.second);
    }
    
    // Вычисляем оптимальный размер холста с небольшими отступами
    const double margin = 0.1; // 10% отступ
    double width = maxX - minX;
    double height = maxY - minY;
    
    // Масштабируем координаты для вывода в консоль
    const int consoleWidth = 200; // Ширина консоли в символах
    const int consoleHeight = 60; // Высота консоли в строках
    
    double scaleX = (consoleWidth - 1) / (width * (1 + 2 * margin));
    double scaleY = (consoleHeight - 1) / (height * (1 + 2 * margin));
    double scale = min(scaleX, scaleY);
    
    // Создаем холст
    vector<string> canvas(consoleHeight, string(consoleWidth, ' '));
    
    // Преобразуем все точки в экранные координаты
    vector<pair<int, int>> screenPoints;
    for (const auto& point : points) {
        int x = static_cast<int>((point.first - minX + width * margin) * scale);
        int y = static_cast<int>((point.second - minY + height * margin) * scale);
        y = consoleHeight - 1 - y;
        screenPoints.emplace_back(x, y);
    }
    
    // Рисуем линии между точками
    for (size_t i = 0; i < screenPoints.size() - 1; ++i) {
        drawLine(canvas, 
                screenPoints[i].first, screenPoints[i].second,
                screenPoints[i+1].first, screenPoints[i+1].second);
    }
    
    // Выводим холст
    for (const string& line : canvas) {
        cout << line << '\n';
    }
    
    return 0;
}