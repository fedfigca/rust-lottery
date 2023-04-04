use grid::*;
use std::{sync::{Arc, Mutex, MutexGuard}, thread};
use rand::{distributions::Alphanumeric, Rng};

#[derive(Clone)]
struct Ticket {
  number: u8,
  series: u16,
  owner: String,
  sold: bool
}

impl Default for Ticket {
  fn default() -> Self {
    Ticket { number: 100, series: 1000, owner: "none".to_owned(), sold: false }
  }
}

fn create_raffle() -> Grid<Ticket>{
  let mut raffle = grid![];

  for s_index in 1..=1000 {
    let mut row = vec![Ticket {series: s_index - 1, ..Default::default()}; 100];

    for (index, ticket) in row.iter_mut().enumerate() {
      ticket.number = index as u8;
    }

    raffle.push_row(row);
  }

  raffle
}

fn main() {
  let raffle_tikets = create_raffle();

  fn buy_ticket(raffle: &Arc<Mutex<Grid<Ticket>>>, number: u8, owner: String) {
    let mut raffle_ref: MutexGuard<Grid<Ticket>> = raffle.lock().unwrap();
    let mut sold = false;

    for s_index in 1..=1000 {
      let ticket = &mut raffle_ref[s_index][number as usize];
      if ticket.sold == false {
        ticket.sold = true;
        ticket.owner = owner.clone();

        let Ticket {number, series, owner, .. } = ticket;
        println!("Sold number {:02}, with series {:03} to {}", number, series, owner);
        sold = true;
        break;
      }
    }

    if !sold {
      println!("No tickets with number {} available for costumer {}", number, owner);
    }
  }

  let raffle: Arc<Mutex<Grid<Ticket>>> = Arc::new(Mutex::new(raffle_tikets));

  let handles = (0..1500).map(|_| {
    let _owner: String = rand::thread_rng().sample_iter(&Alphanumeric).take(9).map(char::from)
    .collect();

    let _number: u8 = rand::thread_rng().gen_range(0..99);

    let raffle_ref = raffle.clone();

    thread::spawn(move || {
      buy_ticket(&raffle_ref, _number, _owner);
    })
  });

  for handle in handles {
    handle.join().unwrap();
  }
}
