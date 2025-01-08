use std::time::Duration;
use bevy::ecs::query::QuerySingleError;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::TweenCommand;

pub struct SolitairePlugin;

impl Plugin for SolitairePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(MeshPickingPlugin)
            .add_systems(OnEnter(GameState::Solitaire), setup_solitaire)
            .add_systems(Update, (
                periodic_timer_system,
                (
                    (
                        random_rotation_system,
                        rotation_transformation_system,
                    ).chain()
                ),
            ).chain().run_if(in_state(GameState::Solitaire)));
    }
}

fn periodic_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut periodic_timers: Query<(Entity, &mut PeriodicTimer, &TimerState, Option<&JustFinished>)>,
) {
    for (entity, mut periodic_timer, timer_state, just_finished) in periodic_timers.iter_mut() {
        let mut timer_entity_commands = commands.entity(entity);
        if just_finished.is_some() {
            timer_entity_commands.remove::<JustFinished>();
        }

        let tick_timer = |periodic_timer: &mut PeriodicTimer, timer_entity_commands: &mut EntityCommands| {
            periodic_timer.timer.tick(time.delta());
            if periodic_timer.timer.just_finished() {
                timer_entity_commands.insert(JustFinished {
                    finishes: periodic_timer.timer.times_finished_this_tick()
                });
            }
        };

        match timer_state {
            TimerState::Started => {
                periodic_timer.timer.reset();
                timer_entity_commands.replace(TimerState::Running);
                tick_timer(&mut periodic_timer, &mut timer_entity_commands);
            }
            TimerState::Running => {
                tick_timer(&mut periodic_timer, &mut timer_entity_commands);
            }
            TimerState::Paused => {}
        }
    }
}

fn rotation_transformation_system(
    mut transforms: Query<(&mut Transform, &Rotation, &mut MeshMaterial2d<ColorMaterial>), Changed<Rotation>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let up_rotation = Quat::from_rotation_z(0.0);
    let left_rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.0);
    let right_rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.0);
    let down_rotation = Quat::from_rotation_z(std::f32::consts::PI);

    for (mut transform, rotation, color_material) in transforms.iter_mut() {
        transform.rotation = match rotation {
            Rotation::Up => up_rotation,
            Rotation::Left => left_rotation,
            Rotation::Right => right_rotation,
            Rotation::Down => down_rotation,
        };

        let color_material = color_materials.get_mut(color_material.0.id()).unwrap();
        color_material.color = match rotation {
            Rotation::Up => Color::srgb(0.2, 0.2, 0.5),
            Rotation::Left => Color::srgb(0.2, 0.5, 0.2),
            Rotation::Right => Color::srgb(0.5, 0.2, 0.2),
            Rotation::Down => Color::srgb(0.5, 0.2, 0.5),
        };
    }
}

#[derive(Component)]
#[require(Transform)]
struct Deck;

#[derive(Component)]
struct JustFinished {
    finishes: u32,
}

#[derive(Component)]
#[require(PeriodicTimer (|| PeriodicTimer::new(Duration::from_secs(1))))]
struct RotationTimer;

#[derive(Component)]
#[require(TimerState)]
struct PeriodicTimer {
    timer: Timer,
}

impl PeriodicTimer {
    fn new(duration: Duration) -> Self {
        Self {
            timer: Timer::new(duration, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Default)]
enum TimerState {
    #[default]
    Started,
    Running,
    Paused,
}



fn random_rotation_system(
    mut commands: Commands,
    finished_rotation_timer: Query<&JustFinished, (Added<JustFinished>, With<RotationTimer>)>,
    decks_query: Query<(Entity, &Children), With<Deck>>,
    cards_query: Query<(Entity, &Rotation), With<Card>>,
) {
    let timer_finished = finished_rotation_timer.get_single();

    match timer_finished {
        Err(QuerySingleError::NoEntities(_)) => {}
        Err(QuerySingleError::MultipleEntities(_)) => {
            panic!("Multiple rotation timers found. Only one is expected.");
        }
        Ok(..) => {
            for (_, children) in decks_query.iter() {
                    children.iter()
                        .flat_map(|card_entity| cards_query.get(*card_entity))
                        .for_each(|(card_entity, _)| {
                            let rotation = match rand::random::<u8>() % 4 {
                                0 => Rotation::Up,
                                1 => Rotation::Left,
                                2 => Rotation::Right,
                                3 => Rotation::Down,
                                _ => unreachable!(),
                            };
                            commands.entity(card_entity).rotate_to(rotation);
                        });
                }
        },
    }
}

#[derive(Component, Clone)]
#[require(Transform, Rotation)]
struct Card;

#[derive(Component, Default)]
enum Rotation {
    #[default]
    Up,
    Left,
    Right,
    Down,
}

fn setup_solitaire(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn(RotationTimer);
    commands.spawn(Deck)
        .with_children(|deck_builder| {
            for i in 0..4 {
                deck_builder.spawn((
                    Card,
                    Mesh2d(meshes.add(Cuboid::new(100.0, 180.0, 1.0))),
                    MeshMaterial2d(materials.add(ColorMaterial {
                        color: Color::srgb(0.2, 0.2, 0.5),
                        ..default()
                    })),
                    Transform{
                        translation: Vec3::new(0.0, 0.0, i as f32),
                        ..default()
                    },
                )).make_draggable()
                    .with_child(
                        (
                            Text2d::new(format!("Card {}", i)),
                            TextLayout::new_with_justify(JustifyText::Center),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            Transform {
                                translation: Vec3::new(0.0, 0.0, 1.0),
                                ..default()
                            }
                        ));
            }
        });
}

trait MakeDraggable {

    fn make_draggable(&mut self) -> &mut Self;
}

trait Replace<T: Component>
{
    fn replace(&mut self, component: T) -> &mut Self;
}

trait RotateTo {
    fn rotate_to(&mut self, rotation: Rotation) -> &mut Self;
}

impl<T: Component> Replace<T> for EntityCommands<'_> {
    fn replace(&mut self, component: T) -> &mut Self {
        self.remove::<T>().insert(component)
    }
}

impl RotateTo for EntityCommands<'_> {
    fn rotate_to(&mut self, rotation: Rotation) -> &mut Self {
        self.replace(rotation)
    }
}


impl MakeDraggable for EntityCommands<'_> {
    fn make_draggable(&mut self) -> &mut Self {
        self.observe(|mouse_drag: Trigger<Pointer<Drag>>, mut draggables: Query<&mut Transform>, mut commands: Commands| {
            commands.entity(mouse_drag.entity()).insert(Dragged);
            let mut transform = draggables.get_mut(mouse_drag.entity()).unwrap();
            transform.translation.x += mouse_drag.event().delta.x;
            transform.translation.y -= mouse_drag.event().delta.y;
        });
        
        self.observe(|mouse_drag: Trigger<Pointer<DragEnd>>, mut draggables: Query<&mut Text>, mut commands: Commands| {
            commands.entity(mouse_drag.entity()).remove::<Dragged>();
        })
    }
}

#[derive(Component)]
struct Dragged;