use std::collections::{BTreeSet, HashMap};

use crate::{ast::App, Settings};

pub fn app(app: &mut App, settings: &Settings) {
    // "compress" priorities
    // if the user specified, for example, task priorities of "1, 3, 6"; we'll compress them into
    // "1, 2, 3" as to leave no gaps
    if settings.optimize_priorities {
        // all task priorities ordered in ascending order
        let priorities = app
            .hardware_tasks
            .values()
            .filter_map(|task| Some(task.args.priority))
            .chain(
                app.software_tasks
                    .values()
                    .filter_map(|task| Some(task.args.priority)),
            )
            .collect::<BTreeSet<_>>();

        let map = priorities
            .iter()
            .cloned()
            .zip(1..)
            .collect::<HashMap<_, _>>();

        for task in app.hardware_tasks.values_mut() {
            task.args.priority = map[&task.args.priority];
        }

        for task in app.software_tasks.values_mut() {
            task.args.priority = map[&task.args.priority];
        }
    }
}
