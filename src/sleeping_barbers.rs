use std::thread;
use std::sync::mpsc;
use std::collections::VecDeque;
use rand;
use rand::distributions::{IndependentSample, Range};

struct Barber {
    id: i32,
    to_shop: mpsc::Sender<Message>,
}

struct Customer {
    id: i32,
    hair_length: u32,
}

struct Barbershop {
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
    num_chairs: usize,
    waiting_customers: VecDeque<Customer>,
    free_barbers: VecDeque<Barber>,
}

enum Message {
    BarberFree(Barber),
    CustomerArrives(Customer),
}

impl Barber {
    fn new(id: i32, shop: &Barbershop) -> Barber {
        Barber{
            id: id,
            to_shop: shop.tx.clone(),
        }
    }

    fn cut_hair(self, c: Customer) {
        println!("Barber {:?} is cutting hair of customer {:?}.", self.id, c.id);
        thread::sleep_ms(c.hair_length);
        self.to_shop.clone().send(Message::BarberFree(self)).unwrap();
    }
}

impl Barbershop {
    fn new(num_chairs: usize) -> Barbershop {
        let (tx, rx) = mpsc::channel::<Message>();
        Barbershop{
            tx: tx,
            rx: rx,
            num_chairs: num_chairs,
            waiting_customers: VecDeque::<Customer>::new(),
            free_barbers: VecDeque::<Barber>::new(),
        }
    }

    fn hire_barber(&mut self, id: i32) {
        let b = Barber::new(id, self);
        self.barber_free(b);
    }

    fn serve_customer(&mut self) {
        if self.free_barbers.len() == 0 {
                return;
        }

        if self.waiting_customers.len() == 0 {
                return;
        }

        let b = self.free_barbers.pop_front().unwrap();
        let c = self.waiting_customers.pop_front().unwrap();

        thread::spawn(move || b.cut_hair(c));
    }

    fn barber_free(&mut self, b: Barber) {
        self.free_barbers.push_back(b);
        self.serve_customer();
    }

    fn new_customer(&mut self, c: Customer) {
        if self.waiting_customers.len() < self.num_chairs {
            self.waiting_customers.push_back(c);
            self.serve_customer();
        }
        else {
            println!("Turned away customer {:?}.", c.id);
        }
    }

    fn operate(&mut self) {
        loop {
            let m = self.rx.recv().unwrap();
            match m {
                Message::BarberFree(b) => self.barber_free(b),
                Message::CustomerArrives(c) => self.new_customer(c),
            };
        }
    }
}

pub fn sleeping_barbers() {
    let mut shop = Barbershop::new(5);
    let to_shop = shop.tx.clone();
    let hair_lengths = Range::new(50, 1000);
    let customer_interval = Range::new(50, 500);
    let mut rng = rand::thread_rng();

    shop.hire_barber(1);
    shop.hire_barber(2);

    thread::spawn(move || shop.operate());

    for i in 1..20 {
        let c = Customer{
            id: i,
            hair_length: hair_lengths.ind_sample(&mut rng),
        };
        println!("Customer {:?} arrives at shop.", i);
        to_shop.send(Message::CustomerArrives(c)).unwrap();
        thread::sleep_ms(customer_interval.ind_sample(&mut rng));
    }
    thread::sleep_ms(10000);
}
