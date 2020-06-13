# Grammar Fuzzer

A grammar fuzzer following the ["Fuzzing with Grammars"](https://www.fuzzingbook.org/html/Grammars.html) and ["Efficient Grammar Fuzzing"](https://www.fuzzingbook.org/html/GrammarFuzzer.html) chapters of ["The Fuzzing Book"](https://www.fuzzingbook.org/) as a starting point, written in Rust. 

- 🛑 This project has not been endorsed by the authors of [The Fuzzing Book](https://www.fuzzingbook.org/).
- ❗ This is not a precise reimplementation of the mentioned chapters, and I have deviated were it's convenient.

## TODO
- [ ] Improve the exploration strategies
- [ ] Improve the runtime of the grammar fuzzer
- [ ] Add unit test
- [ ] Add comments
- [ ] Better error handling
- [ ] Cache computation

# References and Attributions

## [The Fuzzing Book](https://www.fuzzingbook.org/)

From The Fuzzing Book website https://www.fuzzingbook.org/: 

> The content of this project is licensed under the [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License](https://creativecommons.org/licenses/by-nc-sa/4.0/). The source code that is part of the content, as well as the source code used to format and display that content is licensed under the [MIT License](https://github.com/github/choosealicense.com/blob/gh-pages/LICENSE.md). Last change: 2019-12-21 16:38:57+01:00

Copy of the MIT license from https://github.com/uds-se/fuzzingbook/blob/master/LICENSE.md

```txt
Copyright (c) 2018 Saarland University, CISPA, authors, and contributors

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

### [Efficient Grammar Fuzzing](https://www.fuzzingbook.org/html/GrammarFuzzer.html)

> Andreas Zeller, Rahul Gopinath, Marcel Böhme, Gordon Fraser, and Christian Holler: "Efficient Grammar Fuzzing". In Andreas Zeller, Rahul Gopinath, Marcel Böhme, Gordon Fraser, and Christian Holler (eds.), "The Fuzzing Book", https://www.fuzzingbook.org/html/GrammarFuzzer.html. Retrieved 2019-12-21 16:38:57+01:00.

<details><summary>details</summary>

```
@incollection{fuzzingbook2019:GrammarFuzzer,
    author = {Andreas Zeller and Rahul Gopinath and Marcel B{\"o}hme and Gordon Fraser and Christian Holler},
    booktitle = {The Fuzzing Book},
    title = {Efficient Grammar Fuzzing},
    year = {2019},
    publisher = {Saarland University},
    howpublished = {\url{https://www.fuzzingbook.org/html/GrammarFuzzer.html}},
    note = {Retrieved 2019-12-21 16:38:57+01:00},
    url = {https://www.fuzzingbook.org/html/GrammarFuzzer.html},
    urldate = {2019-12-21 16:38:57+01:00}
}
```

</details>

### [Fuzzing with Grammars](https://www.fuzzingbook.org/html/Grammars.html)


> Andreas Zeller, Rahul Gopinath, Marcel Böhme, Gordon Fraser, and Christian Holler: "Fuzzing with Grammars". In Andreas Zeller, Rahul Gopinath, Marcel Böhme, Gordon Fraser, and Christian Holler (eds.), "The Fuzzing Book", https://www.fuzzingbook.org/html/Grammars.html. Retrieved 2019-12-21 16:38:57+01:00.

<details><summary>details</summary>

```
@incollection{fuzzingbook2019:Grammars,
    author = {Andreas Zeller and Rahul Gopinath and Marcel B{\"o}hme and Gordon Fraser and Christian Holler},
    booktitle = {The Fuzzing Book},
    title = {Fuzzing with Grammars},
    year = {2019},
    publisher = {Saarland University},
    howpublished = {\url{https://www.fuzzingbook.org/html/Grammars.html}},
    note = {Retrieved 2019-12-21 16:38:57+01:00},
    url = {https://www.fuzzingbook.org/html/Grammars.html},
    urldate = {2019-12-21 16:38:57+01:00}
}
```

</details>
