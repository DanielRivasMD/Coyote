////////////////////////////////////////////////////////////////////////////////////////////////////

struct Flashcard {
  id: u32,
  ease_factor: f64,
  interval: u32,
  repetitions: u32,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl Flashcard {
  fn new(id: u32) -> Self {
    Flashcard {
      id,
      ease_factor: 2.5, // default ease factor
      interval: 1,      // default interval
      repetitions: 0,   // no repetitions initially
    }
  }

  fn update(&mut self, quality: u32) {
    // ensure quality is in the range [0, 5]
    let quality = quality.min(5).max(0);

    if quality >= 3 {
      if self.repetitions == 0 {
        self.interval = 1;
      } else if self.repetitions == 1 {
        self.interval = 6;
      } else {
        self.interval = (self.interval as f64 * self.ease_factor).round() as u32;
      }
      self.repetitions += 1;
    } else {
      self.repetitions = 0;
      self.interval = 1;
    }

    // update ease factor
    self.ease_factor += 0.1 - (5 - quality) as f64 * (0.08 + (5 - quality) as f64 * 0.02);
    if self.ease_factor < 1.3 {
      self.ease_factor = 1.3;
    }
  }

  fn next_review_in_days(&self) -> u32 {
    self.interval
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// fn main() {
//   let mut flashcard = Flashcard::new(1);

//   // example usage: simulate user responses with different qualities
//   let qualities = vec![5, 4, 3, 2, 5, 5];

//   for (i, &quality) in qualities.iter().enumerate() {
//     println!("Review {}: Quality={}", i + 1, quality);
//     flashcard.update(quality);
//     println!(
//       "Repetitions: {}, Interval: {} days, Ease Factor: {:.2}",
//       flashcard.repetitions, flashcard.interval, flashcard.ease_factor
//     );
//   }
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
