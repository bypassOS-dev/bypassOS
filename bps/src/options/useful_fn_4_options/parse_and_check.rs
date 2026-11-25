use ureq::Response;
use crate::usefull_fun::error_output::error_output;
use crate::json_parser::get_arr_of_vers;

fn _parse_and_check_for_db_existing(response: Response, utils_name: &String) {
    let Ok(data_to_text) = response.into_string() else {
        error_output("Data from server is invalid and we couldn't parse it, sorry.");
        std::process::exit(1);
    };

    let Some(json) = get_arr_of_vers(&data_to_text) else {
        error_output("Data from server is invalid and we couldn't parse it to json, sorry.");
        std::process::exit(1);
    };

    let Some(_data_about_util) = json.get(utils_name) else {
        error_output("Data was damaget and we couldn't get information about package");
        std::process::exit(1);
    };
}