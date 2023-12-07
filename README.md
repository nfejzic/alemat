[![Build](https://img.shields.io/github/actions/workflow/status/nfejzic/alemat/build.yml?logo=github&label=Build)](https://github.com/nfejzic/alemat/actions/workflows/build.yml)
[![CI](https://img.shields.io/github/actions/workflow/status/nfejzic/alemat/ci.yml?logo=github&label=CI)](https://github.com/nfejzic/alemat/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/docsrs/alemat?logo=docs.rs&label=Docs)](https://docs.rs/alemat/latest/alemat/)
[![Crates](https://img.shields.io/crates/v/alemat?logo=rust)](https://crates.io/crates/alemat)

# alemat - MathML Builder

## What is "alemat"?

Al-Alemat is arabic for tags. MathML is markup language that uses tags (similar
to other markup languages such as XML or HTML) to build mathematic notation.

## What is `alemat` for?

`alemat` is a Rust crate for building MathMl documents. The goal is to provide
type-safe and ergonomic API for building and rendering MathMl elements.

## Examples: 

In general, you can check out the `tests/` directory for API examples and
`tests/snapshots/` directory for the rendered output.

Here are some of the examples: 

```rust
let output = MathMl::with_content(
    Radical::builder()
        .index("2")
        .content(alemat::children![
            Num::from(1),
            Operator::from("+"),
            SubSup::builder()
                .base(Ident::from("n"))
                .subscript(Num::from(2))
                .supscript(Num::from(3))
                .build(),
        ])
        .build(),
)
.render();
```

This is rendered out to: 

```html
<math>
  <msqrt>
    <mn>
      1
    </mn>
    <mo>
      +
    </mo>
    <msubsup>
      <mi>
        n
      </mi>
      <mn>
        2
      </mn>
      <mn>
        3
      </mn>
    </msubsup>
  </msqrt>
</math>
```

which looks like this: 

$`\sqrt[2]{1 + n_{2}^{3}}`$

The crate exposes some macros for better ergonomics when building elements. For
example the `children!` macro can combine arbitrary elements into a single
array. Internally, this is done by converting each element into the `Element`
enum, and storing that in the list.

There's the `row!` macro for building an `mrow` of elements. And there are also
macros for creating a `table_row!` and `table!`. For example: 

```rust
let out = MathMl::with_content(alemat::children![
    Frac::builder()
        .num(Ident::from("A"))
        .denom(Num::from(2))
        .build(),
    Operator::eq(),
    alemat::row![
        Operator::lparens(),
        alemat::table![
            [Num::from(1), Num::from(2), Num::from(3)],
            [Num::from(4), Num::from(5), Num::from(6)],
            [Num::from(7), Num::from(8), Num::from(9)],
        ],
        Operator::rparens(),
    ]
])
.render();
```

This generates the MathMl for a matrix:

```html
<math>
  <mfrac>
    <mi>
      A
    </mi>
    <mn>
      2
    </mn>
  </mfrac>
  <mo>
    =
  </mo>
  <mrow>
    <mo>
      (
    </mo>
    <mtable>
      <mtr>
        <mtd>
          <mn>
            1
          </mn>
        </mtd>
        <mtd>
          <mn>
            2
          </mn>
        </mtd>
        <mtd>
          <mn>
            3
          </mn>
        </mtd>
      </mtr>
      <mtr>
        <mtd>
          <mn>
            4
          </mn>
        </mtd>
        <mtd>
          <mn>
            5
          </mn>
        </mtd>
        <mtd>
          <mn>
            6
          </mn>
        </mtd>
      </mtr>
      <mtr>
        <mtd>
          <mn>
            7
          </mn>
        </mtd>
        <mtd>
          <mn>
            8
          </mn>
        </mtd>
        <mtd>
          <mn>
            9
          </mn>
        </mtd>
      </mtr>
    </mtable>
    <mo>
      )
    </mo>
  </mrow>
</math>
```

which looks like this: 

$`A = 
\begin{pmatrix}
1 & 2 & 3\\
4 & 5 & 6\\
7 & 8 & 9\\
\end{pmatrix}`$

# License

This project is licensed under the Apache 2.0 license. See the
[LICENSE](LICENSE) file for more details.
