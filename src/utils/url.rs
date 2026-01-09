type Params<K, V> = Vec<(K, V)>;

pub fn url_encode(unencoded: String) -> String {
    let mut str: String = String::new();

    for c in unencoded.split("") {
        str += match c {
            "!" => "%21",
            "#" => "%23",
            "$" => "%24",
            "&" => "%26",
            "'" => "%27",
            "(" => "%28",
            ")" => "%29",
            "*" => "%2A",
            "+" => "%2B",
            "," => "%2C",
            "/" => "%2F",
            ":" => "%3A",
            ";" => "%3B",
            "=" => "%3D",
            "?" => "%3F",
            "@" => "%40",
            "[" => "%5B",
            "]" => "%5D",
            _ => c,
        }
    }

    str
}

pub fn build_url<K: AsRef<str>, V: AsRef<str>, S: ToString>(
    base: S,
    params: Params<K, V>,
) -> String {
    if params.len() == 0 {
        return base.to_string();
    }

    let mut res = base.to_string() + "?";

    for (index, (k, v)) in params.iter().enumerate() {
        res += k.as_ref();
        res += "=";
        res += url_encode(v.as_ref().to_string()).as_str();
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
