
trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    amount_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.amount_left -= 1;
    }
}


fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    fn bunny_nibbles<T: Bite>(item: &mut T) {
        item.bite();
    }
    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut carrot);
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}