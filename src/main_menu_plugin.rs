use bevy::prelude::*;
use crate::GameState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_main_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), clear_main_menu);
    }
}

fn clear_main_menu(mut commands: Commands, menu: Query<Entity, With<DestructMenu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component, Default)]
struct DestructMenu;

#[derive(Clone, Component)]
struct ButtonColors{
    normal: Color,
    hovered: Color,
    pressed: Color,
    text: Color
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            normal: Color::srgb(0.15, 0.15, 0.15),
            hovered: Color::srgb(0.25, 0.25, 0.25),
            pressed: Color::srgb(0.35, 0.35, 0.35),
            text: Color::srgb(0.9, 0.9, 0.9)
        }
    }
}

#[derive(Component)]
#[require(DestructMenu)]
struct Menu;

#[derive(Component)]
#[require(DestructMenu)]
struct UiCamera;

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((UiCamera, Camera2d, Msaa::Off));

    let button_colors = ButtonColors::default();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Menu
    )).with_children(|parent| {
        parent.spawn(
            (
                Button,
                Node {
                    width: Val::Px(140.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(button_colors.normal),
                button_colors.clone(),
                ChangedState(GameState::Solitaire),
                BorderRadius::all(Val::Px(10.0)),
            )
        )
        .with_child((
            Text::new("Play Solitaire"),
            TextFont {
                font_size: 15.0,
                ..default()
            },
            TextColor(button_colors.text),
        ));
    });
}

#[derive(Component)]
struct ChangedState(GameState);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangedState>
        ),
        (Changed<Interaction>, With<Button>)
    >,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, button_colors, changed_state) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if let Some(ChangedState(new_state)) = changed_state {
                    state.set(new_state.clone())
                }
                *background_color = BackgroundColor(button_colors.pressed);
            }
            Interaction::Hovered => {
                *background_color = BackgroundColor(button_colors.hovered);
            }
            Interaction::None => {
                *background_color = BackgroundColor(button_colors.normal);
            }
        }
    }
}