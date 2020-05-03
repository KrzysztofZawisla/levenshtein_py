# levenshtein_py
Library for Python3 that uses the levenshtein-rs.

## How to import

```py
from levenshtein_py import LevenshteinPy
```

## Signature and how to use

```py
/**
  * Filter function returns filted collection by Levenshtein Algorithm.
  * @input str.
  * @distance int (cant be lower than 0).
  * @collection List[str] - collection to search over.
  * @returns List[str] - filtered collection.
*/
LevenshteinPy.filter(input: str, distance: int, collection: List[str]) -> List[str]
```

## Used libraries

<https://github.com/wooorm/levenshtein-rs>  
<https://github.com/PyO3/pyo3>