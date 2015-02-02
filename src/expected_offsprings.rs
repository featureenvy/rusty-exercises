pub fn run(dd: i32, dm: i32, dr: i32, mm: i32, mr: i32, rr: i32) -> f32 {
    let ddo = dd * 2;
    let dmo = dm * 2;
    let dro = dr * 2;
    let mmo = mm * 2;
    let mro = mr * 2;
    let rro = rr * 2;

    ddo as f32 * 1.0 +
    dmo as f32 * 1.0 +
    dro as f32 * 1.0 +
    mmo as f32 * 0.75 +
    mro as f32 * 0.5 +
    rro as f32 * 0.0
}
