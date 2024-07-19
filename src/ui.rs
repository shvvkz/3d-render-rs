use bevy::prelude::*;
use bevy::ui::Val::{Percent, Px};
use crate::cube::CubeProperties;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, handle_color_change)
            .add_systems(Update, handle_border_radius_change);
    }
}

#[derive(Component)]
struct ColorInput;

#[derive(Component)]
struct BorderRadiusSlider;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    // Root node
    commands.spawn(NodeBundle {
        style: Style {
            width: Px(300.0),
            height: Px(200.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        // Color input
        parent.spawn(NodeBundle {
            style: Style {
                width: Percent(100.0),
                height: Px(30.0),
                ..default()
            },
            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Cube Color:",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    }
                ),
                ..default()
            });
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "#FF0000", // Default value
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    }
                ),
                ..default()
            })
            .insert(ColorInput);
        });

        // Border radius slider
        parent.spawn(NodeBundle {
            style: Style {
                width: Percent(100.0),
                height: Px(30.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            background_color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Border Radius:",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    }
                ),
                ..default()
            });
            parent.spawn(NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                ..default()
            })
            .insert(BorderRadiusSlider);
        });
    });
}

fn handle_color_change(
    mut query: Query<&mut Text, With<ColorInput>>,
    mut cube_props: ResMut<CubeProperties>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    query.for_each_mut(|text| {
        println!("{:?}", text);
        if let Ok(color) = Color::hex(&text.sections[0].value) {
            cube_props.color = color;
            println!("Updated cube color to: {:?}", color);
        }
    });
}

fn handle_border_radius_change(
    mut query: Query<&mut Transform, With<BorderRadiusSlider>>,
    mut cube_props: ResMut<CubeProperties>
) {
    for mut transform in query.iter_mut() {
        let border_radius = 0.5; // Placeholder for the actual slider value
        transform.scale = Vec3::splat(border_radius);
        cube_props.border_radius = border_radius;
        println!("Updated border radius to: {:?}", border_radius);
    }
}
