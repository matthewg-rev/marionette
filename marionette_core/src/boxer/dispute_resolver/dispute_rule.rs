pub struct DisputeRule<T: Clone + PartialEq> {
    pub original: T,
    pub crosser: T,
    pub result: T,
    pub ordered: bool
}

/// Shortform for creating a dispute rule
/// # Arguments
/// * `original` - The original value
/// * `crosser` - The value that crosses the original
/// * `result` - The result of the dispute
/// * `ordered` - Whether the original and crosser must be in the same order
/// # Examples
/// ```
/// let new_rule = rule!(2, 2, 4, false);
/// ```
macro_rules! rule {
    ($original:expr, $crosser:expr, $result:expr, $ordered:expr) => {
        DisputeRule::new($original, $crosser, $result, $ordered)
    };
}

/// Shortform for creating a vector of dispute rules
/// # Arguments
/// * Variadic arguments of tuples of the form (original, crosser, result, ordered)
///     - `original` - The original value
///     - `crosser` - The value that crosses the original
///     - `result` - The result of the dispute
///     - `ordered` - Whether the original and crosser must be in the same order
/// # Examples
/// ```
/// let new_rules = rules!(
///     (2, 2, 4, false),
///     (1, 1, 2, false),
///     (0, 0, 0, false)
/// );
/// ```
macro_rules! rules {
    ($(($original:expr, $crosser:expr, $result:expr, $ordered:expr)),*) => {
        vec![$(rule!($original, $crosser, $result, $ordered)),*]
    };
}

/// Shortform for creating a dispute rule with ordered = false
/// # Arguments
/// * `original` - The original value
/// * `crosser` - The value that crosses the original
/// * `result` - The result of the dispute
/// # Examples
/// ```
/// let new_rule = rule_unord!(2, 2, 4);
/// ```
macro_rules! rule_unord {
    ($original:expr, $crosser:expr, $result:expr) => {
        DisputeRule::new($original, $crosser, $result, false)
    };
}

/// Shortform for creating a vector of dispute rules with ordered = false
/// # Arguments
/// * Variadic arguments of tuples of the form (original, crosser, result)
///     - `original` - The original value
///     - `crosser` - The value that crosses the original
///     - `result` - The result of the dispute
/// # Examples
/// ```
/// let new_rules = rules_unord!(
///     (2, 2, 4),
///     (1, 1, 2),
///     (0, 0, 0)
/// );
macro_rules! rules_unord {
    ($(($original:expr, $crosser:expr, $result:expr)),*) => {
        vec![$(rule_unord!($original, $crosser, $result)),*]
    };
}

/// Shortform for creating a dispute rule with ordered = true
/// # Arguments
/// * `original` - The original value
/// * `crosser` - The value that crosses the original
/// * `result` - The result of the dispute
/// # Examples
/// ```
/// let new_rule = rule_ord!(2, 2, 4);
/// ```
macro_rules! rule_ord {
    ($original:expr, $crosser:expr, $result:expr) => {
        DisputeRule::new($original, $crosser, $result, true)
    };
}

/// Shortform for creating a vector of dispute rules with ordered = true
/// # Arguments
/// * Variadic arguments of tuples of the form (original, crosser, result)
///     - `original` - The original value
///     - `crosser` - The value that crosses the original
///     - `result` - The result of the dispute
/// # Examples
/// ```
/// let new_rules = rules_ord!(
///     (2, 2, 4),
///     (1, 1, 2),
///     (0, 0, 0)
/// );
/// ```
macro_rules! rules_ord {
    ($(($original:expr, $crosser:expr, $result:expr)),*) => {
        vec![$(rule_ord!($original, $crosser, $result)),*]
    };
}

impl <T: Clone + PartialEq> DisputeRule<T> {
    /// Creates a new dispute rule
    /// # Arguments
    /// * `original` - The original value
    /// * `crosser` - The value that crosses the original
    /// * `result` - The result of the dispute
    /// * `ordered` - Whether the original and crosser must be in the same order
    /// # Examples
    /// ```
    /// let new_rule = DisputeRule::new(2, 2, 4, false);
    /// ```
    pub fn new(original: T, crosser: T, result: T, ordered: bool) -> DisputeRule<T> {
        DisputeRule {
            original, crosser,
            result, ordered
        }
    }

    /// Returns whether the rule can be applied to the given values
    /// # Arguments
    /// * `original` - The original value
    /// * `crosser` - The value that crosses the original
    /// # Examples
    /// ```
    /// let new_rule = DisputeRule::new(2, 2, 4, false);
    /// assert!(new_rule.can_be_applied(2, 2));
    /// assert!(!new_rule.can_be_applied(2, 3));
    /// ```
    pub fn can_be_applied(&self, original: T, crosser: T) -> bool {
        if self.ordered {
            original == self.original && crosser == self.crosser
        } else {
            (original == self.original && crosser == self.crosser) ||
            (original == self.crosser && crosser == self.original)
        }
    }

    /// Returns whether the rule can be applied to the given pair
    /// # Arguments
    /// * `pair` - A tuple of the form (original, crosser)
    /// # Examples
    /// ```
    /// let new_rule = DisputeRule::new(2, 2, 4, false);
    /// assert!(new_rule.can_be_applied_tuple((2, 2)));
    /// assert!(!new_rule.can_be_applied_tuple((2, 3)));
    /// ```
    pub fn can_be_applied_tuple(&self, pair: (T, T)) -> bool {
        self.can_be_applied(pair.0, pair.1)
    }

    /// Returns the result of the rule if it can be applied to the given values
    /// # Arguments
    /// * `original` - The original value
    /// * `crosser` - The value that crosses the original
    /// # Examples
    /// ```
    /// let new_rule = DisputeRule::new(2, 2, 4, false);
    /// assert_eq!(new_rule.attempt_apply(2, 2), Some(4));
    /// assert_eq!(new_rule.attempt_apply(2, 3), None);
    /// ```
    pub fn attempt_apply(&self, original: T, crosser: T) -> Option<T> {
        if self.can_be_applied(original, crosser) {
            Some(self.result.clone())
        } else {
            None
        }
    }

    /// Returns the result of the rule if it can be applied to the given pair
    /// # Arguments
    /// * `pair` - A tuple of the form (original, crosser)
    /// # Examples
    /// ```
    /// let new_rule = DisputeRule::new(2, 2, 4, false);
    /// assert_eq!(new_rule.attempt_apply_tuple((2, 2)), Some(4));
    /// assert_eq!(new_rule.attempt_apply_tuple((2, 3)), None);
    /// ```
    pub fn attempt_apply_tuple(&self, pair: (T, T)) -> Option<T> {
        self.attempt_apply(pair.0, pair.1)
    }
}