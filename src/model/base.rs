use crate::fetch::fun::get_detailed_list;

use super::cast::base::{
    date_to_model_index, genre_id_to_model_index, n_episodes_to_model_index, rating_to_model_index,
};

type BaseModel = Vec<Vec<Vec<i32>>>;

pub async fn generate_base_model(s_user: String, reload: bool) -> Result<BaseModel, u16> {
    let list = match get_detailed_list(s_user, reload).await {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    let mut model: BaseModel = new_model();
    let mut count: [i32; 6] = [0, 0, 0, 0, 0, 0];

    for i in 0..list.len() {
        let sus = (list[i].entry.status + 1) as usize;
        let score = list[i].entry.score;

        //airing decade
        match list[i].details.airing_date {
            Some(d) => {
                let x = date_to_model_index(d);
                model[x[0]][x[1]][0] += score as i32;
                model[x[0]][x[1]][sus] += 1;
                count[x[0]] += 1;
                match score {
                    0 => (),
                    _ => model[x[0]][x[1]][1] += 1,
                };
            }
            None => (),
        }

        //rating
        match list[i].details.rating {
            Some(r) => {
                let x = rating_to_model_index(r);
                model[x[0]][x[1]][0] += score as i32;
                model[x[0]][x[1]][sus] += 1;
                count[x[0]] += 1;
                match score {
                    0 => (),
                    _ => model[x[0]][x[1]][1] += 1,
                };
            }
            None => (),
        }

        //number of episodes
        match list[i].details.num_episodes {
            Some(n) => {
                let x = n_episodes_to_model_index(n);
                model[x[0]][x[1]][0] += score as i32;
                model[x[0]][x[1]][sus] += 1;
                count[x[0]] += 1;
                match score {
                    0 => (),
                    _ => model[x[0]][x[1]][1] += 1,
                };
            }
            None => (),
        }

        //genres
        match list[i].details.genres.to_owned() {
            Some(genres) => {
                for g in genres.iter() {
                    match g.to_owned() {
                        Some(g) => {
                            let x = genre_id_to_model_index(g);
                            model[x[0]][x[1]][0] += score as i32;
                            model[x[0]][x[1]][sus] += 1;
                            count[x[0]] += 1;
                            match score {
                                0 => (),
                                _ => model[x[0]][x[1]][1] += 1,
                            };
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    for i in 0..model.len() {
        for c in 0..model[i].len() {
            let tot =
                model[i][c][2] + model[i][c][3] + model[i][c][4] + model[i][c][5] + model[i][c][6];

            model[i][c][2] = model[i][c][2] * 1000 / count[i];
            model[i][c][3] = model[i][c][3] * 1000 / count[i];
            model[i][c][4] = model[i][c][4] * 1000 / count[i];
            model[i][c][5] = model[i][c][5] * 1000 / count[i];
            model[i][c][6] = model[i][c][6] * 1000 / count[i];

            model[i][c][0] = model[i][c][0]
                / match model[i][c][1] {
                    0 => 1,
                    _ => model[i][c][1],
                };

            model[i][c][1] = match tot {
                0 => 0,
                _ => model[i][c][1] * 1000 / tot,
            };
        }
    }

    Ok(model)
}

fn new_model() -> BaseModel {
    vec![
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
        ],
    ]
}