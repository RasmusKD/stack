struct Stack<T> {
    elements: Vec<T>,
    max_size: usize,
}

impl<T: std::fmt::Debug> Stack<T> {
    fn new(max_size: usize) -> Self {
        Stack {
            elements: Vec::with_capacity(max_size),
            max_size,
        }
    }

    fn push(&mut self, item: T) -> Result<(), &'static str> {
        if self.elements.len() >= self.max_size {
            return Err("Stack overflow: Maksimal størrelse overskredet");
        }
        self.elements.push(item);
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn print_stack(&self) {
        if self.elements.is_empty() {
            println!("Stakken er tom");
        } else {
            println!("Stack indhold (top til bund):");
            self.elements.iter().rev().for_each(|item| println!("{:?}", item));
        }
    }

}

fn main() {
    let mut stack: Stack<i32> = Stack::new(2);

    println!("Er stakken tom? {}", stack.is_empty()); // True (tom)
    println!("Top element: {:?}", stack.peek()); // returnerer None (ingen elementser)
    println!("Pop element: {:?}", stack.pop()); // returnerer None (ingen elementser)

    // Tilføjer elementer
    match stack.push(1) {
        Ok(_) => println!("Pushed 1"),
        Err(e) => eprintln!("Fejl: {}", e),
    }
    match stack.push(2) {
        Ok(_) => println!("Pushed 2"),
        Err(e) => eprintln!("Fejl: {}", e),
    }
    // Forsøger at pushe et tredje element, som nu fejler, da max_size er sat til 2
    match stack.push(3) {
        Ok(_) => println!("Pushed 3"),
        Err(e) => eprintln!("Fejl: {}", e),
    }

    println!("Er stakken tom? {}", stack.is_empty()); // False (ikke tom)
    println!("Top element: {:?}", stack.peek()); // returnerer 2 (øverste)

    println!("Stack fra top til bund:");
    stack.print_stack(); // printer stakken


    println!("Pop element: {:?}", stack.pop()); // fjerner element 2 (øverste)
    println!("Pop element: {:?}", stack.pop()); // fjerner element 1 (øverste)
    println!("Stack fra top til bund:");
    stack.print_stack(); // printer stakken
}
