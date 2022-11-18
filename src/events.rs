use crate::{board::{Cell, CellPosition}, SudokuContainer};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use sudoku_variants::{constraint::*, generator::*, solver::*, *};

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Reset>()
            .add_event::<NewGame>()
            .add_event::<NewGameReady>()
            .add_event::<Solve>()
            .add_system(reset_events)
            .add_system(new_game_events)
            .add_system(solve_events)
            .add_system(handle_new_game_task);
    }
}

// Events
pub struct NewGame;
pub struct NewGameReady;
pub struct Reset;
pub struct Solve;

#[derive(Resource)]
pub struct NewGameTask(Task<Sudoku<DefaultConstraint>>);

fn new_game_events(
    mut commands: Commands, 
    mut new_game_event: EventReader<NewGame>,

) {
    for _ in new_game_event.iter() {
        let thread_pool = AsyncComputeTaskPool::get();
        let task = thread_pool.spawn(async move {
            let mut generator = Generator::new_default();
            let mut sudoku = generator.generate(3, 3, DefaultConstraint).unwrap();
            let mut reducer = Reducer::new_default();
            reducer.reduce(&mut sudoku);
            sudoku
        });

        commands.insert_resource(NewGameTask(task));
    }
}

fn handle_new_game_task(
    mut commands: Commands, 
    task: Option<ResMut<NewGameTask>>,
    mut new_game_ready_event: EventWriter<NewGameReady>,
) {
    if let Some(mut task) = task {
        if let Some(sudoku) = future::block_on(future::poll_once(&mut task.0)) {
            commands.insert_resource(SudokuContainer(sudoku));
            commands.remove_resource::<NewGameTask>();
            new_game_ready_event.send(NewGameReady);
        }
    }    
}

fn reset_events(mut reset_event: EventReader<Reset>, mut query: Query<&mut Cell>) {
    for _ in reset_event.iter() {
        for mut cell in query.iter_mut() {
            cell.reset();
        }
    }
}

fn solve_events(
    mut solve_events: EventReader<Solve>,
    mut query: Query<(&mut Cell, &CellPosition)>,
) {
    for _ in solve_events.iter() {
        let mut sudoku = Sudoku::new_empty(3, 3, DefaultConstraint).unwrap();
        let grid = sudoku.grid_mut();
        for (cell, cell_pos) in query.iter_mut() {
            if let Some(value) = cell.value() {
                grid.set_cell(cell_pos.x, cell_pos.y, value.into()).unwrap();
            }
        }
        if !sudoku.is_valid() {
            error!("Sudoku is not valid");
            return;
        }

        let solver = BacktrackingSolver;
        match solver.solve(&sudoku) {
            Solution::Impossible => {
                error!("Sudoku is Ambiguous");
            }
            Solution::Unique(solution) => {
                for (mut cell, cell_pos) in query.iter_mut() {
                    if let Ok(grid_cell) = solution.get_cell(cell_pos.x, cell_pos.y) {
                        cell.set_value(grid_cell.unwrap().into());
                    }
                }
            }
            Solution::Ambiguous => {
                error!("Sudoku is Ambiguous");
            }
        }
    }
}
