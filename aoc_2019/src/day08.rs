type Layer = [[u8; 6]; 25];

#[aoc_generator(day8)]
fn generator(input: &str) -> Vec<Layer> {
    let layers = input.len() / (6 * 25);
    let mut v: Vec<Layer> = Vec::with_capacity(layers);
    for i in 0..layers {
        for x in 0..25 {
            let row = [0; 6];
            v[i][x] = row;
        }
    }
    input.chars().map(|c| c.to_string().parse::<u8>().unwrap()).enumerate().for_each(|(i, v)| {
        let layer = i / layers;
        let intern = i % layers;
        let x = intern / 25;
        let y = intern % 6;
        (v[layer])[x][y] = v;
    });
    v
}