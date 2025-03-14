/*
    Nth Fibonacci Number
    Implement a function to calculate the `n`th Fibonacci number. 
    The Fibonacci sequence is defined as follows:
    F(0) = 0, F(1) = 1, F(n) = F(n-1) + F(n-2) for n > 1.

    You need to implement the function `fib(n: i32) -> i32` to return the `n`th Fibonacci number.
    
    Hint: Consider using matrix exponentiation to solve the problem in O(log n) time complexity.
*/

use std::fmt::{self, Display, Formatter};

pub fn fib(n: i32) -> i32 {
    // Handle negative input
    if n < 0 {
        return 0;
    }
    
    // Handle base cases
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    // Matrix multiplication helper function
    fn multiply_matrices(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
        [
            [
                a[0][0] * b[0][0] + a[0][1] * b[1][0],
                a[0][0] * b[0][1] + a[0][1] * b[1][1]
            ],
            [
                a[1][0] * b[0][0] + a[1][1] * b[1][0],
                a[1][0] * b[0][1] + a[1][1] * b[1][1]
            ]
        ]
    }
    
    // Matrix power using divide and conquer
    fn matrix_power(matrix: [[i32; 2]; 2], power: i32) -> [[i32; 2]; 2] {
        if power == 0 {
            return [[1, 0], [0, 1]];  // Identity matrix
        }
        if power == 1 {
            return matrix;
        }
        
        let half = matrix_power(matrix, power / 2);
        let result = multiply_matrices(half, half);
        
        if power % 2 == 0 {
            result
        } else {
            multiply_matrices(result, matrix)
        }
    }
    
    // The base matrix for Fibonacci
    let base_matrix = [[1, 1], [1, 0]];
    
    // Calculate power
    let result_matrix = matrix_power(base_matrix, n - 1);
    
    // The nth Fibonacci number is result_matrix[0][0]
    result_matrix[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
