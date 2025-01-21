# Shamir Secret Sharing Threshold Scheme

This Rust library implements the Shamir Secret Sharing scheme, which allows a secret to be divided into multiple shares. A minimum number of shares (threshold) is required to reconstruct the secret. The library also supports an additional password for enhanced security.

## Functions

### `setup`

Generates `n` shares from a given secret using a polynomial of degree `threshold - 1`.

#### Arguments

- `secret`: The secret to be shared.
- `threshold`: The minimum number of shares required to reconstruct the secret.
- `n`: The total number of shares to generate.

#### Returns

A vector of tuples where each tuple contains a share (x, y).

### `reconstruct_data`

Reconstructs the secret from the given shares using the Shamir Secret Sharing scheme.

#### Arguments

- `x`: A vector of x-coordinates of the shares.
- `y`: A vector of y-coordinates of the shares.

#### Returns

The reconstructed secret.

### `passworded_setup`

Generates `n` shares from a given secret and password using a polynomial of degree `threshold - 1`.

#### Arguments

- `secret`: The secret to be shared.
- `threshold`: The minimum number of shares required to reconstruct the secret.
- `n`: The total number of shares to generate.
- `password`: An additional password used in the share generation.

#### Returns

A vector of tuples where each tuple contains a share (x, y).

### `reconstruct_data_with_password`

Reconstructs the secret from the given shares and password using the Shamir Secret Sharing scheme.

#### Arguments

- `x`: A vector of x-coordinates of the shares.
- `y`: A vector of y-coordinates of the shares.
- `password`: The password used in the share generation.

#### Returns

The reconstructed secret.

## Example Usage

```rust
use shamir_secret_sharing::{setup, reconstruct_data, passworded_setup, reconstruct_data_with_password};
use ark_ff::PrimeField;

fn main() {
    // Example usage of setup and reconstruct_data
    let secret = Fq::from(1234567890u64);
    let threshold = 3;
    let n = 5;
    let shares = setup(secret, threshold, n);
    let (x, y): (Vec<_>, Vec<_>) = shares.iter().cloned().unzip();
    let reconstructed_secret = reconstruct_data(x, y);
    assert_eq!(secret, reconstructed_secret);

    // Example usage of passworded_setup and reconstruct_data_with_password
    let password = Fq::from(987654321u64);
    let passworded_shares = passworded_setup(secret, threshold, n, password);
    let (x, y): (Vec<_>, Vec<_>) = passworded_shares.iter().cloned().unzip();
    let reconstructed_secret_with_password = reconstruct_data_with_password(x, y, password);
    assert_eq!(secret, reconstructed_secret_with_password);
}
```

```

## License

This project is licensed under the MIT License.



```
