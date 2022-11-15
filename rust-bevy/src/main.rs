use bevy::prelude::*;
use rand::prelude::random;
use bevy::core::FixedTimestep;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "snake".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        // Arena
        .add_startup_system(setup_camera)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(adapt_to_resize),
        )
        // Snake
        .insert_resource(SnakeBody::default())
        .insert_resource(SnakeTail::default())
        .add_startup_system(spawn_snake)
        .add_system(
            get_snake_input
                .label(SnakeMovement::Input)
                .before(SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.1))
                .with_system(move_snake.label(SnakeMovement::Movement))
                .with_system(
                    snake_eating
                        .label(SnakeMovement::Eating)
                        .after(SnakeMovement::Movement),
                )
                .with_system(
                    snake_growth
                        .label(SnakeMovement::Growth)
                        .after(SnakeMovement::Eating),
                )
        )
        // Food
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(spawn_food),
        )
        .add_event::<GrowthEvent>()
        // Game management
        .add_event::<GameOverEvent>()
        .add_system(game_over.after(SnakeMovement::Movement))
        .run();
}

// PHYSICS

#[derive(Component,Clone,Copy,PartialEq,Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(PartialEq,Copy,Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

// ARENA

const ARENA_SIZE: u32 = 20;

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn adapt_to_resize(
    windows: Res<Windows>,
    mut q: Query<(&Size, &Position, &mut Transform)>
) {
    let window = windows.get_primary().unwrap();
    let tile_size = fmin(
        window.width() as f32 / ARENA_SIZE as f32,
        window.height() as f32 / ARENA_SIZE as f32,
    );
    for (size, pos, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            size.width as f32 * tile_size,
            size.height as f32 * tile_size,
            1.0,
        );
        transform.translation = Vec3::new(
            (pos.x as f32 + 0.5 - ARENA_SIZE as f32 / 2.) * tile_size,
            (pos.y as f32 + 0.5 - ARENA_SIZE as f32 / 2.) * tile_size,
            0.0,
        );
    }
}

fn fmin(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

// FOOD

#[derive(Component)]
struct Food;

struct GrowthEvent;

const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

fn spawn_food(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: FOOD_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Food)
        .insert(Position {
            x: (random::<f32>() * ARENA_SIZE as f32) as i32,
            y: (random::<f32>() * ARENA_SIZE as f32) as i32,
        })
        .insert(Size::square(0.8));
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

// SNAKE

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const SNAKE_SEGMENT_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);

#[derive(Component)]
struct SnakeHead {
    direction: Direction,
}

#[derive(Component)]
struct SnakePart;

#[derive(Default)]
struct SnakeBody(Vec<Entity>);

#[derive(Default)]
struct SnakeTail(Option<Position>);

fn spawn_snake(
    mut commands: Commands,
    mut segments: ResMut<SnakeBody>
) {
    segments.0 = vec![
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakePart)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_snake_part(commands, Position { x: 3, y: 2 }),
    ];
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

fn get_snake_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut SnakeHead>,
) {
    // Why don't we loop through the snakes query here? Cos we know at most
    // one snake head exists? Here we would edit to process multiple snakes.
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn move_snake(
    body: ResMut<SnakeBody>,
    mut tail: ResMut<SnakeTail>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let body_positions = body
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_SIZE
            || head_pos.y as u32 >= ARENA_SIZE
            || body_positions.contains(&head_pos)
        {
            game_over_writer.send(GameOverEvent);
        }
        body_positions
            .iter()
            .zip(body.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });
        tail.0 = Some(*body_positions.last().unwrap());
    }
}

fn spawn_snake_part(mut commands: Commands, position: Position) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_SEGMENT_COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(SnakePart)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}

fn snake_growth(
    commands: Commands,
    tail: Res<SnakeTail>,
    mut segments: ResMut<SnakeBody>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    if growth_reader.iter().next().is_some() {
        segments
            .0
            .push(spawn_snake_part(commands, tail.0.unwrap()));
    }
}

// GAME MANAGEMENT

struct GameOverEvent;

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    body: ResMut<SnakeBody>,
    food: Query<Entity, With<Food>>,
    parts: Query<Entity, With<SnakePart>>,
) {
    if reader.iter().next().is_some() {
        for ent in food.iter().chain(parts.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, body);
    }
}
