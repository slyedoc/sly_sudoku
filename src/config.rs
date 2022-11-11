use bevy::{math::vec2, prelude::*};

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SudokuConfig>();
    }
}

// Users editable Settings

// Config generated from Settings
pub struct SudokuConfig {
    pub cell_size: f32,
    pub cell_padding: f32,
    pub line_size: f32,
    pub bold_line_size: f32,

    pub cell_mat: Handle<StandardMaterial>,
    pub line_mat: Handle<StandardMaterial>,
    pub bold_line_mat: Handle<StandardMaterial>,

    pub board_size: f32,
    pub board_half_size: f32,
    pub cell_mesh: Handle<Mesh>,
    pub line_mesh: Handle<Mesh>,
    pub bold_line_mesh: Handle<Mesh>,
    //pub cell_collider: Handle<Collider>,
}

impl FromWorld for SudokuConfig {
    fn from_world(world: &mut World) -> Self {
        
        let cell_size = 1.0;
        let cell_padding = 0.1;
        let line_size = 0.05;
        let bold_line_size = 0.1;

        let board_size = (cell_size * 9.0) + (cell_padding * 18.0);
        let board_half_size = board_size * 0.5;
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        let cell_mesh = meshes.add(Mesh::from(shape::Quad {
            size: vec2(cell_size, cell_size),
            flip: false,
        }));

        let line_mesh = meshes.add(Mesh::from(shape::Quad {
            size: vec2(board_size, line_size),
            flip: false,
        }));

        let bold_line_mesh = meshes.add(Mesh::from(shape::Quad {
            size: vec2(board_size + bold_line_size, bold_line_size),
            flip: false,
        }));

        let mut materials = world.resource_mut::<Assets<StandardMaterial>>();
        let cell_mat = materials.add(StandardMaterial {
            base_color: Color::RED,
            unlit: true,
            ..default()
        });

        let line_mat = materials.add(StandardMaterial {
            base_color:  Color::GRAY,
            unlit: true,
            ..default()
        });

        let bold_line_mat = materials.add(StandardMaterial {
            base_color: Color::BLACK,
            unlit: true,
            ..default()
        });

        Self {
            board_size,
            board_half_size,
            cell_size,
            cell_padding,
            line_size,
            bold_line_size,
            cell_mesh,
            line_mesh,
            bold_line_mesh,
            cell_mat,
            line_mat,
            bold_line_mat,
        }
}
}