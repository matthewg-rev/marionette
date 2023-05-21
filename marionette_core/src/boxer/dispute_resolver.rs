use crate::boxer::dispute_resolver::dispute_rule::DisputeRule;

#[macro_use]
pub mod dispute_rule;

pub trait DisputeResolverTrait<T: Clone + PartialEq> {
    fn add_rule(&mut self, rule: DisputeRule<T>);
    fn add_rules(&mut self, rules: Vec<DisputeRule<T>>);

    fn resolve(&self, original: T, crosser: T) -> Option<T>;
    fn resolve_tuple(&self, pair: (T, T)) -> Option<T>;

    /// Resolves a vector of tuples.
    /// # Arguments
    /// * `pairs` - The vector of tuples to resolve.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::dispute_rule::DisputeRule;
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    ///
    /// let mut resolver = SimpleDisputeResolver::default();
    /// resolver.add_rule(DisputeRule::new(2, 2, 4, false));
    /// let results = resolver.resolve_tuple_vec(vec![
    ///     (2, 2), (2, 3),
    ///     (3, 2), (3, 3)
    /// ]);
    /// assert_eq!(results, vec![Some(4), None, None, None]);
    /// ```
    fn resolve_tuple_vec(&self, pairs: Vec<(T, T)>) -> Vec<Option<T>> {
        let mut results = Vec::new();
        for pair in pairs {
            results.push(self.resolve_tuple(pair));
        }
        results
    }
}

pub struct SimpleDisputeResolver<T: Clone + PartialEq> {
    pub rules: Vec<DisputeRule<T>>,
}

impl Default for SimpleDisputeResolver<i16> {
    /// Creates a blank dispute resolver for i16 values.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    /// let resolver = SimpleDisputeResolver::<i16>::default();
    /// ```
    fn default() -> Self {
        SimpleDisputeResolver::new()
    }
}

impl<T: Clone + PartialEq> SimpleDisputeResolver<T> {
    /// Creates a blank dispute resolver.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    /// let resolver = SimpleDisputeResolver::<i16>::new();
    /// ```
    pub fn new() -> SimpleDisputeResolver<T> {
        SimpleDisputeResolver {
            rules: Vec::new(),
        }
    }
}

impl<T: Clone + PartialEq> DisputeResolverTrait<T> for SimpleDisputeResolver<T> {
    /// Adds a rule to the dispute resolver.
    /// # Arguments
    /// * `rule` - The rule to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::dispute_rule::DisputeRule;
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    ///
    /// let mut resolver = SimpleDisputeResolver::default();
    /// resolver.add_rule(DisputeRule::new(2, 2, 4, false));
    /// ```
    fn add_rule(&mut self, rule: DisputeRule<T>) {
        self.rules.push(rule);
    }

    /// Adds a vector of rules to the dispute resolver.
    /// # Arguments
    /// * `rules` - The rules to add.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::dispute_rule::DisputeRule;
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    ///
    /// let mut resolver: SimpleDisputeResolver<i16> = SimpleDisputeResolver::new();
    /// let rules = vec![
    ///     DisputeRule::new(2, 2, 4, false),
    ///     DisputeRule::new(2, 3, 5, false)
    /// ];
    /// resolver.add_rules(rules);
    /// ```
    fn add_rules(&mut self, rules: Vec<DisputeRule<T>>) {
        for rule in rules {
            self.add_rule(rule);
        }
    }

    /// Attempts to resolve a dispute between two values.
    /// # Arguments
    /// * `original` - The original value.
    /// * `crosser` - The crosser value.
    /// # Returns
    /// * `Some(T)` - The resolved value.
    /// * `None` - If no rule was found to resolve the dispute.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::dispute_rule::DisputeRule;
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    ///
    /// let mut resolver = SimpleDisputeResolver::default();
    /// resolver.add_rule(DisputeRule::new(2, 2, 4, false));
    /// let result = resolver.resolve(2, 2);
    /// assert_eq!(result, Some(4));
    /// ```
    fn resolve(&self, original: T, crosser: T) -> Option<T> {
        for rule in &self.rules {
            if let Some(result) = rule.attempt_apply(original.clone(), crosser.clone()) {
                return Some(result);
            }
        }
        None
    }

    /// Attempts to resolve a dispute between a pair of values.
    /// # Arguments
    /// * `pair` - The pair of values.
    /// # Returns
    /// * `Some(T)` - The resolved value.
    /// * `None` - If no rule was found to resolve the dispute.
    /// # Example
    /// ```
    /// use marionette_core::boxer::dispute_resolver::dispute_rule::DisputeRule;
    /// use marionette_core::boxer::dispute_resolver::SimpleDisputeResolver;
    ///
    /// let mut resolver = SimpleDisputeResolver::default();
    /// resolver.add_rule(DisputeRule::new(2, 2, 4, false));
    /// let result = resolver.resolve_tuple((2, 2));
    /// assert_eq!(result, Some(4));
    /// ```
    fn resolve_tuple(&self, pair: (T, T)) -> Option<T> {
        self.resolve(pair.0, pair.1)
    }
}

