use toml::Value;
use std::collections::BTreeMap;
use pad::{PadStr, Alignment};
use console::style;

fn get_column_width(vec_keys: &Vec<String>) -> usize{
    let max_value: Option<usize> = vec_keys.iter().map(|x| x.len()).max();
    let col_width = match max_value {
    // The division was valid
    Some(x) => x,
    // The division was invalid
    None    => 10,
    };
    col_width
}

fn get_formated_titles(vec_keys: &Vec<String>, width: usize) -> String{
    let names_with_white_space: Vec<_> = vec_keys 
        .iter()
        .map(|x| x.pad_to_width_with_alignment(width, Alignment::Right))
        .collect();
    let title = names_with_white_space.join("  ");
    let left_margin = "\t";
    let title_formated:String = format!("{}{}",left_margin ,title);
    return title_formated;
}

fn get_formated_types(toml_btreemap: BTreeMap<String, Vec<toml::Value>>,
                    width: usize) -> String{
    let value_types: BTreeMap<String, Vec<String>> = toml_btreemap
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(|v| 
                 match v {
                     Value::String(_v)       => format!("<str>"),
                     Value::Integer(_v)      => format!("<int>"),
                     Value::Float(_v)        => format!("<dbl>"),
                     Value::Boolean(_v)      => format!("<lgl>"),
                     Value::Datetime(_v)     => format!("<dttm>"),
                     _                       => format!("None")})
             .collect()))
        .collect();

    let first_values = value_types.values().
        filter_map(|vec| vec.first()).collect::<Vec<&String>>(); 

    let types_with_white_space: Vec<_> = first_values
        .iter()
        .map(|x| x.pad_to_width_with_alignment(width, Alignment::Right))
        .collect();
    let types = types_with_white_space.join("  ");
    let left_margin = "\t";
    let types_formated:String = format!("{}{}",left_margin ,types);
    return types_formated;
}

fn main() {
    let contents = std::fs::read_to_string("data/data.toml").unwrap();
    let raw_btreemap_toml: BTreeMap<String, Vec<Value>> = toml::from_str(&contents).unwrap();
    let toml_values = raw_btreemap_toml.values().cloned().collect::<Vec<_>>();
    let toml_keys = raw_btreemap_toml.keys().cloned().collect::<Vec<String>>();
    let bt = raw_btreemap_toml.clone();
    let n_element: usize = toml_values[0].len();
    let width = get_column_width(&toml_keys);
    let min_width: usize = 10;
    let width = if min_width > width {min_width} else {width};
    let titles = get_formated_titles(&toml_keys, width);
    let types = get_formated_types(bt, width);
    println!("{}",style(titles.clone()).bold());
    println!("{}",style(types.clone()).bright().black());
    for i in 0..n_element{
        print!("\t");
        toml_values.iter().for_each(|v| print!("\t{} ", v[i]));
        println!("");
    }
    
   // for i in 0..n_element{
   //     toml_values.iter().for_each(|v| v.into_iter().map(|x| x.collect::<Vec<String>>())
   // }

}
