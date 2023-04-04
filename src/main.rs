use grid::*; // Provides a 2d matrix data structure with most of Vec features
use std::{sync::{Arc, Mutex, MutexGuard}, thread}; // Provides Atomically Reference Counted and Mutual Exclusio for concurrency
use rand::{distributions::Alphanumeric, Rng}; // Random string and number generators

/*
 * Ticket structure,
 * number and series will match matrix index structure
 * but having the data in the struct make the object portable
 * and easy to extract if necessary
 */
#[derive(Clone)]
struct Ticket {
  number: u8,
  series: u16,
  owner: String,
  sold: bool
}

impl Default for Ticket {
  /*
   * Default values for number and series are one
   * over the maximum of 99 and 999.
   *
   * WIP - TODO - Find a better way for this
   */
  fn default() -> Self {
    Ticket { number: 100, series: 1000, owner: "none".to_owned(), sold: false }
  }
}

/*
 * Generates a 2d matrix grid of tickets,
 * since the grid crate works "row centric"
 * we do 1000 rows with 100 columns
 */
fn create_raffle() -> Grid<Ticket>{
  let mut raffle = grid![];

  for s_index in 1..=1000 { // 1000 rows
    let mut row = vec![Ticket {series: s_index - 1, ..Default::default()}; 100]; // A vector of 100 tickets

    for (index, ticket) in row.iter_mut().enumerate() {
      ticket.number = index as u8; // Initializes the thicket's number
    }

    raffle.push_row(row);
  }

  raffle // return the matrix
}

fn main() {
  let raffle_tikets = create_raffle(); // Create the ticket matrix

  /**
   * Using the Atomicity and Mutual exlusion this
   * function is ready for multi-threads
   */
  fn buy_ticket(raffle: &Arc<Mutex<Grid<Ticket>>>, number: u8, owner: String) {
    let mut raffle_ref: MutexGuard<Grid<Ticket>> = raffle.lock().unwrap();
    let mut sold = false; // Keep false if we don't find a free series for the requested number

    /*
     * From lowest and up find a free series for the requested number
     * (yes, there has to be a better and more just way to pick series)
     */
    for s_index in 1..=1000 {
      let ticket = &mut raffle_ref[s_index][number as usize];
      if ticket.sold == false {
        ticket.sold = true;
        ticket.owner = owner.clone();

        let Ticket {number, series, owner, .. } = ticket;
        println!("Sold number {:02}, with series {:03} to {}", number, series, owner);
        sold = true; // If we found a number, break and set this to true
        break;
      }
    }

    if !sold {
      println!("No tickets with number {} available for costumer {}", number, owner);
    }
  }

  /* Create today's raffle with Atomicity and Mutual Exlcusion */
  let raffle: Arc<Mutex<Grid<Ticket>>> = Arc::new(Mutex::new(raffle_tikets));

  /*
   * Each handle will trigger a new thread and they will all be collected
   * later for thread conclusion control
   */
  let handles = (0..1500).map(|_| {
    let _owner: String = rand::thread_rng().sample_iter(&Alphanumeric).take(9).map(char::from)
    .collect();

    let _number: u8 = rand::thread_rng().gen_range(0..99);

    let raffle_ref = raffle.clone();

    thread::spawn(move || {
      buy_ticket(&raffle_ref, _number, _owner);
    })
  });

  /*
   * Collects the handles and continues when they are all done
   */
  for handle in handles {
    handle.join().unwrap();
  }
}
