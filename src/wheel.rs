// All the properties of all the elements of a roulette table.
#[derive(PartialEq, Eq, Hash)]
pub enum WheelElementProperty {
    Zero,
    One,
    Two,
    Three,
    Four,

    Red,
    Black,
    Green,

    Odd,
    Oven,
}

// All the possible bets for a roulette table
#[derive(PartialEq, Eq, Hash)]
pub enum WheelElement {
    Zero,
    One,
    Two,
    Three,
    Four,
    ThritySix,
}

pub fn is_win(bet: WheelElementProperty, wheel: WheelElement) -> bool {
    use itertools::Itertools;
    use self::WheelElement as Element;
    use self::WheelElementProperty as Property;
    use std::collections::{HashMap, HashSet};

    let properties_by_element: HashMap<Element, HashSet<Property>> =
        vec![(Element::Zero, vec![Property::Zero, Property::Green])]
            .into_iter()
            .map(|(element, properties)| {
                     (element, properties.into_iter().collect::<HashSet<Property>>())
                 })
            .collect();

    let all_element_property_pairs: Vec<(Element, Property)> = properties_by_element
        .into_iter()
        .flat_map(|(element, properties)| {
                      properties
                          .into_iter()
                          .map(|property| (element, property))
                  })
        .collect();

    let aaaaaaa = all_element_property_pairs
        .into_iter()
        .group_by(|&(_, property)| property)
        .into_iter();

    let elements_by_property: HashMap<Property, HashSet<Element>> = aaaaaaa
        .map(|(property, pairs)| {
                 (property,
                  pairs
                      .into_iter()
                      .map(|(element, _)| element)
                      .into_iter()
                      .collect::<HashSet<Element>>())
             })
        .collect();

    elements_by_property
        .get(&bet)
        .expect("Element not defined in rules")
        .contains(&wheel)
}

#[cfg(test)]
mod tests {
    use wheel::*;

    #[test]
    fn given_number_bet_when_outcome_is_number_then_true() {
        let bet = WheelElementProperty::Two;
        let wheel = WheelElement::Two;

        assert_eq!(true, is_win(bet, wheel));
    }



    #[test]
    fn given_number_bet_when_outcome_is_different_number_then_false() {
        let bet = WheelElementProperty::Two;
        let wheel = WheelElement::Three;

        assert_eq!(false, is_win(bet, wheel));
    }
}
