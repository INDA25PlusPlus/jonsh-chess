# Schack Bibliotek
Vill du veta hur mon kod fungerar?
Jag med.

## Movement&period;rs
movement&period;rs innehåller alla funktioner som har med förflyttnignen av pjäser att göra.

### move_piece()
move&#95;piece() flyttar pjäserna. Man anger fyra positiva heltal som motsvarar positionen som pjäsen ligger på och positionen som man vill att pjäsen ska till. `move_piece(från x: usize, från y: usize, till x: usize, till y: usize)`
```rust
let new_board = Board::new() //initiera en ny bräda
let (from_x, from_y, to_x, to_y) = (4usize, 6usize, 4usize, 4usize) //e2 - e4
new_board.move_piece(from_x, from_y, to_x, to_y) //Flytta pjäsen
```
Ifall ett ogiltigt drag eller icke existerande pjäs anges så händer ingenting men om ogiltiga koordinater anges t.ex. x = 9 eller y = -1 så krashar får koden panik.

### legal_moves()
legal&#95;moves() returnerar en vektor med tuples av möjliga drag som en pjäs kan göra. och man anger bara två positiva heltal för koordinaterna som pjäsen har t.ex. `legal_moves(x: usize, y: usize)`
```rust
let new_board = Board::new() //initiera en ny bräda
println!("{:?}", new_board.legal_moves(4, 6)) // Lagliga drag för e2
```
Detta exempel returnerar `[(4, 5), (4, 4)]`

### valid_moves()
valid&#95;moves() returnerar samma sak som legal_moves fast även drag som skulle satt kungen i schack. Den är också den stora funktionen som beskriver vilka drag som en viss pjäs kan göra.

### is_check()
is&#95;check() returnerar en tuple av två bools som visar ifall antingen den vita eller svarta kungen är i schack. (false, false/true) ingen är i schack, (true, false) svart är i schack och (true, true) vit är i schack.

### is_checkmate()
is&#95;checkmate() returnerar en tuple av tre bools som visar ifall det antigen är schackmatt för vit eller svart eller patt. (false, false, false) inte schackmatt eller patt, (true, true/false, false) schackmatt för antingen vit eller svart, (false, false, true) patt.


## board&period;rs
Här finns all kod för hur brädet ser ut och hur det initieras.

### new()
Som ni kanske sett på tidigare exempel så är det den här funktionen som skapar ett nytt bräde och placerar ut alla pjäser i deras startposition.

### print_board()
Denna funktion är för testning enbart och skriver ut hur brädan ser ut i terminalen.

### in_bounds()
Denna funktion används överallt i movement&period;rs och den kollar om angivna koordinater ligger innanför brädet.

## Resten
I resten av filerna så finns ingen kod som är nödvändig för spelets funktion utan innehåller bara funktioner som jag använde när jag testade min kod. I min main&period;rs så finns också en väldigt barebones test som man kan spela spelet med.

