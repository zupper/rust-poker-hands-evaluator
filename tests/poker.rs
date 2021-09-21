use poker_hands::winning_hands;

#[test]
fn it_works() {
  let _res: Vec<&str> = winning_hands(&[]);
}

#[test]
#[should_panic]
fn it_panics_on_empty_string_hands() {
  let _res = winning_hands(&[""]);
}

#[test]
#[should_panic]
fn it_panics_on_single_char_string_hands() {
  let _res = winning_hands(&["a 5H 6H 7H 10H"]);
}

#[test]
#[should_panic]
fn it_panics_on_non_ascii_chars() {
  let _res = winning_hands(&["БЮ 5H 6H 7H 10H"]);
}

#[test]
#[should_panic]
fn it_fails_with_a_wrong_suite() {
  let _res = winning_hands(&["2A 5H 6H 7H 10H"]);
}

#[test]
fn it_works_with_the_correct_suite() {
  let _res = winning_hands(&["2C 5H 6D 7S 10H"]);
}

#[test]
#[should_panic]
fn it_panics_on_card_with_four_chars() {
  let _res = winning_hands(&["222C 5H 6D 7S 10H"]);
}

#[test]
#[should_panic]
fn it_panics_on_invalid_rank_z() {
  let _res = winning_hands(&["ZC 5H 6D 7S 10H"]);
}

#[test]
#[should_panic]
fn it_panics_on_invalid_rank_b() {
  let _res = winning_hands(&["BC 5H 6D 7S 10H"]);
}

#[test]
fn it_works_with_numeric_rank() {
  let _res = winning_hands(&["5C 5H 6D 7S 10H"]);
}

#[test]
#[should_panic]
fn it_panics_on_invalid_rank_1() {
  let _res = winning_hands(&["1C 5H 6D 7S 10H"]);
}

#[test]
fn it_works_with_valid_alpha_ranks() {
  let _res = winning_hands(&["JC 5H 6D 7S 10H"]);
  let _res = winning_hands(&["QC 5H 6D 7S 10H"]);
  let _res = winning_hands(&["KC 5H 6D 7S 10H"]);
  let _res = winning_hands(&["AC 5H 6D 7S 10H"]);
}

#[test]
fn it_works_with_rank_10() {
  let _res = winning_hands(&["10H 5H 6D 7S 10H"]);
}

#[test]
fn it_works_with_five_cards() {
  let _res = winning_hands(&["4H 5H 6H 7H 10H"]);
}

#[test]
#[should_panic]
fn it_panics_with_six_cards() {
  let _res = winning_hands(&["4H 5H 6H 7H 10H AH"]);
}

#[test]
#[should_panic]
fn it_panics_with_four_cards() {
  let _res = winning_hands(&["4H 5H 6H 7H"]);
}

#[test]
fn it_returns_the_only_hand_passed() {
  let winner = "2C 5D 7H 9S 10C";
  let res: Vec<&str> = winning_hands(&[winner]);
  assert_eq!(res[0], winner);
}

