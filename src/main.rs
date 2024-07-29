fn median(a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    let mut sorted: Vec<f32> = a.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if sorted.len() % 2 == 1 {
        Some(sorted[sorted.len() / 2])
    } else {
        Some((sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0)
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
