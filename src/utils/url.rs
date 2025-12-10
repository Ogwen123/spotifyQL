type Params<K, V> = Vec<(K, V)>;

pub fn build_url<K: AsRef<str>, V: AsRef<str>, S: ToString>(base: S, params: Params<K, V>) -> String {
    let mut res = base.to_string() + "?";

    for (index, (k, v)) in params.iter().enumerate() {
        res += k.as_ref();
        res += "=";
        res += v.as_ref();
        if index != params.len() - 1 {
            res += "&"
        }
    }

    res
}

pub fn parameterise_list<T: AsRef<str>>(data: Vec<T>) -> String {
    let mut res = String::new();

    for (index, i) in data.iter().enumerate() {
        res += i.as_ref();

        if index != data.len() - 1 {
            res += ","
        }
    }

    res
}
