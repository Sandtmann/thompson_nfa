# **Projektarbeit: Thompson-NFA in Rust**

Umsetzung eines Thompson-NFA (nondeterministic finite automaton) in Rust durch Matthias Handtmann basierend auf den 
[Implementierungsaufgaben in C++](https://sulzmann.github.io/SoftwareProjekt/labor.html#(10)) 
von Prof. Dr. Martin Sulzmann.

## **Reguläre Ausdrücke**

Ob Rust als objektorientiert zu bezeichnen ist, hängt von der ausgewählten Definition ab. Ein von bekannten 
objektorientierten Sprachen kommender Entwickler wird auf jeden Fall bei der Nutzung von Rust auf einige gewohnte 
Eigenschaften verzichten müssen. Im Gegensatz zu C++ gibt es in Rust keine Klassen, was dem allerdings nahe kommt, sind 
sogenannte *Structs*. Diese bieten jedoch keine Vererbung, die ohne spezielle Makros ohnehin nicht in Rust vorhanden ist.
Mit sogenannten *Traits* ist zwar Polymorphismus möglich aber damit C++ Beispiele eins zu eins nachzustellen ist nicht zu empfehlen, 
auch weil Subtyping in Rust etwas anders ist. Um unterschiedlich reguläre Ausdrücke darzustellen, bieten sich in Rust *Enums* 
an, diese können auch eigene Felder haben und mit dem *match* Konstrukt lässt sich einfach über alle Fälle einer *Enums* iterieren. 
```rust
pub enum REType {
    Phi {},
    Eps {},
    Char {val: char},
    Alt {left: Box<REType>, right: Box<REType>},
    Conc {left: Box<REType>, right: Box<REType>},
    Star {val: Box<REType>}
}
```
Mit diesem Enum und einer passenden Implementation lassen sich die sechs uns vorgegebenen regulären Ausdrücke ganz 
einfach darstellen.  
*Phi* ist die leere Sprache.  
*Eps* ist das leere Wort.  
*Char* ist ein Zeichen.  
*Alt* ist die Alternative aus zwei Objekten.  
*Conc* ist Verkettung zweier Objekte.  
*Star* ist die Kleenesche Hülle.  

Die Implementation der zugehörigen Funktionen nutzt Operationen die wir auch aus der C++ Welt kennen, so hat Rust 
natürlich auch diverse Möglichkeiten Strings zu bearbeiten oder *If, else* Ausdrücke um Verzweigungen zu erstellen. 
Wichtig für das Iterieren ist das oben genannte *Match* Konstrukt und zusätzlich dazu kommt auch noch *mem::discriminant* 
zum Einsatz, was uns *Enum-Varianten* vergleichen lässt unabhängig von ihrem Inhalt.

```rust
match self {
    REType::Phi {} => {}
    REType::Eps {} => {}
    REType::Char { val } => {}
    REType::Alt { left, right } => {}
    REType::Conc { left, right } => {}
    REType::Star { val } => {}
}
```

## **Transformation Regulärer Ausdrücke in Automaten**

Nach der Umsetzung der regulären Ausrücke ging es darum, diese in einen [Thompson-Automaten](https://en.wikipedia.org/wiki/Thompson%27s_construction) 
umzuwandeln. Dafür kamen dieses Mal die schon erwähnten *Structs* zum Einsatz.

```rust
pub struct Transition {
    from:   i32,
    to:     i32,
    char:   char,
}

pub struct NFA {
    transitions:    Vec<Transition>,
    init_state:           i32,
    final_state:         i32,
}

struct TransformWorker {
    name_supply: i32,
}
```

Die zugehörigen Funktionen werden erst im Implementationsteil geschrieben. Zu beachten ist hier, dass Rust kein *Overloading* 
kennt, es müssen also unterschiedliche Fälle in einer Funktion abgearbeitet, oder eine neue Funktion mit anderer Bezeichnung
geschrieben werden.  
Es gibt für *Structs* im Übrigen keinen dedizierten Konstruktor, allerdings ist es üblich eine dafür assoziierte Funktion 
mit *new* zu benennen.

```rust
impl NFA {
    pub fn new(regular_expression: REType) -> Self {
        let transform_worker: TransformWorker = TransformWorker { name_supply: 0 };
        let nfa: Box<NFA> = transform_worker.transform(&regular_expression);
        Self { transitions: nfa.get_transitions(), init_state: nfa.init_state, final_state: nfa.final_state }
    }
}
```
## **Ausführung, Testen & Fazit**

Der letzte Teil der Projektarbeit behandelte noch die Ausführung des NFA mithilfe einer Zustandsmaschine.
Diese ließ sich relativ einfach implementieren. Wir übergeben die erzeugte NFA und geben dann noch ein Wort mit. Die 
Zustandsmaschine iteriert dann einfach über das Wort und versucht es in der NFA abzulaufen under der Berücksichtigung der 
möglichen Epsilon-Übergänge.

Zum Testen sind zwei Ausdrücke für NFAs vorhanden. Zum einen das Beispiel aus den Implementierungsaufgaben zum anderen
der auf Wikipedia abgebildete Automat, der die Vielfachen von drei binär darstellt.

Zu Rust ist zu sagen, dass es wirklich eine interessante, wenn auch ungewöhnliche Sprache ist. Um so mehr man 
sich einließt, um so größer wird das Verständnis warum bestimmte Designentscheidungen getroffen wurden. Allerdings ist es 
doch oft erst verstörend, dass bekannte alt gediente Methoden nicht anwendbar sind. Man muss auch der Rust-Community großen
Respekt zollen, da sie nicht nur sehr aktiv ist, sondern auch für eine große und gute Dokumentation gesorgt hat. Der wohl 
angenehmste Unterschied zu C++ ist das automatische Memory Management, dass einem viele Sorgen nimmt, solange man noch nicht 
bei [**unsafe Rust**](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) gelandet ist. 