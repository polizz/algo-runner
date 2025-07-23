use std::collections::HashMap;

#[inline(always)]
fn get_match_cost(c1: &u8, c2: &u8) -> usize {
  if c1 == c2 {
    0
  } else {
    1
  }
}

#[inline(always)]
fn get_delete_cost(_c: &u8) -> usize {
  1
}

#[inline(always)]
fn get_insert_cost(_c: &u8) -> usize {
  1
}

type Cache = HashMap<(isize, isize), usize>;

pub fn get_distance_r(p: &[u8], t: &[u8], i: isize, j: isize, cache: &mut Cache) -> usize {
  if let Some(cost) = cache.get(&(i, j)) {
    return cost.to_owned();
  }

  const MATCH: usize = 0;
  const INSERT: usize = 1;
  const DELETE: usize = 2;

  let mut opt = [0; 3];

  if i < 0 {
    return (j + 1) as usize * get_insert_cost(&b' ');
  }

  if j < 0 {
    return (i + 1) as usize * get_delete_cost(&b' ');
  }

  opt[MATCH] =
    get_distance_r(p, t, i - 1, j - 1, cache) + get_match_cost(&p[i as usize], &t[j as usize]);
  opt[INSERT] = get_distance_r(p, t, i, j - 1, cache) + get_insert_cost(&t[j as usize]);
  opt[DELETE] = get_distance_r(p, t, i - 1, j, cache) + get_delete_cost(&p[i as usize]);

  let mut lowest_cost = opt[MATCH];
  for c in INSERT..=DELETE {
    if opt[c] < lowest_cost {
      lowest_cost = opt[c];
    }
  }

  cache.insert((i, j), lowest_cost);
  lowest_cost
}

pub fn get_distance(pattern: &[u8], target: &[u8]) -> usize {
  let mut cache: Cache = HashMap::new();

  get_distance_r(
    &pattern,
    &target,
    pattern.len() as isize - 1,
    target.len() as isize - 1,
    &mut cache,
  )
}

mod tests {
  use super::*;

  #[test]
  fn you_should_thou_shalt() {
    let target = "you should".as_bytes();
    let pattern = "thou shalt".as_bytes();

    let d = get_distance(&pattern, &target);

    assert_eq!(d, 5);
  }

  #[test]
  fn names() {
    let pattern = "landrew".as_bytes();
    let target = "andrew".as_bytes();

    let d = get_distance(&pattern, &target);

    assert_eq!(d, 1);
  }

  #[test]
  fn max() {
    let target = "12345 andrew should want".as_bytes();
    let pattern = "67890 kelly is another way".as_bytes();

    let d = get_distance(&pattern, &target);

    assert_eq!(d, 21);
  }

  #[test]
  fn extreme() {
    let target = "For associations there has a particular use case where Arun brought up -- no filtering has to happen due to Astra (CXO) wanting to choose the right care plan after promotions at cart level is applied. Even though we may have accommodated this in Elixir this may be inconsistent with Walmart behavior and introduces many challenges such as payload sizes, multiple networks to Product, OfferListing, Pricing, and Promotions when trying to achieve what is needed to provide all care plans (or associations) irrespective of association group type -- warranties, installations, care-plans.".as_bytes();

    let pattern = "For associations there has a particular use case where Arun brought up -- no filtering has to happen due to Astra (CXO) wanting to choose the right care plan after promotions at cart level is applied. Even1 though we may have accommodated this in Elixir this may be inconsistent with Walmart behavior and introduces many challenges such as payload sizes, multiple networks to Product, OfferListing, Pricing, and Promotions when trying to achieve what is needed to provide all care plans (or associations) irrespective of association group type -- warranties, installations, care-plans.".as_bytes();

    let d = get_distance(&pattern, &target);

    assert_eq!(d, 1);
  }
}
