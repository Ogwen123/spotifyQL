use crate::query::data::KeyAccess;

fn bigger<T>(v1: &T, v2: &T, attributes: &Vec<String>) -> Result<bool, String>
where
    T: KeyAccess + Clone,
{
    for i in attributes {
        if v1.access(i)? > v2.access(i)? {
            return Ok(true);
        } else if v1.access(i)? < v2.access(i)? {
            return Ok(false);
        }
    }

    Ok(false)
}

/// Mergesort for an array of any struct implementing KeyAccess
pub fn mergesort<T>(valid: &Vec<T>, attributes: &Vec<String>) -> Result<Vec<T>, String>
where
    T: KeyAccess + Clone + Default,
{
    if valid.len() == 1 || valid.len() == 0 {
        return Ok(valid.clone());
    }

    let mut res: Vec<T> = Vec::with_capacity(valid.len());
    res.resize_with(valid.len(), T::default); // init array with default values

    let mut data: Vec<T> = valid.clone().to_vec();

    let split = valid.len() / 2;

    let rhs = mergesort(&data.split_off(split), attributes)?;
    let lhs = mergesort(&data, attributes)?;

    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;
    
    while i < rhs.len() && j < lhs.len() {
        if bigger(&rhs[i], &lhs[j], attributes)? {
            res[k] = lhs[j].clone();
            j += 1;
        } else {
            res[k] = rhs[i].clone();
            i += 1;
        }
        k += 1
    }
    
    while i < rhs.len() {
        res[k] = rhs[i].clone();
        i += 1;
        k += 1;
    }
    
    while j < lhs.len() {
        res[k] = lhs[j].clone();
        j += 1;
        k += 1;
    }
    
    Ok(res)
}