#[test]
fn it_returns_the_high_card_hand() {
  let winner = "10C 2C 5D 7H 9S";
  let loser = "2C 5D 7H 8S 9C";
  let res: Vec<&str> = winning_hands(&[loser, winner]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_the_same_hand_twice() {
  let winner1 = "2C 5D 7H 9S 10C";
  let winner2 = "2C 5D 7H 9S 10C";
  let res: Vec<&str> = winning_hands(&[winner1, winner2]);
  assert_eq!(res[0], winner1);
  assert_eq!(res[1], winner2);
}

#[test]
fn it_returns_one_pair_over_high_card() {
  let winner = "2C 2D 7H 9S 10C";
  let loser = "2C 5D 7H 9S AC";
  let res: Vec<&str> = winning_hands(&[loser, winner]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_highest_of_several_one_pair_hands() {
  let winner = "7C 7D 8H 9S 10C";
  let loser1 = "2C 2D 7H KS AC";
  let loser2 = "3C 3D 7H KS AC";
  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_two_pairs_over_one_pair() {
  let winner = "7C 7D 8H 8S 10C";
  let loser1 = "AC AD 7H KS QC";
  let loser2 = "KC KD 6H 7S QC";
  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_two_pair_with_higher_high_card() {
  let winner = "7C 7D 8H 8S 10C";
  let loser1 = "4C 4D 5H 5S 7C";
  let loser2 = "3C 3D 2H 2S 7C";
  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_two_pair_with_higher_low_card() {
  let winner1 = "8C 8D 7H 7S 10C";
  let loser1 = "8C 8D 5H 5S 7C";

  let res: Vec<&str> = winning_hands(&[loser1, winner1]);
  assert_eq!(res[0], winner1);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_two_pair_with_higher_kicker() {
  let winner = "8C 8D 7H 7S 10C";
  let loser1 = "8C 8D 7H 7S 9C";
  let loser2 = "8C 8D 7H 7S 6C";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_three_of_a_kind_over_two_pair() {
  let winner = "8C 8D 8H 7S 10C";
  let loser1 = "8C 8D 7H 7S AC";
  let loser2 = "AC AD KH KS QC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_three_of_a_kind_with_highest_ranking_triplet() {
  let winner = "8C 8D 8H 7S 10C";
  let loser1 = "6C 6D 6H 7S AC";
  let loser2 = "7C 7D 7H KS QC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_three_of_a_kind_with_highest_ranking_first_kicker_and_equal_triplets() {
  let winner = "8C 8D 8H AS 10C";
  let loser1 = "8C 8D 8H 7S KC";
  let loser2 = "8C 8D 8H KS QC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_three_of_a_kind_with_highest_ranking_second_kicker_all_else_equal() {
  let winner = "8C 8D 8H AS KC";
  let loser1 = "8C 8D 8H 7S AC";
  let loser2 = "8C 8D 8H 6S AC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_straight_over_three_of_a_kind() {
  let winner = "3C 4D 5H 6S 7C";
  let loser1 = "8C 8D 8H 7S AC";
  let loser2 = "8C 8D 8H 6S AC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_recognizes_baby_straight() {
  let winner = "AC 2D 3H 4S 5C";
  let loser1 = "8C 8D 8H 7S AC";
  let loser2 = "8C 8D 8H 6S AC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_higer_straight() {
  let winner = "KC QD AH JS 10C";
  let loser1 = "2C 3D 4H 5S 6C";
  let loser2 = "9C 10D JH QS KC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_flush_over_straight() {
  let winner = "2C 5C 7C 9C JC";
  let loser1 = "2C 3D 4H 5S 6C";
  let loser2 = "9C 10D JH QS KC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_higher_flush() {
  let winner = "3C 5C 9C KC AC";
  let loser1 = "2C 4C 5C 6C 7C";
  let loser2 = "2C 4C 6C QC AC";
  let loser3 = "2C 5C 8C KC AC";
  let loser4 = "2C 4C 9C KC AC";
  let loser5 = "2C 5C 9C KC AC";

  let res: Vec<&str> = winning_hands(&[loser5, loser4, loser3, winner, loser1, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_full_house_over_flush() {
  let winner = "3C 3S 3H 2C 2S";
  let loser1 = "3C 5C 9C KC AC";
  let loser2 = "3C 5C 9C QC KC";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_full_house_with_higher_triplet() {
  let winner = "5C 5S 5H 2C 2S";
  let loser1 = "4C 4S 4H 2C 2S";
  let loser2 = "3C 3S 3H 2C 2S";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_full_house_with_higher_pair() {
  let winner = "5C 5S 5H 4C 4S";
  let loser1 = "5C 5S 5H 2C 2S";
  let loser2 = "5C 5S 5H 3C 3S";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_four_of_a_kind_over_full_house() {
  let winner = "3C 3S 3H 3D 2S";
  let loser1 = "5C 5S 5H 2C 2S";
  let loser2 = "5C 5S 5H 3C 3S";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_higher_quadruplet_four_of_a_kind() {
  let winner = "KC KS KH KD 2S";
  let loser1 = "JC JS JH JD 2S";
  let loser2 = "3C 3S 3H 3D 2S";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_higher_kicker_four_of_a_kind() {
  let winner = "KC KS KH KD 7S";
  let loser1 = "KC KS KH KD 3S";
  let loser2 = "KC KS KH KD 5S";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_straight_flush_over_four_of_a_kind() {
  let winner = "2D 3D 4D 5D 6D";
  let loser1 = "KC KS KH KD 3S";
  let loser2 = "KC KS KH KD 5S";

  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}

#[test]
fn it_returns_higher_straight_flush() {
  let winner = "AD 10D JD QD KD";
  let loser1 = "AD 2D 3D 4D 5D";
  let loser2 = "7D 8D 9D 10D JD";


  let res: Vec<&str> = winning_hands(&[loser1, winner, loser2]);
  assert_eq!(res[0], winner);
  assert_eq!(res.len(), 1);
}
