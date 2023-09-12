#Interesting tid-bits

    1. RefCell is checked @ runtime (which is why the Docs refer to it as a mutable memory location with ***dynamically***  checked borrow rules)
    2. RefCell lets you check at runtime if "any one else" is mutating at the same time (pssst... so they're great for graphs and trees)
