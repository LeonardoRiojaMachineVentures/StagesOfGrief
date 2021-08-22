enum Stage {
	Denial,
	Anger,
	Bargaining,
	Depression,
	Acceptance,
}

impl Stage {
	fn after(&self) -> Stage {
		use crate::Stage::*;
		match self {
			Denial => Anger,
			Anger => Bargaining,
			Bargaining => Depression,
			Depression => Acceptance,
			Acceptance => Denial,
		}
	}
}

enum Problem {
	Avoidance,
	Confussion,
	Elation,
	Shock,
	Fear,
	Frustation,
	Irritation,
	Anxiety,
	StrugglingToFindMeaning,
	ReachingOutToOthers,
	TellingOnesStory,
	Overwhelmed,
	Helplessness,
	Hostility,
	Flight,
	ExploringOptions,
	NewPlanInPlace,
	MovingOn,
}


impl Stage {
	fn problems(&self) -> Vec<Problem> {
		match self {
			Denial => DENIAL_PROBLEMS,
			Anger => ANGER_PROBLEMS,
			Bargaining => BARGAINING_PROBLEMS,
			Depression => DEPRESSION_PROBLEMS,
			Acceptance => ACCEPTANCE_PROBLEMS,
		}
	}
}

const DENIAL_PROBLEMS = vec![Problem::Avoidance, Problem::Confusion, Problem::Elation, 
Problem::Shock, Problem::Fear];

const ANGER_PROBLEMS = vec![Problem::Frustation, Problem::Irritation, Problem::Anxiety];

const BARGAINING_PROBLEMS = vec![Problem::StrugglingToFindMeaning, Problem::ReachingOutToOthers, Problem::TellingOnesStory];

const DEPRESSION_PROBLEMS = vec![Problem::Overwhelmed, Problem::Helplessness, Problem::Hostility, Problem::Flight];

const ACCEPTANCE_PROBLEMS = vec![Problem::ExloringOptions, Problem::NewPlanInPlace, Problem::MovingOn];

const KUBLER_ROSS_GRIEF_CYCLE = vec![Stage::Denial, Stage::Anger, Stage::Bargaining, Stage::Depression, Stage::Acceptance];

use std::collections::HashMap;

let problem_to_stage: HashMap<Problem, Stage> = [
(Problem::Avoidance, Stage::Denial),
(Problem::Confussion, Stage::Denial),
(Problem::Elation, Stage::Denial),
(Problem::Shock, Stage::Denial),
(Problem::Fear, Stage::Denial),
(Problem::Frustation, Stage::Anger),
(Problem::Irritation, Stage::Anger),
(Problem::Anxiety, Stage::Anger),
(Problem::StrugglingToFindMeaning, Stage::Bargaining),
(Problem::ReachingOutToOthers, Stage::Bargaining),
(Problem::TellingOnesStory, Stage::Bargaining),
(Problem::Overwhelmed, Stage::Depression),
(Problem::Helplessness, Stage::Depression),
(Problem::Hostility, Stage::Depression),
(Problem::Flight, Stage::Depression),
(Problem::ExploringOptions, Stage::Acceptance),
(Problem::NewPlanInPlace, Stage::Acceptance),
(Problem::MovingOn, Stage::Acceptance)].iter().cloned().collect();


fn main() {
	use crate::Stage::*;
	let s = Denial;
	println!("{:?}", s.after());
	//set Stage
	//: Stage is finite
	//set K Combinatorics(Stage)
	//let KUBLER_ROSS_GRIEF_CYCLE K
	//|K| != 1
	//
	println!("{:?}", s.problems());
	println!(problem_to_stage.get(Problem::Avoidance));
}





