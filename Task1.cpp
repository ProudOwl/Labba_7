#include <iostream>
#include <vector>
#include <cmath>
#include <SFML/Graphics.hpp>
using namespace std;

void kochCurve(vector<pair<double, double>>& points, 
               double x1, double y1, 
               double x2, double y2, 
               int n) {
    if (n == 0) {
        points.emplace_back(x1, y1);
        return;
    }
    
    double dx = x2 - x1;
    double dy = y2 - y1;
    
    // Точки деления отрезка
    double xA = x1 + dx / 3;
    double yA = y1 + dy / 3;
    double xB = x1 + 2 * dx / 3;
    double yB = y1 + 2 * dy / 3;
    
    double xC = (x1 + x2) / 2 + dy * sqrt(3) / 6;
    double yC = (y1 + y2) / 2 - dx * sqrt(3) / 6;
    
    // Рекурсивное построение
    kochCurve(points, x1, y1, xA, yA, n - 1);
    kochCurve(points, xA, yA, xC, yC, n - 1);
    kochCurve(points, xC, yC, xB, yB, n - 1);
    kochCurve(points, xB, yB, x2, y2, n - 1);
}

int main() {
    int n;
    cout << "Введите глубину рекурсии n: ";
    cin >> n;
    
    // Начальные точки (горизонтальная линия в нижней части экрана)
    double x1 = 200, y1 = 600;
    double x2 = 1000, y2 = 600;
    
    vector<pair<double, double>> points;
    kochCurve(points, x1, y1, x2, y2, n);
    points.emplace_back(x2, y2);
    
    // Создание окна
    sf::RenderWindow window(sf::VideoMode(1200, 800), "Кривая Коха");
    window.setFramerateLimit(60);
    
    // Подготовка линий
    sf::VertexArray lines(sf::LineStrip, points.size());
    for (size_t i = 0; i < points.size(); ++i) {
        lines[i].position = sf::Vector2f(points[i].first, points[i].second);
        lines[i].color = sf::Color::Green;
    }
    
    // Основной цикл
    while (window.isOpen()) {
        sf::Event event;
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed)
                window.close();
        }
        
        window.clear(sf::Color::Black);
        window.draw(lines);
        window.display();
    }
    
    return 0;
}
