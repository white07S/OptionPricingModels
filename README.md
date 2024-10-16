# OptionPricingModels

A comprehensive Rust library for pricing financial options using various mathematical models and regression techniques. This project implements the Binomial, Black-Scholes, Heston, and Intrinsic Value models, alongside regression methods like Least Squares Monte Carlo (LSM) and Random Forest Regression to handle American options.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Technical Overview](#technical-overview)
  - [Option Pricing Models](#option-pricing-models)
    - [Binomial Model](#binomial-model)
    - [Black-Scholes Model](#black-scholes-model)
    - [Heston Model](#heston-model)
    - [Intrinsic Value](#intrinsic-value)
  - [Regression Techniques](#regression-techniques)
    - [Least Squares Monte Carlo (LSM)](#least-squares-monte-carlo-lsm)
    - [Random Forest Regression](#random-forest-regression)
  - [Utilities](#utilities)
    - [Interpolation](#interpolation)
    - [Mathematical Functions](#mathematical-functions)
  - [Data Structures](#data-structures)
    - [InterestRateCurve](#interestratecurve)
  - [Traits and Errors](#traits-and-errors)
- [Mathematical Foundations](#mathematical-foundations)
  - [Binomial Model](#binomial-model-1)
  - [Black-Scholes Model](#black-scholes-model-1)
  - [Heston Model](#heston-model-1)
  - [Regression Methods](#regression-methods)
- [Design Choices](#design-choices)
  - [Modular Design](#modular-design)
  - [Trait-based Abstraction](#trait-based-abstraction)
  - [Error Handling](#error-handling)
  - [Performance Optimizations](#performance-optimizations)
- [Library Dependencies](#library-dependencies)
- [Performance Indicators](#performance-indicators)
- [Implementation Methods](#implementation-methods)
  - [Numerical Methods](#numerical-methods)
  - [Simulation Techniques](#simulation-techniques)
- [Contributing](#contributing)
- [License](#license)
- [References](#references)

## Overview

The `OptionPricingModels` library is designed to provide robust and efficient tools for pricing European and American options using a variety of mathematical models. By leveraging Rust's performance and safety features, this library ensures accurate computations suitable for financial analysis and research.

## Features

- **Binomial Model**: Discrete-time model for option pricing, suitable for American options.
- **Black-Scholes Model**: Analytical model for European option pricing.
- **Heston Model**: Stochastic volatility model for more realistic option pricing.
- **Intrinsic Value**: Simple model calculating the immediate exercise value.
- **Regression Techniques**: 
  - Least Squares Monte Carlo (LSM) for American options.
  - Random Forest Regression for advanced predictive modeling.
- **Utilities**: Functions for interpolation and mathematical computations.
- **Modular Design**: Easy to extend with additional models or techniques.
- **Error Handling**: Comprehensive error management using Rust’s `Result` and `Error` traits.

## Installation

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/yourusername/OptionPricingModels.git
cd OptionPricingModels
cargo build --release
```

## Usage

An example of pricing an American put option using the Heston model:

```rust
use option_pricing_lib::data::InterestRateCurve;
use option_pricing_lib::models::heston::HestonModel;
use option_pricing_lib::regression::RegressionMethod;
use option_pricing_lib::traits::OptionPricingModel;
use option_pricing_lib::OptionType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interest_rate_curve = InterestRateCurve::new(vec![0.0, 1.0], vec![0.05, 0.05]);

    let heston_model = HestonModel {
        option_type: OptionType::Put,
        spot_price: 100.0,
        strike_price: 100.0,
        time_to_expiry: 1.0,
        initial_variance: 0.04,
        risk_free_rate_curve: interest_rate_curve,
        kappa: 2.0,
        theta: 0.04,
        sigma: 0.1,
        rho: -0.7,
        is_american: true,
        regression_method: RegressionMethod::LeastSquaresMonteCarlo,
        num_paths: 10000,
        num_steps: 50,
    };

    let price = heston_model.price()?;
    println!("Heston Model American Option Price: {:.4}", price);

    Ok(())
}
```

## Project Structure

```
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    ├── data
    │   ├── interest_rates.rs
    │   └── mod.rs
    ├── errors.rs
    ├── lib.rs
    ├── main.rs
    ├── models
    │   ├── binomial.rs
    │   ├── black_scholes.rs
    │   ├── heston.rs
    │   ├── intrinsic_value.rs
    │   └── mod.rs
    ├── regression
    │   ├── lsm.rs
    │   ├── mod.rs
    │   └── random_forest.rs
    ├── traits
    │   ├── mod.rs
    │   └── option_pricing.rs
    └── utils
        ├── interpolation.rs
        ├── math.rs
        └── mod.rs

7 directories, 21 files
```

- **data**: Contains data structures like `InterestRateCurve`.
- **models**: Implements various option pricing models.
- **regression**: Contains regression techniques for American option pricing.
- **traits**: Defines traits such as `OptionPricingModel`.
- **utils**: Provides utility functions for interpolation and mathematical computations.
- **errors.rs**: Defines custom error types.
- **lib.rs**: Exposes the library’s public API.
- **main.rs**: Example usage of the library.

## Technical Overview

### Option Pricing Models

#### Binomial Model

A discrete-time model for pricing options, accommodating both European and American styles. It constructs a binomial tree representing possible asset price movements over time.

**Key Components:**

- **Asset Price Tree**: Simulates upward and downward movements.
- **Risk-neutral Probability**: Probability of an upward movement.
- **Backward Induction**: Computes option values from maturity to present.

**Implementation Highlights:**

- Parameters: Spot price, strike price, volatility, risk-free rate, number of steps, option type, and style (American/European).
- Error Handling: Validates the number of steps and other inputs.
- American Option Handling: Compares continuation value with immediate exercise value.

#### Black-Scholes Model

An analytical model providing closed-form solutions for European option prices under the assumption of constant volatility and interest rates.

**Key Components:**

- **d1 and d2**: Intermediate variables in the Black-Scholes formula.
- **Cumulative Normal Distribution**: Calculates probabilities for option pricing.
- **Discounting**: Present value calculation using the risk-free rate.

**Implementation Highlights:**

- Parameters: Spot price, strike price, volatility, risk-free rate, time to expiry, and option type.
- Mathematical Computations: Implements the Black-Scholes formula accurately.
- Error Handling: Validates positive time to expiry.

#### Heston Model

A stochastic volatility model capturing the dynamic behavior of volatility, providing more realistic option pricing by allowing volatility to vary over time.

**Key Components:**

- **Stochastic Differential Equations**: Models the evolution of asset price and variance.
- **Correlation (rho)**: Captures the relationship between asset price and volatility.
- **Monte Carlo Simulation**: Generates multiple asset paths to estimate option prices.
- **Regression Techniques**: Applies LSM or Random Forest to handle American options.

**Implementation Highlights:**

- Parameters: Includes additional parameters like initial variance, mean reversion rate (kappa), long-term variance (theta), volatility of volatility (sigma), and correlation (rho).
- Simulation: Uses the Euler-Maruyama method for path generation.
- American Option Handling: Implements regression-based techniques to determine optimal exercise strategy.

#### Intrinsic Value

Calculates the immediate exercise value of an option without considering time value or future price movements.

**Key Components:**

- **Option Type**: Determines whether to calculate call or put intrinsic value.
- **Immediate Exercise**: Computes the difference between spot and strike prices.

**Implementation Highlights:**

- Simple and fast computation.
- Suitable for understanding basic option payoff structures.

### Regression Techniques

#### Least Squares Monte Carlo (LSM)

A regression-based method for pricing American options by estimating the continuation value at each step using least squares.

**Key Components:**

- **Regression Data Points**: Asset prices and corresponding continuation values.
- **Polynomial Basis Functions**: Typically second-degree polynomials for regression.
- **Backward Induction**: Updates option values based on regression predictions.

**Implementation Highlights:**

- Uses `ndarray` and `ndarray-linalg` for linear algebra operations.
- Fits a polynomial regression model to estimate continuation values.
- Implements a `RegressionModel` trait for prediction.

#### Random Forest Regression

An ensemble learning method using multiple decision trees to estimate the continuation value, providing robustness and flexibility.

**Key Components:**

- **Decision Trees**: Individual models within the forest.
- **Ensemble Averaging**: Combines predictions from multiple trees to improve accuracy.
- **Feature Importance**: Automatically captures non-linear relationships.

**Implementation Highlights:**

- Utilizes the `smartcore` library for machine learning algorithms.
- Configurable parameters for the random forest regressor.
- Implements a `RegressionModel` trait for prediction.

### Utilities

#### Interpolation

Provides linear interpolation for interest rate curves, ensuring accurate rate estimation at any given time.

**Key Components:**

- **Linear Interpolation Formula:**

  \[
  r(t) = r_0 + (r_1 - r_0) \times \frac{t - t_0}{t_1 - t_0}
  \]

- **Edge Cases Handling**: Returns boundary rates if time is outside the curve.

**Implementation Highlights:**

- Validates the integrity of the interest rate curve data.
- Efficiently locates the correct interval for interpolation.

#### Mathematical Functions

Includes functions like the cumulative normal distribution, essential for models like Black-Scholes.

**Key Components:**

- **Cumulative Normal Distribution (\( \Phi(x) \))**:

  \[
  \Phi(x) = \frac{1}{\sqrt{2\pi}} \int_{-\infty}^{x} e^{-t^2/2} dt
  \]

**Implementation Highlights:**

- Uses the `statrs` library for accurate statistical computations.
- Provides a simple interface for computing \( \Phi(x) \).

### Data Structures

#### InterestRateCurve

Represents the term structure of interest rates, allowing interpolation to obtain rates at any given time.

**Fields:**

- `times: Vec<f64>`: Time points (e.g., in years).
- `rates: Vec<f64>`: Corresponding interest rates.

**Implementation Highlights:**

- Ensures that `times` and `rates` vectors are of equal length and properly ordered.
- Provides a constructor for easy initialization.

### Traits and Errors

#### OptionPricingModel Trait

Defines a common interface for all option pricing models, enforcing the implementation of the `price` method.

```rust
pub trait OptionPricingModel {
    fn price(&self) -> Result<f64, OptionPricingError>;
}
```

#### Custom Error Handling

Uses the `thiserror` crate to define comprehensive error types, enhancing reliability and debuggability.

**Error Variants:**

- `InvalidInput(String)`: For invalid parameters.
- `ComputationError(String)`: For general computation issues.
- `RegressionError(String)`: Specific to regression failures.
- `InterpolationError(String)`: For interpolation-related errors.

## Mathematical Foundations

### Binomial Model

The Binomial Model constructs a discrete-time lattice to model the possible paths of an underlying asset's price. At each step, the price can move up by a factor \( u \) or down by a factor \( d \).

**Formulas:**

- **Up and Down Factors:**

  \[
  u = e^{\sigma \sqrt{\Delta t}}, \quad d = \frac{1}{u}
  \]

- **Risk-neutral Probability:**

  \[
  p = \frac{e^{r \Delta t} - d}{u - d}
  \]

- **Option Pricing via Backward Induction:**

  At each node, the option value is the discounted expected value of the option in the next step:

  \[
  V = e^{-r \Delta t} (p V_u + (1 - p) V_d)
  \]

  For American options, the value is the maximum of the continuation value and the intrinsic value.

### Black-Scholes Model

The Black-Scholes Model provides a closed-form solution for European option prices under the assumptions of constant volatility and interest rates.

**Formulas:**

- **d1 and d2:**

  \[
  d1 = \frac{\ln(S/K) + \left(r + \frac{\sigma^2}{2}\right) T}{\sigma \sqrt{T}}, \quad d2 = d1 - \sigma \sqrt{T}
  \]

- **Option Price:**

  \[
  C = S \Phi(d1) - K e^{-r T} \Phi(d2) \quad (\text{Call})
  \]

  \[
  P = K e^{-r T} \Phi(-d2) - S \Phi(-d1) \quad (\text{Put})
  \]

  Where:
  - \( S \) = Spot price
  - \( K \) = Strike price
  - \( r \) = Risk-free rate
  - \( \sigma \) = Volatility
  - \( T \) = Time to expiry
  - \( \Phi \) = Cumulative normal distribution function

### Heston Model

The Heston Model introduces stochastic volatility, allowing the volatility of the underlying asset to vary over time according to its own random process.

**Stochastic Differential Equations:**

- **Asset Price Process:**

  \[
  dS_t = \mu S_t dt + \sqrt{V_t} S_t dW_t^S
  \]

- **Variance Process:**

  \[
  dV_t = \kappa (\theta - V_t) dt + \sigma \sqrt{V_t} dW_t^V
  \]

  \[
  \text{Corr}(dW_t^S, dW_t^V) = \rho
  \]

**Parameters:**

- \( \kappa \): Mean reversion rate of variance
- \( \theta \): Long-term variance
- \( \sigma \): Volatility of volatility
- \( \rho \): Correlation between asset and variance processes

**Pricing Technique:**

- **Monte Carlo Simulation**: Generates multiple paths of asset and variance processes.
- **Regression for American Options**: Estimates continuation values to determine optimal exercise strategy.

### Regression Methods

#### Least Squares Monte Carlo (LSM)

LSM estimates the continuation value of an American option at each step by regressing the discounted future payoffs against basis functions of the asset price.

**Steps:**

1. **Simulate Paths**: Generate multiple asset price paths using Monte Carlo simulation.
2. **Initialize Payoffs**: At maturity, set option payoffs based on option type.
3. **Backward Induction**: Move backward through each time step, fitting a regression model to estimate continuation values.
4. **Exercise Decision**: Compare immediate exercise value with continuation value to decide on exercising the option.

**Mathematical Basis:**

The continuation value \( C \) is estimated as:

\[
C(S_t) = \mathbb{E}[e^{-r \Delta t} V_{t+\Delta t} | S_t]
\]

Where \( V_{t+\Delta t} \) is the option value at the next time step.

#### Random Forest Regression

An ensemble method that builds multiple decision trees and averages their predictions to estimate the continuation value.

**Advantages:**

- **Non-linear Relationships**: Captures complex dependencies between asset prices and continuation values.
- **Robustness**: Reduces overfitting compared to single decision trees.

**Implementation Notes:**

- Utilizes the `smartcore` library for constructing and training the random forest regressor.
- Handles high-dimensional data effectively.

## Design Choices

### Modular Design

The project is structured into distinct modules (`models`, `regression`, `utils`, etc.) to promote separation of concerns, ease of maintenance, and scalability. Each module encapsulates specific functionalities, making the codebase organized and manageable.

### Trait-based Abstraction

Using Rust’s trait system (`OptionPricingModel`, `Regression`, `RegressionModel`), the library achieves polymorphism, allowing different models and regression techniques to be interchangeable and extendable without modifying existing code.

### Error Handling

Comprehensive error management is implemented using the `thiserror` crate, providing clear and descriptive error messages. The use of Rust’s `Result` type ensures that errors are handled gracefully, enhancing the library’s robustness.

### Performance Optimizations

- **Parallelism**: Monte Carlo simulations and regression fits can be parallelized for improved performance.
- **Efficient Data Structures**: Utilizes `ndarray` for efficient numerical computations.
- **Minimal Library Usage**: Relies on lightweight and performant libraries to keep overhead low.

## Library Dependencies

- **ndarray**: For numerical operations and array manipulations.
- **ndarray-linalg**: Provides linear algebra routines essential for regression.
- **rand** & **rand_distr**: For generating random numbers in simulations.
- **smartcore**: Implements machine learning algorithms like Random Forest.
- **statrs**: For statistical functions, including the cumulative normal distribution.
- **thiserror**: Facilitates easy and descriptive error definitions.

## Performance Indicators

- **Simulation Speed**: Optimized Monte Carlo simulations handle thousands of paths efficiently.
- **Regression Accuracy**: Both LSM and Random Forest provide accurate estimations of continuation values.
- **Scalability**: The library can handle increasing complexity (e.g., more paths, steps) with reasonable performance due to efficient algorithms and data handling.

## Implementation Methods

### Numerical Methods

- **Euler-Maruyama Method**: Used in the Heston model to discretize and simulate the stochastic differential equations governing asset prices and variance.
- **Least Squares Regression**: Applied in LSM to fit continuation values based on basis functions.

### Simulation Techniques

- **Monte Carlo Simulation**: Generates a multitude of possible asset price paths to estimate option prices, particularly effective for complex models like Heston.
- **Regression-based Optimization**: Enhances the ability to price American options by determining optimal exercise points through regression.

## Contributing

Contributions are welcome! Please open issues or submit pull requests for enhancements, bug fixes, or new features.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m 'Add YourFeature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## References

1. **Option Pricing Models:**
   - Cox, J. C., Ross, S. A., & Rubinstein, M. (1979). *Option Pricing: A Simplified Approach*. Journal of Financial Economics.
   - Black, F., & Scholes, M. (1973). *The Pricing of Options and Corporate Liabilities*. Journal of Political Economy.
   - Heston, S. L. (1993). *A Closed-Form Solution for Options with Stochastic Volatility with Applications to Bond and Currency Options*. The Review of Financial Studies.

2. **Regression Techniques:**
   - Longstaff, F. A., & Schwartz, E. S. (2001). *Valuing American Options by Simulation: A Simple Least-Squares Approach*. Review of Financial Studies.
   - Breiman, L. (2001). *Random Forests*. Machine Learning.

3. **Numerical Methods:**
   - Higham, D. J. (2001). *An Algorithmic Introduction to Numerical Simulation of Stochastic Differential Equations*. SIAM Review.

4. **Rust Programming:**
   - The Rust Programming Language. *https://www.rust-lang.org/learn*

---

This README provides a comprehensive overview of the `OptionPricingModels` project, detailing its structure, functionalities, mathematical foundations, and design choices. Whether you are a developer looking to utilize the library or a researcher interested in the underlying models, this documentation aims to equip you with the necessary information to effectively engage with the project.