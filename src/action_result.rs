use crate::{utils, ProtoJsonValue};
use tonic::Status;

pub struct ActionResult {
    pub start_time: i64,
    pub end_time: i64,
    pub data: ProtoJsonValue,
}

impl ActionResult {
    pub fn begin() -> Self {
        Self {
            start_time: utils::time_millis(),
            end_time: 0,
            data: ProtoJsonValue { kind: None },
        }
    }

    pub fn end(mut self) -> Result<Self, Status> {
        self.end_time = utils::time_millis();
        Ok(self)
    }

    /// How many time(million seconds) did a workflow cost
    pub fn cost(&self) -> i64 {
        self.end_time - self.start_time
    }
}

impl std::fmt::Debug for ActionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("start_time", &self.start_time)
            .field("end_time", &self.end_time)
            .field("data", &self.data)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ActionResult, Vars};

    #[test]
    fn action_result_begin() {
        let state = ActionResult::begin();
        assert!(state.start_time > 0)
    }

    #[test]
    fn action_result_end() {
        let state = ActionResult::begin();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let result = state.end();
        assert!(result.unwrap().cost() > 0)
    }

    #[test]
    fn action_data_ok() {
        let mut state = ActionResult::begin();

        let mut vars = Vars::new();
        vars.insert("name", &().into());
        state.data = vars.prost_vars();
        assert_eq!(state.data.kind.is_some(), true);
    }
}
