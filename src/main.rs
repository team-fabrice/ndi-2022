use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(draw_board)
        .run()
}

#[derive(Component)]
struct Card {
    pos: Vec2,
    dragging: bool,
}

impl Card {
    fn new(pos: Vec2) -> Self {
        Card {
            pos,
            dragging: false,
        }
    }

    fn spawn(self, mut commands: Commands, asset_server: Res<AssetServer>) {
        let rect = commands
            .spawn(GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: Vec2::new(100., 150.),
                    origin: RectangleOrigin::TopLeft,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::Rgba {
                        red: 0.7,
                        green: 0.6,
                        blue: 0.6,
                        alpha: 1.0,
                    }),
                    outline_mode: StrokeMode {
                        options: Default::default(),
                        color: Color::Rgba {
                            red: 0.9,
                            green: 0.9,
                            blue: 0.8,
                            alpha: 1.0,
                        },
                    },
                },
                Transform::default(),
            ))
            .id();
        let text = commands
            .spawn(Text2dBundle {
                text: Text::from_section(
                    "Carte",
                    TextStyle {
                        font: asset_server.load("noto.ttf"),
                        font_size: 12.0,
                        color: Color::WHITE,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(10., 10., 0.0)),
                ..Default::default()
            })
            .id();

        let pos = self.pos;
        commands
            .spawn(self)
            .insert(TransformBundle::from_transform(
                Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0)),
            ))
            .insert(VisibilityBundle::default())
            .add_child(text)
            .add_child(rect);
    }
}

fn draw_board(mut commands: Commands, asset_server: Res<AssetServer>, win: Res<Windows>) {
    let win = win.iter().next().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            win.width() / 2.0,
            win.height() / -2.0,
            0.0,
        )),
        ..Default::default()
    });

    Card::new(Vec2::new(50.0, 0.0)).spawn(commands, asset_server);
}
