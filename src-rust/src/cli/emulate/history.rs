/// Bash-like command history
pub struct History {
    /// Contains the command history from most recent to least recent
    ///
    /// The first element is reserved for storing the current user input before accessing the history
    history: Vec<String>,
    /// Current cursor position of user inside history
    current_pos: usize,
    /// Max amount of history entries
    max_size: usize,
}

impl History {
    /// Create a new history with the specified max size
    pub fn new(max_size: usize) -> Self {
        Self {
            history: vec![],
            current_pos: 0,
            max_size,
        }
    }

    pub fn get_next(&mut self) -> Option<&str> {
        self.current_pos += 1;

        if self.current_pos >= self.history.len() {
            None
        } else {
            let res = Some(self.history[self.current_pos].as_str());

            res
        }
    }

    pub fn get_prev(&mut self) -> Option<&str> {
        if self.current_pos == 0 {
            None
        } else {
            self.current_pos -= 1;

            Some(self.history[self.current_pos].as_str())
        }
    }

    pub fn reset(&mut self) {
        self.current_pos = 0
    }

    /// Create a new history entry
    ///
    /// If the history contains already `max_size` entries the oldest entry is discarded
    pub fn push(&mut self, entry: &str) {
        if self.history.len() == self.max_size {
            self.history.truncate(self.max_size - 1);
        }

        self.history.push(entry.to_owned());
    }

    /// Stores the current user input as first entry in the history
    ///
    /// This should be called each time the user starts browsing the history
    ///
    /// # Panics
    /// If called while the user is currently browsing the history eg. [reset()] has not been called
    pub fn cache_user_input(&mut self, input: &str) {
        assert_eq!(self.current_pos, 0, "Called cache_user_input while user was still browisng the history. Call reset() on the history before caching new user input.");

        self.history[0] = input.to_owned();
    }
}
