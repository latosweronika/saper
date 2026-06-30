# Saper (Minesweeper)

Implementacja gry Saper napisana w języku Rust z wykorzystaniem biblioteki egui/eframe.

### Funkcjonalności 

- trzy rozmiary planszy: 8x8, 10x10, 14x14
- losowe rozmieszczenie min stanowiących 15% pól
- odkrywanie pól
- automatyczne odkrywanie pustych obszarów o warości 0
- oznaczanie pól flagą prawym klawiszem
- wykrywanie wygranej
- zakończenie gry po trafieniu na minę

### Technologie

Rust, egui, eframe, rand

### Uruchomienie

Uruchomienienie aplikacji z poziomu sapper_app/src poprzez:
```
cargo run
```

### Sterowanie

- Lewy przycisk myszy - odkrycie pola
- Prawy przycisk myszy - dodanie lub usunięcie oflagowania
- Nowa gra / Restart - rozpoczęcie nowej rozrywki

### Zasady gry

Celem gry jest odkrycie wszyskich pól, które nie zawierają min. Liczba wyświetlana na odkrytym polu oznacza liczą min znajgujących się na sąsiednich polach (8 dookoła). Trafienie na minę kończy grę i odkrywa całą planszę.

