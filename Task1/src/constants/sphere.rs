pub const WATER_DENSITY: f64 = 1000.0;
pub const DEFAULT_EPSILON: f64 = 1e-6;
pub const VOLUME_COEFFICIENT: f64 = 4.0;
pub const SUBMERSION_COEFFICIENT: f64 = 3.0;
pub const MAX_DEPTH_FACTOR: f64 = 2.0;
pub const TABLE_WIDTH: usize = 80;

pub const WOOD_MATERIALS: &[(&str, f64)] = &[
    ("Пробка", 250.0),
    ("Бамбук", 400.0),
    ("Сосна (белая)", 500.0),
    ("Кедр", 550.0),
    ("Дуб", 700.0),
    ("Бук", 750.0),
    ("Красное дерево", 800.0),
    ("Тиковое дерево", 850.0),
    ("Парафин", 900.0),
    ("Полиэтилен", 920.0),
    ("Пчелиный воск", 950.0),
];
