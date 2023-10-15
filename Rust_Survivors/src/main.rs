#![allow(non_snake_case)]
use bevy::{ math::vec2, math::vec3, 
            prelude::*, 
            sprite::collide_aabb::*};

//player
const PLAYER_SIZE: Vec2 = Vec2::new(200.0, 200.0);
const PLAYER_SPEED: f32 = 500.0;

//wall
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_player, player_check_collisions))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //spawn camera
    commands.spawn(Camera2dBundle::default());

    //spawn player
    //let p_image_handle = asset_server.load("textures/poe_0.jpg");
    commands.spawn((
        //adds sprite bundle to player entity
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            texture: asset_server.load("textures/poe_0.png"),
            ..default()
        },
        Player, //adds player component to player
        Collider { size: vec2(200.0, 200.0) }
    ));

    //spawn walls
    {
        let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        //spawn LEFT WALL
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            },
        });

        //spawn RIGHT WALL
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(RIGHT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            },
        });

        //spawn BOTTOM WALL
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });

        //spawn TOP WALL
        commands.spawn(WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });
    }
}

fn move_player(
    //gives access to keyboard input Resource from DefaultPlugins
    input: Res<Input<KeyCode>>,
    //gives access to the time Resource from DefaultPlugins
    time_step: Res<FixedTime>,
    //gets every entity with both Transform and Player
    //specifies that we want read/write to Transform data and only read to Player data
    mut query: Query<&mut Transform, With<Player>>,
) {
    //gets transform from query BUT WILL CRASH GAME IF > 1
    let mut player_transform = query.single_mut();

    //X DIRECTION PLAYER MOVEMENT
    //capture player input in the x direction
    let mut x_direction = 0.0;
    if input.pressed(KeyCode::A) {
        x_direction -= 1.0;
    }
    if input.pressed(KeyCode::D) {
        x_direction += 1.0;
    }
    //calculate new player x transform
    let new_x = player_transform.translation.x
        + x_direction * PLAYER_SPEED * time_step.period.as_secs_f32();
    //update player transform to new x position
    player_transform.translation.x = new_x;

    //Y DIRECTION PLAYER MOVEMENT
    //capture player input in the x direction
    let mut y_direction = 0.0;
    if input.pressed(KeyCode::W) {
        y_direction += 1.0;
    }
    if input.pressed(KeyCode::S) {
        y_direction -= 1.0;
    }
    //calculate new player x transform
    let new_y = player_transform.translation.y
        + y_direction * PLAYER_SPEED * time_step.period.as_secs_f32();
    //update player transform to new x position
    player_transform.translation.y = new_y;
}


fn player_check_collisions(
    //read transform and read player components
    mut player_query: Query<(&Transform, &Player, &Collider)>,
    //any entity with both Transform and Collider Components
    collider_query: Query<(&Transform, &Collider), Without<Player>>,
) {
    //gets player tuple (&Transform, &Plyer)
    let (mut player_transform, 
        player_component, 
        mut player_collider) = player_query.single_mut();
    
    //loop through every entity in collider_query
    for (transform, other) in &collider_query {
        
        //if player collides with other, then ___
        match collide(
            player_transform.translation,
            player_collider.size,
            transform.translation,
            other.size,
        ) {
            Some(_) => println!("Collision!"),
            None    => println!("no collision detected"),
        };

    }
}

fn player_lose_health(enemy_transform: &Transform, enemy_collider: &Collider) {
    
}





//----------------- From Game Objects ----------------
#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct Collider {
    size: Vec2,
}

#[derive(Bundle)]
struct WallBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyName(String);

#[derive(Component)]
pub struct Health {
    max_hp: f32,
    current_hp: f32,
    regen_rate: f32,
}
