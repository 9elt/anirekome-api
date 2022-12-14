use crate::helper;

use crate::algorithm::model::Model;
use crate::algorithm::user;

pub async fn get_user_model(user: &String, force_update: &bool) -> Result<Model<i16>, u16> {
    let mut stats_model = Model::<i16>::empty();
    let mut update_required: bool = false;

    if !force_update {
        let check_db = helper::get_user_model(&user);
        match check_db {
            Ok(model) => {
                update_required = model.requires_update();
                match model.model() {
                    Some(m) => {
                        stats_model = Model::from_vec(m);
                    }
                    None => update_required = true,
                }
            }
            Err(_) => update_required = true,
        }
    }

    if update_required || *force_update {
        let list = match helper::get_detailed_list(user, true, false).await {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        stats_model = user::stats::stats_model(list);
        helper::save_user_model(&user, &stats_model);
    }

    Ok(stats_model)
}
