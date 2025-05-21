fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() {
            return;
        }
        
        let mut tem_zero_na_coluna = false;
        let mut tem_zero_na_linha = false;

        //testar primeira coluna
        for coluna in 0..matrix[0].len() {
            if matrix[0][coluna] == 0 {
                tem_zero_na_coluna = true;
                break;
            }
        }

        //testar primeira linha
        for linha in 0..matrix.len() {
            if matrix[linha][0] == 0 {
                tem_zero_na_linha = true;
                break;
            }
        }

        //testar todos os elementos, a partir da 2 linha e coluna
        for linha in 1..matrix.len() {
            for coluna in 1..matrix[linha].len() {
                if matrix[linha][coluna] == 0 {
                    matrix[0][coluna] = 0;
                    matrix[linha][0] = 0;
                }
            }
        }
        
        //marcar com zero, todas colunas marcados com zero
        for coluna in 1..matrix[0].len() {
            if matrix[0][coluna] == 0 {
                for linha in 1..matrix.len() {
                    matrix[linha][coluna] = 0;
                }
            }
        }
        
        //marcar com zero, todas as linhas marcados com zero
        for linha in 1..matrix.len() {
            if matrix[linha][0] == 0 {
                for coluna in 1..matrix[0].len() {
                    matrix[linha][coluna] = 0;
                }
            }
        }
        
        if tem_zero_na_coluna {
            for coluna in 0..matrix[0].len() {
                matrix[0][coluna] = 0;
            }
        }
        
        if tem_zero_na_linha {
            for linha in 0..matrix.len() {
                matrix[linha][0] = 0;
            }
        }
    }
}

#[test]
fn test_set_zeroes() {
    /*let mut matrix = vec![
        vec![1,1,1],
        vec![1,0,1],
        vec![1,1,1],
    ];
    Solution::set_zeroes(&mut matrix);
    let expected = vec![
        vec![1,0,1],
        vec![0,0,0],
        vec![1,0,1],
    ];
    assert_eq!(matrix, expected);*/

    let mut matrix = vec![
        vec![0, 1, 2, 0], 
        vec![3, 4, 5, 2], 
        vec![1, 3, 1, 5]
    ];
    Solution::set_zeroes(&mut matrix);
    let expected = vec![
        vec![0, 0, 0, 0], 
        vec![0, 4, 5, 0], 
        vec![0, 3, 1, 0]
    ];

    assert_eq!(matrix, expected);
}
