
struct OblivionSave {
    file_header: FileHeader,
    save_header: SaveHeader,
    plugins: Vec<String>,
    global: GlobalSect
    change_records: vec<ChangeRecords>,
    temp_effects: TemporaryEffectsSect,
    form_ids: FormIds,
    world_spaces: WorldSpaces,
}

struct FileHeader {
    file_id: [u8; 12],
    major_version: u8,
    minor_version: u8,
    exe_time: SystemTime,
}

struct SaveHeader {
    header_version: u32,
    save_header_size: u32,
    save_num: u32,
    pc_name: String,
    pc_level: u16,
    pc_location: String,
    game_days: f32,
    game_ticks: u32,
    game_time: SystemTime,
    screenshot: Screenshot,
}

struct GlobalSect {
    form_ids_offset: u32,
    records_num: u32,
    next_object_id: u32,
    world_id: u32,
    world_x: u32,
    world_y: u32,
    pc_location: Location,
    globals_num: u16,
    globals: Vec<Global>,
    tes_class_size: u16,
    num_death_counts: u32,
    death_counts: Vec<DeathCount>,
    game_mode_seconds: f32,
    processes_size: u16,
    processes_data: Vec<u8>,
    spec_event_size: u16,
    spec_event_data: Vec<u8>,
    weather_size: u16,
    weather_data: Vec<u8>,
    player_combat_count: u32,
    created_num: u32,
    created_data: Vec<Record>,
    quick_keys_size: u16,
    quick_keys_data: Vec<QuickKey>,
    reticule_size: u16,
    reticule_data: Vec<u8>,
    interface_size: u16,
    interface_data: Vec<u8>,
    regions_size: u16,
    regions_num: u16,
    regions: Vec<Region>,
}

struct TemporaryEffectsSect {
    temp_effect_size: u32,
    data: Vec<u8>,
}

struct FormIds {
    form_ids_num: u32,
    form_ids: Vec<u32>,
}

struct WorldSpaces {
    world_spaces_num: u32,
    world_spaces: Vec<u32>,
}

struct SystemTime {
    year: u16,
    month: u16,
    day_of_week: u16,
    day: u16,
    hour: u16,
    minute: u16,
    second: u16,
    miliseconds: u16,
}

struct Screenshot {
    size: u32,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

struct Location {
    cell: u32,
    x: f32,
    y: f32,
    z: f32,
}

struct Global {
    iref: u32,
    value: float,
}

struct DeathCount {
    actor: u32,
    death_count: u16,
}

struct Record {
    type: [u8; 4],
    data_size: u32,
    flags: u32,
    form_id: u32,
    version_control_info: u32,
    data: Vec<u8>,
}

struct QuickKey {
    flag: u8,
    iref: u32,
}

struct Region {
    iref: u32
    unknown6: i32, // Flags?
}

struct ChangeRecord {
    form_id: u32,
    type: u8,
    flags: u32,
    version: u8,
    data_size: u16,
    data: Vec<u8>
}
