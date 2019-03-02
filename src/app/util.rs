pub fn get_percentage_char(percentage: f64, from: &[char]) -> char
{
    let idx = (percentage / 101.0 * from.len() as f64).floor() as usize;
    from[idx]
}
