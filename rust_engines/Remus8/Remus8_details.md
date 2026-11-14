# Remus8 source reference

Read [the engine overview](remus8.md) for the algorithms, options, and measurement limits. This reference shows every source line, including comments and spacing, with structural annotations. Original source comments carry the algorithm-specific reasoning. Source identity is not a behavior test.

## Line-by-line source reference

### attacks.rs

Bitboard rays, piece attacks, and king-safety queries.

[Open source](src/attacks.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Find out what each piece reaches and whether a square is attacked.</code> | Source explanation: Find out what each piece reaches and whether a square is attacked. |
| 2 | <code>//!</code> | Source explanation: |
| 3 | <code>//! The tables hold, for each square, a bitboard of what a knight, king, or pawn on that</code> | Source explanation: The tables hold, for each square, a bitboard of what a knight, king, or pawn on that |
| 4 | <code>//! square reaches, and the squares along each of the eight rays. The compiler fills them</code> | Source explanation: square reaches, and the squares along each of the eight rays. The compiler fills them |
| 5 | <code>//! in while it builds the program, so they cost nothing when the program runs.</code> | Source explanation: in while it builds the program, so they cost nothing when the program runs. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>use crate::board::{BISHOP, KING, KNIGHT, PAWN, Position, QUEEN, ROOK};</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code></code> | Blank line separating related declarations or steps. |
| 9 | <code>const KNIGHT&#95;JUMPS: &#91;(i32, i32); 8&#93; = &#91;</code> | Define KNIGHT&#95;JUMPS, including its explicit type and initial value. |
| 10 | <code>    (-2, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 11 | <code>    (-2, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 12 | <code>    (-1, -2),</code> | Continue the enclosing declaration or initialization expression. |
| 13 | <code>    (-1, 2),</code> | Continue the enclosing declaration or initialization expression. |
| 14 | <code>    (1, -2),</code> | Continue the enclosing declaration or initialization expression. |
| 15 | <code>    (1, 2),</code> | Continue the enclosing declaration or initialization expression. |
| 16 | <code>    (2, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 17 | <code>    (2, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 18 | <code>&#93;;</code> | Close the preceding block, expression, call, or collection. |
| 19 | <code></code> | Blank line separating related declarations or steps. |
| 20 | <code>// The rays: the first four raise the square number and the last four lower it.</code> | Source explanation: The rays: the first four raise the square number and the last four lower it. |
| 21 | <code>// Rooks use rays 0, 1, 4, and 5, and bishops use rays 2, 3, 6, and 7.</code> | Source explanation: Rooks use rays 0, 1, 4, and 5, and bishops use rays 2, 3, 6, and 7. |
| 22 | <code>const DIRECTIONS: &#91;(i32, i32); 8&#93; = &#91;</code> | Define DIRECTIONS, including its explicit type and initial value. |
| 23 | <code>    (1, 0),</code> | Continue the enclosing declaration or initialization expression. |
| 24 | <code>    (0, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 25 | <code>    (1, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 26 | <code>    (1, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 27 | <code>    (-1, 0),</code> | Continue the enclosing declaration or initialization expression. |
| 28 | <code>    (0, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 29 | <code>    (-1, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 30 | <code>    (-1, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 31 | <code>&#93;;</code> | Close the preceding block, expression, call, or collection. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>const fn bit&#95;if&#95;on&#95;board(row: i32, col: i32) -&gt; u64 {</code> | Declare bit&#95;if&#95;on&#95;board(); its parameters and return type define the interface. |
| 34 | <code>    if row &gt;= 0 &amp;&amp; row &lt; 8 &amp;&amp; col &gt;= 0 &amp;&amp; col &lt; 8 {</code> | Test the condition and execute this branch only when it matches. |
| 35 | <code>        1&#95;u64 &lt;&lt; (row &#42; 8 + col)</code> | Evaluate or continue this expression in bit&#95;if&#95;on&#95;board(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 37 | <code>        0</code> | Evaluate or continue this expression in bit&#95;if&#95;on&#95;board(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 39 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 40 | <code></code> | Blank line separating related declarations or steps. |
| 41 | <code>const fn build&#95;steps(steps: &amp;&#91;(i32, i32); 8&#93;) -&gt; &#91;u64; 64&#93; {</code> | Declare build&#95;steps(); its parameters and return type define the interface. |
| 42 | <code>    let mut table = &#91;0; 64&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 43 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 44 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 45 | <code>        let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>        let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>        let mut i = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 48 | <code>        while i &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 49 | <code>            table&#91;square&#93; &#124;= bit&#95;if&#95;on&#95;board(row + steps&#91;i&#93;.0, col + steps&#91;i&#93;.1);</code> | Update the named state using the expression and assignment operator shown. |
| 50 | <code>            i += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 51 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 54 | <code>    table</code> | Evaluate or continue this expression in build&#95;steps(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code></code> | Blank line separating related declarations or steps. |
| 57 | <code>const fn build&#95;rays() -&gt; &#91;&#91;u64; 64&#93;; 8&#93; {</code> | Declare build&#95;rays(); its parameters and return type define the interface. |
| 58 | <code>    let mut table = &#91;&#91;0; 64&#93;; 8&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 59 | <code>    let mut direction = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 60 | <code>    while direction &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 61 | <code>        let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 62 | <code>        while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 63 | <code>            let mut row = (square / 8) as i32 + DIRECTIONS&#91;direction&#93;.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 64 | <code>            let mut col = (square % 8) as i32 + DIRECTIONS&#91;direction&#93;.1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 65 | <code>            while bit&#95;if&#95;on&#95;board(row, col) != 0 {</code> | Repeat this block while its condition remains true. |
| 66 | <code>                table&#91;direction&#93;&#91;square&#93; &#124;= bit&#95;if&#95;on&#95;board(row, col);</code> | Update the named state using the expression and assignment operator shown. |
| 67 | <code>                row += DIRECTIONS&#91;direction&#93;.0;</code> | Update the named state using the expression and assignment operator shown. |
| 68 | <code>                col += DIRECTIONS&#91;direction&#93;.1;</code> | Update the named state using the expression and assignment operator shown. |
| 69 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 70 | <code>            square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 71 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 72 | <code>        direction += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 73 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code>    table</code> | Evaluate or continue this expression in build&#95;rays(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 76 | <code></code> | Blank line separating related declarations or steps. |
| 77 | <code>const fn build&#95;pawn&#95;attacks() -&gt; &#91;&#91;u64; 64&#93;; 2&#93; {</code> | Declare build&#95;pawn&#95;attacks(); its parameters and return type define the interface. |
| 78 | <code>    let mut table = &#91;&#91;0; 64&#93;; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 79 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 80 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 81 | <code>        let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 82 | <code>        let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 83 | <code>        // Row 0 is rank 8, so a White pawn attacks toward row 0</code> | Source explanation: Row 0 is rank 8, so a White pawn attacks toward row 0 |
| 84 | <code>        table&#91;0&#93;&#91;square&#93; = bit&#95;if&#95;on&#95;board(row - 1, col - 1) &#124; bit&#95;if&#95;on&#95;board(row - 1, col + 1);</code> | Update the named state using the expression and assignment operator shown. |
| 85 | <code>        table&#91;1&#93;&#91;square&#93; = bit&#95;if&#95;on&#95;board(row + 1, col - 1) &#124; bit&#95;if&#95;on&#95;board(row + 1, col + 1);</code> | Update the named state using the expression and assignment operator shown. |
| 86 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 87 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 88 | <code>    table</code> | Evaluate or continue this expression in build&#95;pawn&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 89 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 90 | <code></code> | Blank line separating related declarations or steps. |
| 91 | <code>pub static KNIGHT&#95;ATTACKS: &#91;u64; 64&#93; = build&#95;steps(&amp;KNIGHT&#95;JUMPS);</code> | Define KNIGHT&#95;ATTACKS, including its explicit type and initial value. |
| 92 | <code>pub static KING&#95;ATTACKS: &#91;u64; 64&#93; = build&#95;steps(&amp;DIRECTIONS);</code> | Define KING&#95;ATTACKS, including its explicit type and initial value. |
| 93 | <code>pub static PAWN&#95;ATTACKS: &#91;&#91;u64; 64&#93;; 2&#93; = build&#95;pawn&#95;attacks();</code> | Define PAWN&#95;ATTACKS, including its explicit type and initial value. |
| 94 | <code>static RAYS: &#91;&#91;u64; 64&#93;; 8&#93; = build&#95;rays();</code> | Define RAYS, including its explicit type and initial value. |
| 95 | <code></code> | Blank line separating related declarations or steps. |
| 96 | <code>const fn build&#95;queen&#95;lines() -&gt; &#91;u64; 64&#93; {</code> | Declare build&#95;queen&#95;lines(); its parameters and return type define the interface. |
| 97 | <code>    let rays = build&#95;rays();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 98 | <code>    let mut table = &#91;0; 64&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 99 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 100 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 101 | <code>        let mut direction = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>        while direction &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 103 | <code>            table&#91;square&#93; &#124;= rays&#91;direction&#93;&#91;square&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 104 | <code>            direction += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 105 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 106 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 107 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 108 | <code>    table</code> | Evaluate or continue this expression in build&#95;queen&#95;lines(); a final expression without a semicolon supplies its block&#x27;s value. |
| 109 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 110 | <code></code> | Blank line separating related declarations or steps. |
| 111 | <code>/// For each square, the squares on its rank, file, and two diagonals. A piece that</code> | Source explanation: For each square, the squares on its rank, file, and two diagonals. A piece that |
| 112 | <code>/// stands off the lines of its own king cannot uncover that king by moving.</code> | Source explanation: stands off the lines of its own king cannot uncover that king by moving. |
| 113 | <code>pub static QUEEN&#95;LINES: &#91;u64; 64&#93; = build&#95;queen&#95;lines();</code> | Define QUEEN&#95;LINES, including its explicit type and initial value. |
| 114 | <code></code> | Blank line separating related declarations or steps. |
| 115 | <code>/// The squares along a ray that raises the square number, up to the first piece.</code> | Source explanation: The squares along a ray that raises the square number, up to the first piece. |
| 116 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 117 | <code>fn rising&#95;ray(direction: usize, square: usize, occupied: u64) -&gt; u64 {</code> | Declare rising&#95;ray(); its parameters and return type define the interface. |
| 118 | <code>    let ray = RAYS&#91;direction&#93;&#91;square&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 119 | <code>    // The top bit stands in for a piece on h1, square 63. No rising ray goes past h1,</code> | Source explanation: The top bit stands in for a piece on h1, square 63. No rising ray goes past h1, |
| 120 | <code>    // so its table entry is empty and an open ray keeps all of its squares. The</code> | Source explanation: so its table entry is empty and an open ray keeps all of its squares. The |
| 121 | <code>    // lookup then needs no test for &quot;no piece on the ray&quot;, a test the processor</code> | Source explanation: lookup then needs no test for &quot;no piece on the ray&quot;, a test the processor |
| 122 | <code>    // predicts badly.</code> | Source explanation: predicts badly. |
| 123 | <code>    let first = ((ray &amp; occupied) &#124; (1 &lt;&lt; 63)).trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 124 | <code>    ray ^ RAYS&#91;direction&#93;&#91;first&#93;</code> | Evaluate or continue this expression in rising&#95;ray(); a final expression without a semicolon supplies its block&#x27;s value. |
| 125 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 126 | <code></code> | Blank line separating related declarations or steps. |
| 127 | <code>/// The squares along a ray that lowers the square number, up to the first piece.</code> | Source explanation: The squares along a ray that lowers the square number, up to the first piece. |
| 128 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 129 | <code>fn falling&#95;ray(direction: usize, square: usize, occupied: u64) -&gt; u64 {</code> | Declare falling&#95;ray(); its parameters and return type define the interface. |
| 130 | <code>    let ray = RAYS&#91;direction&#93;&#91;square&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 131 | <code>    // The low bit stands in for a piece on a8, square 0: no falling ray goes past a8</code> | Source explanation: The low bit stands in for a piece on a8, square 0: no falling ray goes past a8 |
| 132 | <code>    let last = 63 - ((ray &amp; occupied) &#124; 1).leading&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 133 | <code>    ray ^ RAYS&#91;direction&#93;&#91;last&#93;</code> | Evaluate or continue this expression in falling&#95;ray(); a final expression without a semicolon supplies its block&#x27;s value. |
| 134 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 135 | <code></code> | Blank line separating related declarations or steps. |
| 136 | <code>/// The squares a rook on the square reaches, including the first piece on each line.</code> | Source explanation: The squares a rook on the square reaches, including the first piece on each line. |
| 137 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 138 | <code>pub fn rook&#95;attacks(square: usize, occupied: u64) -&gt; u64 {</code> | Declare rook&#95;attacks(); its parameters and return type define the interface. |
| 139 | <code>    rising&#95;ray(0, square, occupied)</code> | Evaluate or continue this expression in rook&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 140 | <code>        &#124; rising&#95;ray(1, square, occupied)</code> | Evaluate or continue this expression in rook&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 141 | <code>        &#124; falling&#95;ray(4, square, occupied)</code> | Evaluate or continue this expression in rook&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 142 | <code>        &#124; falling&#95;ray(5, square, occupied)</code> | Evaluate or continue this expression in rook&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 143 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 144 | <code></code> | Blank line separating related declarations or steps. |
| 145 | <code>/// The squares a bishop on the square reaches, including the first piece on each line.</code> | Source explanation: The squares a bishop on the square reaches, including the first piece on each line. |
| 146 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 147 | <code>pub fn bishop&#95;attacks(square: usize, occupied: u64) -&gt; u64 {</code> | Declare bishop&#95;attacks(); its parameters and return type define the interface. |
| 148 | <code>    rising&#95;ray(2, square, occupied)</code> | Evaluate or continue this expression in bishop&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 149 | <code>        &#124; rising&#95;ray(3, square, occupied)</code> | Evaluate or continue this expression in bishop&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 150 | <code>        &#124; falling&#95;ray(6, square, occupied)</code> | Evaluate or continue this expression in bishop&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 151 | <code>        &#124; falling&#95;ray(7, square, occupied)</code> | Evaluate or continue this expression in bishop&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 153 | <code></code> | Blank line separating related declarations or steps. |
| 154 | <code>/// Return true if a piece of the given color attacks the square.</code> | Source explanation: Return true if a piece of the given color attacks the square. |
| 155 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 156 | <code>pub fn is&#95;attacked(position: &amp;Position, square: usize, by&#95;white: bool) -&gt; bool {</code> | Declare is&#95;attacked(); its parameters and return type define the interface. |
| 157 | <code>    let side = usize::from(!by&#95;white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>    let attackers = position.by&#95;side&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 159 | <code>    let kinds = &amp;position.by&#95;kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 160 | <code>    // Look out from the square: a pawn of the other color on the square would attack</code> | Source explanation: Look out from the square: a pawn of the other color on the square would attack |
| 161 | <code>    // exactly the squares where a pawn of this color attacks it</code> | Source explanation: exactly the squares where a pawn of this color attacks it |
| 162 | <code>    if PAWN&#95;ATTACKS&#91;1 - side&#93;&#91;square&#93; &amp; attackers &amp; kinds&#91;PAWN as usize&#93; != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 163 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 164 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 165 | <code>    if KNIGHT&#95;ATTACKS&#91;square&#93; &amp; attackers &amp; kinds&#91;KNIGHT as usize&#93; != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 166 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 167 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 168 | <code>    if KING&#95;ATTACKS&#91;square&#93; &amp; attackers &amp; kinds&#91;KING as usize&#93; != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 169 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 170 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 171 | <code>    let queens = kinds&#91;QUEEN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 172 | <code>    let occupied = position.occupied();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 173 | <code>    let straight = attackers &amp; (kinds&#91;ROOK as usize&#93; &#124; queens);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 174 | <code>    if straight != 0 &amp;&amp; rook&#95;attacks(square, occupied) &amp; straight != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 175 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 176 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 177 | <code>    let diagonal = attackers &amp; (kinds&#91;BISHOP as usize&#93; &#124; queens);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 178 | <code>    diagonal != 0 &amp;&amp; bishop&#95;attacks(square, occupied) &amp; diagonal != 0</code> | Evaluate or continue this expression in is&#95;attacked(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 180 | <code></code> | Blank line separating related declarations or steps. |
| 181 | <code>/// Return true if the king of the given color is attacked.</code> | Source explanation: Return true if the king of the given color is attacked. |
| 182 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 183 | <code>pub fn in&#95;check(position: &amp;Position, white: bool) -&gt; bool {</code> | Declare in&#95;check(); its parameters and return type define the interface. |
| 184 | <code>    let kings = position.by&#95;side&#91;usize::from(!white)&#93; &amp; position.by&#95;kind&#91;KING as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 185 | <code>    is&#95;attacked(position, kings.trailing&#95;zeros() as usize, !white)</code> | Evaluate or continue this expression in in&#95;check(); a final expression without a semicolon supplies its block&#x27;s value. |
| 186 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### board.rs

Position and move representation, square coordinates, and position keys.

[Open source](src/board.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! The position, the starting position, and the names of squares and moves.</code> | Source explanation: The position, the starting position, and the names of squares and moves. |
| 2 | <code>//!</code> | Source explanation: |
| 3 | <code>//! Square numbers match Remus5: row 0 is rank 8 and column 0 is file a, so a8 is 0,</code> | Source explanation: Square numbers match Remus5: row 0 is rank 8 and column 0 is file a, so a8 is 0, |
| 4 | <code>//! h8 is 7, a1 is 56, and h1 is 63. A piece is its kind, from 1 for a pawn to 6 for a</code> | Source explanation: h8 is 7, a1 is 56, and h1 is 63. A piece is its kind, from 1 for a pawn to 6 for a |
| 5 | <code>//! king, plus 8 if it is White.</code> | Source explanation: king, plus 8 if it is White. |
| 6 | <code>//!</code> | Source explanation: |
| 7 | <code>//! The position also holds bitboards: one 64-bit number per kind of piece and per side,</code> | Source explanation: The position also holds bitboards: one 64-bit number per kind of piece and per side, |
| 8 | <code>//! where bit n is set when square n holds such a piece. Bitboards answer &quot;where are the</code> | Source explanation: where bit n is set when square n holds such a piece. Bitboards answer &quot;where are the |
| 9 | <code>//! rooks?&quot; and &quot;what does this rook reach?&quot; in a few machine instructions.</code> | Source explanation: rooks?&quot; and &quot;what does this rook reach?&quot; in a few machine instructions. |
| 10 | <code></code> | Blank line separating related declarations or steps. |
| 11 | <code>pub const EMPTY: u8 = 0;</code> | Define EMPTY, including its explicit type and initial value. |
| 12 | <code>pub const PAWN: u8 = 1;</code> | Define PAWN, including its explicit type and initial value. |
| 13 | <code>pub const KNIGHT: u8 = 2;</code> | Define KNIGHT, including its explicit type and initial value. |
| 14 | <code>pub const BISHOP: u8 = 3;</code> | Define BISHOP, including its explicit type and initial value. |
| 15 | <code>pub const ROOK: u8 = 4;</code> | Define ROOK, including its explicit type and initial value. |
| 16 | <code>pub const QUEEN: u8 = 5;</code> | Define QUEEN, including its explicit type and initial value. |
| 17 | <code>pub const KING: u8 = 6;</code> | Define KING, including its explicit type and initial value. |
| 18 | <code>pub const WHITE: u8 = 8; // Added to the kind of a White piece</code> | Define WHITE, including its explicit type and initial value. |
| 19 | <code>pub const MATE&#95;SCORE: i32 = 100&#95;000; // Larger than any material count</code> | Define MATE&#95;SCORE, including its explicit type and initial value. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>/// A move packed as from &#42; 512 + to &#42; 8 + promotion. Comparing two packed moves gives</code> | Source explanation: A move packed as from &#42; 512 + to &#42; 8 + promotion. Comparing two packed moves gives |
| 22 | <code>/// the same order as comparing the tuples (from, to, promotion) in Remus5. The number 0</code> | Source explanation: the same order as comparing the tuples (from, to, promotion) in Remus5. The number 0 |
| 23 | <code>/// means &quot;no move&quot;: a8 to a8 is never played.</code> | Source explanation: means &quot;no move&quot;: a8 to a8 is never played. |
| 24 | <code>pub type Move = u16;</code> | Declare the type Move used by this module. |
| 25 | <code></code> | Blank line separating related declarations or steps. |
| 26 | <code>pub const NO&#95;MOVE: Move = 0;</code> | Define NO&#95;MOVE, including its explicit type and initial value. |
| 27 | <code></code> | Blank line separating related declarations or steps. |
| 28 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 29 | <code>pub const fn new&#95;move(start: usize, target: usize, promotion: u8) -&gt; Move {</code> | Declare new&#95;move(); its parameters and return type define the interface. |
| 30 | <code>    ((start &lt;&lt; 9) &#124; (target &lt;&lt; 3) &#124; promotion as usize) as Move</code> | Evaluate or continue this expression in new&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 34 | <code>pub const fn move&#95;start(mv: Move) -&gt; usize {</code> | Declare move&#95;start(); its parameters and return type define the interface. |
| 35 | <code>    (mv &gt;&gt; 9) as usize</code> | Evaluate or continue this expression in move&#95;start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 37 | <code></code> | Blank line separating related declarations or steps. |
| 38 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 39 | <code>pub const fn move&#95;target(mv: Move) -&gt; usize {</code> | Declare move&#95;target(); its parameters and return type define the interface. |
| 40 | <code>    ((mv &gt;&gt; 3) &amp; 63) as usize</code> | Evaluate or continue this expression in move&#95;target(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code></code> | Blank line separating related declarations or steps. |
| 43 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 44 | <code>pub const fn move&#95;promotion(mv: Move) -&gt; u8 {</code> | Declare move&#95;promotion(); its parameters and return type define the interface. |
| 45 | <code>    (mv &amp; 7) as u8</code> | Evaluate or continue this expression in move&#95;promotion(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 47 | <code></code> | Blank line separating related declarations or steps. |
| 48 | <code>/// One step of the SplitMix64 random number generator, usable at compile time.</code> | Source explanation: One step of the SplitMix64 random number generator, usable at compile time. |
| 49 | <code>const fn next&#95;random(state: u64) -&gt; (u64, u64) {</code> | Declare next&#95;random(); its parameters and return type define the interface. |
| 50 | <code>    let state = state.wrapping&#95;add(0x9E37&#95;79B9&#95;7F4A&#95;7C15);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 51 | <code>    let mut z = state;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 52 | <code>    z = (z ^ (z &gt;&gt; 30)).wrapping&#95;mul(0xBF58&#95;476D&#95;1CE4&#95;E5B9);</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>    z = (z ^ (z &gt;&gt; 27)).wrapping&#95;mul(0x94D0&#95;49BB&#95;1331&#95;11EB);</code> | Update the named state using the expression and assignment operator shown. |
| 54 | <code>    (state, z ^ (z &gt;&gt; 31))</code> | Evaluate or continue this expression in next&#95;random(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code></code> | Blank line separating related declarations or steps. |
| 57 | <code>/// A random number for every piece on every square, for each en passant square, for</code> | Source explanation: A random number for every piece on every square, for each en passant square, for |
| 58 | <code>/// each set of castling rights, and for Black to move. The key of a position is the</code> | Source explanation: each set of castling rights, and for Black to move. The key of a position is the |
| 59 | <code>/// exclusive-or of the numbers that describe it, which make&#95;move() updates in a few</code> | Source explanation: exclusive-or of the numbers that describe it, which make&#95;move() updates in a few |
| 60 | <code>/// steps instead of reading all 64 squares.</code> | Source explanation: steps instead of reading all 64 squares. |
| 61 | <code>pub struct KeyTable {</code> | Declare the struct KeyTable used by this module. |
| 62 | <code>    pub piece: &#91;&#91;u64; 64&#93;; 15&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 63 | <code>    pub en&#95;passant: &#91;u64; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 64 | <code>    pub castling: &#91;u64; 16&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 65 | <code>    pub black: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 66 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code></code> | Blank line separating related declarations or steps. |
| 68 | <code>const fn build&#95;keys() -&gt; KeyTable {</code> | Declare build&#95;keys(); its parameters and return type define the interface. |
| 69 | <code>    let mut table = KeyTable {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 70 | <code>        piece: &#91;&#91;0; 64&#93;; 15&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 71 | <code>        en&#95;passant: &#91;0; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 72 | <code>        castling: &#91;0; 16&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 73 | <code>        black: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 74 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 75 | <code>    let mut state = 2016&#95;u64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 76 | <code>    let mut piece = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 77 | <code>    while piece &lt; 15 {</code> | Repeat this block while its condition remains true. |
| 78 | <code>        let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 79 | <code>        while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 80 | <code>            let (next, value) = next&#95;random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 81 | <code>            state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 82 | <code>            table.piece&#91;piece&#93;&#91;square&#93; = value;</code> | Update the named state using the expression and assignment operator shown. |
| 83 | <code>            square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 84 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 85 | <code>        piece += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 86 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 87 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 88 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 89 | <code>        let (next, value) = next&#95;random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 90 | <code>        state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 91 | <code>        table.en&#95;passant&#91;square&#93; = value;</code> | Update the named state using the expression and assignment operator shown. |
| 92 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 93 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code>    let mut rights = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 95 | <code>    while rights &lt; 16 {</code> | Repeat this block while its condition remains true. |
| 96 | <code>        let (next, value) = next&#95;random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 97 | <code>        state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 98 | <code>        table.castling&#91;rights&#93; = value;</code> | Update the named state using the expression and assignment operator shown. |
| 99 | <code>        rights += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 100 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 101 | <code>    let (&#95;, value) = next&#95;random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>    table.black = value;</code> | Update the named state using the expression and assignment operator shown. |
| 103 | <code>    table</code> | Evaluate or continue this expression in build&#95;keys(); a final expression without a semicolon supplies its block&#x27;s value. |
| 104 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 105 | <code></code> | Blank line separating related declarations or steps. |
| 106 | <code>pub static KEYS: KeyTable = build&#95;keys();</code> | Define KEYS, including its explicit type and initial value. |
| 107 | <code></code> | Blank line separating related declarations or steps. |
| 108 | <code>/// The position. Copying this small struct is the copy of copy-make.</code> | Source explanation: The position. Copying this small struct is the copy of copy-make. |
| 109 | <code>#&#91;derive(Clone, Copy)&#93;</code> | Compiler attribute controlling the following declaration. |
| 110 | <code>pub struct Position {</code> | Declare the struct Position used by this module. |
| 111 | <code>    pub board: &#91;u8; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 112 | <code>    pub by&#95;kind: &#91;u64; 7&#93;, // The squares of each kind of piece, of either side</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 113 | <code>    pub by&#95;side: &#91;u64; 2&#93;, // The squares of White&#x27;s pieces, then Black&#x27;s</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 114 | <code>    pub key: u64,          // The number that names the position</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 115 | <code>    pub white: bool,       // True when White is to move</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 116 | <code>    pub castling: u8,      // 1, 2 for White&#x27;s rights, 4, 8 for Black&#x27;s</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 117 | <code>    pub en&#95;passant: i8,    // The square a pawn just skipped over, or -1</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 118 | <code>    pub halfmove: i32,     // The moves since the last capture or pawn move</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 119 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 120 | <code></code> | Blank line separating related declarations or steps. |
| 121 | <code>impl Position {</code> | Begin the implementation of methods or a trait for the named type. |
| 122 | <code>    /// Build a position from its plain parts.</code> | Source explanation: Build a position from its plain parts. |
| 123 | <code>    pub fn new(</code> | Declare new(); its parameters and return type define the interface. |
| 124 | <code>        board: &#91;u8; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 125 | <code>        white: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 126 | <code>        castling: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 127 | <code>        en&#95;passant: i8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 128 | <code>        halfmove: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 129 | <code>    ) -&gt; Position {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 130 | <code>        let mut position = Position {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 131 | <code>            board: &#91;EMPTY; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 132 | <code>            by&#95;kind: &#91;0; 7&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 133 | <code>            by&#95;side: &#91;0; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 134 | <code>            key: KEYS.castling&#91;castling as usize&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 135 | <code>            white,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 136 | <code>            castling,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 137 | <code>            en&#95;passant,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 138 | <code>            halfmove,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 139 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 140 | <code>        for (square, &amp;piece) in board.iter().enumerate() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 141 | <code>            if piece != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 142 | <code>                position.put&#95;piece(square, piece);</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 143 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 144 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 145 | <code>        if !white {</code> | Test the condition and execute this branch only when it matches. |
| 146 | <code>            position.key ^= KEYS.black;</code> | Update the named state using the expression and assignment operator shown. |
| 147 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 148 | <code>        if en&#95;passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 149 | <code>            position.key ^= KEYS.en&#95;passant&#91;en&#95;passant as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 150 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 151 | <code>        position</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 153 | <code></code> | Blank line separating related declarations or steps. |
| 154 | <code>    /// The starting position.</code> | Source explanation: The starting position. |
| 155 | <code>    pub fn start() -&gt; Position {</code> | Declare start(); its parameters and return type define the interface. |
| 156 | <code>        let back&#95;rank = &#91;ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 157 | <code>        let mut board = &#91;EMPTY; 64&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>        for col in 0..8 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 159 | <code>            board&#91;col&#93; = back&#95;rank&#91;col&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 160 | <code>            board&#91;8 + col&#93; = PAWN;</code> | Evaluate or continue this expression in start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 161 | <code>            board&#91;48 + col&#93; = WHITE + PAWN;</code> | Evaluate or continue this expression in start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>            board&#91;56 + col&#93; = WHITE + back&#95;rank&#91;col&#93;;</code> | Evaluate or continue this expression in start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 163 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 164 | <code>        Position::new(board, true, 15, -1, 0)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 165 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 166 | <code></code> | Blank line separating related declarations or steps. |
| 167 | <code>    /// Build a position from FEN text, for the workshop designer&#x27;s checks.</code> | Source explanation: Build a position from FEN text, for the workshop designer&#x27;s checks. |
| 168 | <code>    pub fn from&#95;fen(words: &amp;&#91;&amp;str&#93;) -&gt; Position {</code> | Declare from&#95;fen(); its parameters and return type define the interface. |
| 169 | <code>        let mut board = &#91;EMPTY; 64&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 170 | <code>        let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 171 | <code>        for letter in words&#91;0&#93;.chars() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 172 | <code>            if let Some(gap) = letter.to&#95;digit(10) {</code> | Test the condition and execute this branch only when it matches. |
| 173 | <code>                square += gap as usize;</code> | Update the named state using the expression and assignment operator shown. |
| 174 | <code>            } else if letter != &#x27;/&#x27; {</code> | Test the condition and execute this branch only when it matches. |
| 175 | <code>                let kind = &quot;.pnbrqk&quot;.find(letter.to&#95;ascii&#95;lowercase()).unwrap&#95;or(0) as u8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 176 | <code>                board&#91;square&#93; = kind</code> | Update the named state using the expression and assignment operator shown. |
| 177 | <code>                    + if letter.is&#95;ascii&#95;uppercase() {</code> | Evaluate or continue this expression in from&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 178 | <code>                        WHITE</code> | Evaluate or continue this expression in from&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>                    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 180 | <code>                        0</code> | Evaluate or continue this expression in from&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 181 | <code>                    };</code> | Close the preceding block, expression, call, or collection. |
| 182 | <code>                square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 183 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 184 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 185 | <code>        let mut castling = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 186 | <code>        for (letter, bit) in &#91;(&#x27;K&#x27;, 1), (&#x27;Q&#x27;, 2), (&#x27;k&#x27;, 4), (&#x27;q&#x27;, 8)&#93; {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 187 | <code>            if words&#91;2&#93;.contains(letter) {</code> | Test the condition and execute this branch only when it matches. |
| 188 | <code>                castling += bit;</code> | Update the named state using the expression and assignment operator shown. |
| 189 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 190 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 191 | <code>        let en&#95;passant = if words&#91;3&#93; == &quot;-&quot; {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 192 | <code>            -1</code> | Evaluate or continue this expression in from&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 193 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 194 | <code>            parse&#95;square(words&#91;3&#93;) as i8</code> | Evaluate or continue this expression in from&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 195 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 196 | <code>        let halfmove = words.get(4).and&#95;then(&#124;text&#124; text.parse().ok()).unwrap&#95;or(0);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 197 | <code>        Position::new(board, words&#91;1&#93; == &quot;w&quot;, castling, en&#95;passant, halfmove)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 198 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 199 | <code></code> | Blank line separating related declarations or steps. |
| 200 | <code>    /// Place a piece on an empty square.</code> | Source explanation: Place a piece on an empty square. |
| 201 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 202 | <code>    pub fn put&#95;piece(&amp;mut self, square: usize, piece: u8) {</code> | Declare put&#95;piece(); its parameters and return type define the interface. |
| 203 | <code>        let bit = 1&#95;u64 &lt;&lt; square;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 204 | <code>        self.board&#91;square&#93; = piece;</code> | Update the named state using the expression and assignment operator shown. |
| 205 | <code>        self.by&#95;kind&#91;(piece &amp; 7) as usize&#93; &#124;= bit;</code> | Evaluate or continue this expression in put&#95;piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 206 | <code>        self.by&#95;side&#91;usize::from(piece &lt; WHITE)&#93; &#124;= bit;</code> | Evaluate or continue this expression in put&#95;piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 207 | <code>        self.key ^= KEYS.piece&#91;piece as usize&#93;&#91;square&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 208 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 209 | <code></code> | Blank line separating related declarations or steps. |
| 210 | <code>    /// Lift the piece off a square.</code> | Source explanation: Lift the piece off a square. |
| 211 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 212 | <code>    pub fn remove&#95;piece(&amp;mut self, square: usize) {</code> | Declare remove&#95;piece(); its parameters and return type define the interface. |
| 213 | <code>        let piece = self.board&#91;square&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 214 | <code>        let bit = 1&#95;u64 &lt;&lt; square;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 215 | <code>        self.board&#91;square&#93; = EMPTY;</code> | Update the named state using the expression and assignment operator shown. |
| 216 | <code>        self.by&#95;kind&#91;(piece &amp; 7) as usize&#93; &amp;= !bit;</code> | Evaluate or continue this expression in remove&#95;piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 217 | <code>        self.by&#95;side&#91;usize::from(piece &lt; WHITE)&#93; &amp;= !bit;</code> | Evaluate or continue this expression in remove&#95;piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 218 | <code>        self.key ^= KEYS.piece&#91;piece as usize&#93;&#91;square&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 219 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 220 | <code></code> | Blank line separating related declarations or steps. |
| 221 | <code>    /// The squares of every piece on the board.</code> | Source explanation: The squares of every piece on the board. |
| 222 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 223 | <code>    pub fn occupied(&amp;self) -&gt; u64 {</code> | Declare occupied(); its parameters and return type define the interface. |
| 224 | <code>        self.by&#95;side&#91;0&#93; &#124; self.by&#95;side&#91;1&#93;</code> | Evaluate or continue this expression in occupied(); a final expression without a semicolon supplies its block&#x27;s value. |
| 225 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 226 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 227 | <code></code> | Blank line separating related declarations or steps. |
| 228 | <code>/// Return the square that text such as &quot;e2&quot; names.</code> | Source explanation: Return the square that text such as &quot;e2&quot; names. |
| 229 | <code>pub fn parse&#95;square(text: &amp;str) -&gt; usize {</code> | Declare parse&#95;square(); its parameters and return type define the interface. |
| 230 | <code>    let bytes = text.as&#95;bytes();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 231 | <code>    let row = b&#x27;8&#x27;.wrapping&#95;sub(bytes&#91;1&#93;) as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 232 | <code>    let col = bytes&#91;0&#93;.wrapping&#95;sub(b&#x27;a&#x27;) as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 233 | <code>    (row &#42; 8 + col) &amp; 63</code> | Evaluate or continue this expression in parse&#95;square(); a final expression without a semicolon supplies its block&#x27;s value. |
| 234 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 235 | <code></code> | Blank line separating related declarations or steps. |
| 236 | <code>/// Add the name of a square, such as &quot;e2&quot;, to the text.</code> | Source explanation: Add the name of a square, such as &quot;e2&quot;, to the text. |
| 237 | <code>fn push&#95;square(text: &amp;mut String, square: usize) {</code> | Declare push&#95;square(); its parameters and return type define the interface. |
| 238 | <code>    text.push((b&#x27;a&#x27; + (square % 8) as u8) as char);</code> | Evaluate or continue this expression in push&#95;square(); a final expression without a semicolon supplies its block&#x27;s value. |
| 239 | <code>    text.push((b&#x27;8&#x27; - (square / 8) as u8) as char);</code> | Evaluate or continue this expression in push&#95;square(); a final expression without a semicolon supplies its block&#x27;s value. |
| 240 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 241 | <code></code> | Blank line separating related declarations or steps. |
| 242 | <code>/// Return the UCI text of a move, such as &quot;e7e8q&quot;.</code> | Source explanation: Return the UCI text of a move, such as &quot;e7e8q&quot;. |
| 243 | <code>pub fn move&#95;text(mv: Move) -&gt; String {</code> | Declare move&#95;text(); its parameters and return type define the interface. |
| 244 | <code>    let mut text = String::with&#95;capacity(5);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 245 | <code>    push&#95;square(&amp;mut text, move&#95;start(mv));</code> | Evaluate or continue this expression in move&#95;text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 246 | <code>    push&#95;square(&amp;mut text, move&#95;target(mv));</code> | Evaluate or continue this expression in move&#95;text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 247 | <code>    let promotion = move&#95;promotion(mv) as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 248 | <code>    if promotion != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 249 | <code>        text.push(b&quot;.pnbrqk&quot;&#91;promotion&#93; as char);</code> | Evaluate or continue this expression in move&#95;text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 250 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 251 | <code>    text</code> | Evaluate or continue this expression in move&#95;text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 252 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### evaluation.rs

Score material, piece placement, pawn structure, and king/endgame terms.

[Open source](src/evaluation.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Score a position in centipawns for the side to move. Every table and term matches</code> | Source explanation: Score a position in centipawns for the side to move. Every table and term matches |
| 2 | <code>//! evaluation.py in Remus5, and every division rounds down as Python&#x27;s // does.</code> | Source explanation: evaluation.py in Remus5, and every division rounds down as Python&#x27;s // does. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use crate::attacks::{KNIGHT&#95;ATTACKS, PAWN&#95;ATTACKS, bishop&#95;attacks, rook&#95;attacks};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use crate::board::{BISHOP, KING, KNIGHT, PAWN, Position, QUEEN, ROOK};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>/// Return a 64-square table built from files a to d of every rank.</code> | Source explanation: Return a 64-square table built from files a to d of every rank. |
| 8 | <code>const fn mirrored&#95;table(left&#95;half: &#91;i32; 32&#93;) -&gt; &#91;i32; 64&#93; {</code> | Declare mirrored&#95;table(); its parameters and return type define the interface. |
| 9 | <code>    let columns = &#91;0, 1, 2, 3, 3, 2, 1, 0&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 10 | <code>    let mut table = &#91;0; 64&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 11 | <code>    let mut row = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 12 | <code>    while row &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 13 | <code>        let mut col = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 14 | <code>        while col &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 15 | <code>            table&#91;row &#42; 8 + col&#93; = left&#95;half&#91;row &#42; 4 + columns&#91;col&#93;&#93;;</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 16 | <code>            col += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 17 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 18 | <code>        row += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 19 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 20 | <code>    table</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 21 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 22 | <code></code> | Blank line separating related declarations or steps. |
| 23 | <code>#&#91;rustfmt::skip&#93;</code> | Compiler attribute controlling the following declaration. |
| 24 | <code>static PAWN&#95;TABLE: &#91;i32; 64&#93; = mirrored&#95;table(&#91;</code> | Define PAWN&#95;TABLE, including its explicit type and initial value. |
| 25 | <code>      0,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 26 | <code>     50,  50,  50,  50,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 27 | <code>     10,  10,  20,  30,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 28 | <code>      5,   5,  10,  25,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 29 | <code>      0,   0,   0,  20,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>      5,  -5, -10,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>      5,  10,  10, -20,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 32 | <code>      0,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 33 | <code>&#93;);</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 34 | <code></code> | Blank line separating related declarations or steps. |
| 35 | <code>#&#91;rustfmt::skip&#93;</code> | Compiler attribute controlling the following declaration. |
| 36 | <code>static KNIGHT&#95;TABLE: &#91;i32; 64&#93; = mirrored&#95;table(&#91;</code> | Define KNIGHT&#95;TABLE, including its explicit type and initial value. |
| 37 | <code>    -50, -40, -30, -30,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>    -40, -20,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 39 | <code>    -30,   0,  10,  15,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 40 | <code>    -30,   5,  15,  20,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>    -30,   0,  15,  20,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 42 | <code>    -30,   5,  10,  15,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 43 | <code>    -40, -20,   0,   5,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 44 | <code>    -50, -40, -30, -30,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 45 | <code>&#93;);</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code></code> | Blank line separating related declarations or steps. |
| 47 | <code>#&#91;rustfmt::skip&#93;</code> | Compiler attribute controlling the following declaration. |
| 48 | <code>static BISHOP&#95;TABLE: &#91;i32; 64&#93; = mirrored&#95;table(&#91;</code> | Define BISHOP&#95;TABLE, including its explicit type and initial value. |
| 49 | <code>    -20, -10, -10, -10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 50 | <code>    -10,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 51 | <code>    -10,   0,   5,  10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 52 | <code>    -10,   5,   5,  10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 53 | <code>    -10,   0,  10,  10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>    -10,  10,  10,  10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>    -10,   5,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 56 | <code>    -20, -10, -10, -10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 57 | <code>&#93;);</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 58 | <code></code> | Blank line separating related declarations or steps. |
| 59 | <code>#&#91;rustfmt::skip&#93;</code> | Compiler attribute controlling the following declaration. |
| 60 | <code>static KING&#95;MIDDLE&#95;TABLE: &#91;i32; 64&#93; = mirrored&#95;table(&#91;</code> | Define KING&#95;MIDDLE&#95;TABLE, including its explicit type and initial value. |
| 61 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 63 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>    -20, -30, -30, -40,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 66 | <code>    -10, -20, -20, -20,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>     20,  20,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code>     20,  30,  10,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 69 | <code>&#93;);</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 70 | <code></code> | Blank line separating related declarations or steps. |
| 71 | <code>#&#91;rustfmt::skip&#93;</code> | Compiler attribute controlling the following declaration. |
| 72 | <code>static ROOK&#95;TABLE: &#91;i32; 64&#93; = mirrored&#95;table(&#91;</code> | Define ROOK&#95;TABLE, including its explicit type and initial value. |
| 73 | <code>      0,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 74 | <code>      5,  10,  10,  10,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 76 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 77 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 78 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 79 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 80 | <code>      0,   0,   0,   5,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>&#93;);</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 82 | <code></code> | Blank line separating related declarations or steps. |
| 83 | <code>#&#91;rustfmt::skip&#93;</code> | Compiler attribute controlling the following declaration. |
| 84 | <code>static KING&#95;END&#95;TABLE: &#91;i32; 64&#93; = mirrored&#95;table(&#91;</code> | Define KING&#95;END&#95;TABLE, including its explicit type and initial value. |
| 85 | <code>    -50, -40, -30, -20,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 86 | <code>    -30, -20, -10,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 87 | <code>    -30, -10,  20,  30,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 88 | <code>    -30, -10,  30,  40,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 89 | <code>    -30, -10,  30,  40,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 90 | <code>    -30, -10,  20,  30,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 91 | <code>    -30, -30,   0,   0,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 92 | <code>    -50, -30, -30, -30,</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 93 | <code>&#93;);</code> | Evaluate or continue this expression in mirrored&#95;table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 94 | <code></code> | Blank line separating related declarations or steps. |
| 95 | <code>// What each kind of piece is worth, from empty (0) to king (6)</code> | Source explanation: What each kind of piece is worth, from empty (0) to king (6) |
| 96 | <code>pub const PIECE&#95;VALUES: &#91;i32; 7&#93; = &#91;0, 100, 300, 300, 500, 900, 0&#93;;</code> | Define PIECE&#95;VALUES, including its explicit type and initial value. |
| 97 | <code></code> | Blank line separating related declarations or steps. |
| 98 | <code>// The bonus of a passed pawn by the moves it needs to promote</code> | Source explanation: The bonus of a passed pawn by the moves it needs to promote |
| 99 | <code>const PASSED&#95;BONUS: &#91;i32; 7&#93; = &#91;0, 120, 80, 50, 30, 15, 10&#93;;</code> | Define PASSED&#95;BONUS, including its explicit type and initial value. |
| 100 | <code>const MOBILITY&#95;BONUS: i32 = 2; // Per square a knight, bishop, rook, or queen reaches</code> | Define MOBILITY&#95;BONUS, including its explicit type and initial value. |
| 101 | <code>const BISHOP&#95;PAIR: i32 = 30;</code> | Define BISHOP&#95;PAIR, including its explicit type and initial value. |
| 102 | <code>const ROOK&#95;OPEN&#95;FILE: i32 = 20; // A file with no pawns</code> | Define ROOK&#95;OPEN&#95;FILE, including its explicit type and initial value. |
| 103 | <code>const ROOK&#95;HALF&#95;OPEN&#95;FILE: i32 = 10; // A file with only enemy pawns</code> | Define ROOK&#95;HALF&#95;OPEN&#95;FILE, including its explicit type and initial value. |
| 104 | <code>const DOUBLED&#95;PAWN: i32 = 15; // For each extra pawn on a file</code> | Define DOUBLED&#95;PAWN, including its explicit type and initial value. |
| 105 | <code>const ISOLATED&#95;PAWN: i32 = 15; // No friendly pawn on a file beside it</code> | Define ISOLATED&#95;PAWN, including its explicit type and initial value. |
| 106 | <code>const BACKWARD&#95;PAWN: i32 = 18; // Unsupported pawn whose advance meets an enemy pawn attack</code> | Define BACKWARD&#95;PAWN, including its explicit type and initial value. |
| 107 | <code>const BACKWARD&#95;OPEN&#95;FILE: i32 = 8; // No enemy pawn screens the backward pawn&#x27;s file</code> | Define BACKWARD&#95;OPEN&#95;FILE, including its explicit type and initial value. |
| 108 | <code>const CENTRAL&#95;HOLE: i32 = 12; // Empty central square an enemy knight can use</code> | Define CENTRAL&#95;HOLE, including its explicit type and initial value. |
| 109 | <code>const KNIGHT&#95;OUTPOST: i32 = 15; // Advanced knight that enemy pawns cannot challenge</code> | Define KNIGHT&#95;OUTPOST, including its explicit type and initial value. |
| 110 | <code>const SUPPORTED&#95;OUTPOST: i32 = 15; // Extra bonus for friendly pawn protection</code> | Define SUPPORTED&#95;OUTPOST, including its explicit type and initial value. |
| 111 | <code>const UNSTOPPABLE&#95;PAWN: i32 = 500; // A passed pawn the enemy king cannot catch</code> | Define UNSTOPPABLE&#95;PAWN, including its explicit type and initial value. |
| 112 | <code>const SHIELD&#95;PAWN: i32 = 10; // For each pawn in front of a king on its back rank</code> | Define SHIELD&#95;PAWN, including its explicit type and initial value. |
| 113 | <code>const OPEN&#95;KING&#95;FILE: i32 = 15; // For each file at the king with no friendly pawn</code> | Define OPEN&#95;KING&#95;FILE, including its explicit type and initial value. |
| 114 | <code></code> | Blank line separating related declarations or steps. |
| 115 | <code>const fn file&#95;mask(col: i32) -&gt; u64 {</code> | Declare file&#95;mask(); its parameters and return type define the interface. |
| 116 | <code>    if col &lt; 0 &#124;&#124; col &gt; 7 {</code> | Test the condition and execute this branch only when it matches. |
| 117 | <code>        0</code> | Evaluate or continue this expression in file&#95;mask(); a final expression without a semicolon supplies its block&#x27;s value. |
| 118 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 119 | <code>        0x0101&#95;0101&#95;0101&#95;0101&#95;u64 &lt;&lt; col</code> | Evaluate or continue this expression in file&#95;mask(); a final expression without a semicolon supplies its block&#x27;s value. |
| 120 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 121 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 122 | <code></code> | Blank line separating related declarations or steps. |
| 123 | <code>/// For each side and square, the squares where an enemy pawn can stop a pawn on that</code> | Source explanation: For each side and square, the squares where an enemy pawn can stop a pawn on that |
| 124 | <code>/// square: ahead of it, on its own file or a file beside it.</code> | Source explanation: square: ahead of it, on its own file or a file beside it. |
| 125 | <code>const fn build&#95;passed&#95;masks() -&gt; &#91;&#91;u64; 64&#93;; 2&#93; {</code> | Declare build&#95;passed&#95;masks(); its parameters and return type define the interface. |
| 126 | <code>    let mut table = &#91;&#91;0; 64&#93;; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 127 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 128 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 129 | <code>        let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 130 | <code>        let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 131 | <code>        let files = file&#95;mask(col - 1) &#124; file&#95;mask(col) &#124; file&#95;mask(col + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 132 | <code>        // Row 0 is rank 8, so the rows ahead of a White pawn have smaller numbers</code> | Source explanation: Row 0 is rank 8, so the rows ahead of a White pawn have smaller numbers |
| 133 | <code>        let rows&#95;above = if row == 0 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 134 | <code>            0</code> | Evaluate or continue this expression in build&#95;passed&#95;masks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 135 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 136 | <code>            u64::MAX &gt;&gt; (64 - 8 &#42; row)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 137 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 138 | <code>        let rows&#95;below = if row == 7 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 139 | <code>            0</code> | Evaluate or continue this expression in build&#95;passed&#95;masks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 140 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 141 | <code>            u64::MAX &lt;&lt; (8 &#42; (row + 1))</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 142 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 143 | <code>        table&#91;0&#93;&#91;square&#93; = files &amp; rows&#95;above;</code> | Update the named state using the expression and assignment operator shown. |
| 144 | <code>        table&#91;1&#93;&#91;square&#93; = files &amp; rows&#95;below;</code> | Update the named state using the expression and assignment operator shown. |
| 145 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 146 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 147 | <code>    table</code> | Evaluate or continue this expression in build&#95;passed&#95;masks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 148 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 149 | <code></code> | Blank line separating related declarations or steps. |
| 150 | <code>static PASSED&#95;MASKS: &#91;&#91;u64; 64&#93;; 2&#93; = build&#95;passed&#95;masks();</code> | Define PASSED&#95;MASKS, including its explicit type and initial value. |
| 151 | <code></code> | Blank line separating related declarations or steps. |
| 152 | <code>const SIGNS: &#91;i32; 2&#93; = &#91;1, -1&#93;;</code> | Define SIGNS, including its explicit type and initial value. |
| 153 | <code></code> | Blank line separating related declarations or steps. |
| 154 | <code>/// Mark pawn-controlled squares, including squares occupied by friendly pieces.</code> | Source explanation: Mark pawn-controlled squares, including squares occupied by friendly pieces. |
| 155 | <code>fn pawn&#95;attacks(pawns: &amp;&#91;u64; 2&#93;) -&gt; &#91;u64; 2&#93; {</code> | Declare pawn&#95;attacks(); its parameters and return type define the interface. |
| 156 | <code>    let mut attacks = &#91;0; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 157 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 158 | <code>        let mut left = pawns&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 159 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 160 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 161 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 162 | <code>            attacks&#91;side&#93; &#124;= PAWN&#95;ATTACKS&#91;side&#93;&#91;square&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 163 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 164 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 165 | <code>    attacks</code> | Evaluate or continue this expression in pawn&#95;attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 166 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 167 | <code></code> | Blank line separating related declarations or steps. |
| 168 | <code>/// Can an adjacent-file pawn attack this square, directly or after advancing?</code> | Source explanation: Can an adjacent-file pawn attack this square, directly or after advancing? |
| 169 | <code>fn pawn&#95;can&#95;challenge(pawns: u64, side: usize, square: usize) -&gt; bool {</code> | Declare pawn&#95;can&#95;challenge(); its parameters and return type define the interface. |
| 170 | <code>    let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 171 | <code>    let adjacent = file&#95;mask(col - 1) &#124; file&#95;mask(col + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 172 | <code>    pawns &amp; adjacent &amp; PASSED&#95;MASKS&#91;1 - side&#93;&#91;square&#93; != 0</code> | Evaluate or continue this expression in pawn&#95;can&#95;challenge(); a final expression without a semicolon supplies its block&#x27;s value. |
| 173 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 174 | <code></code> | Blank line separating related declarations or steps. |
| 175 | <code>/// Score backward pawns, central holes, and knight outposts from White&#x27;s side.</code> | Source explanation: Score backward pawns, central holes, and knight outposts from White&#x27;s side. |
| 176 | <code>fn positional&#95;terms(position: &amp;Position, pawns: &amp;&#91;u64; 2&#93;, attacks: &amp;&#91;u64; 2&#93;) -&gt; i32 {</code> | Declare positional&#95;terms(); its parameters and return type define the interface. |
| 177 | <code>    let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 178 | <code>    let knights = position.by&#95;kind&#91;KNIGHT as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 179 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 180 | <code>        let enemy = 1 - side;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 181 | <code>        let sign = SIGNS&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 182 | <code>        let step = if side == 0 { -8 } else { 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 183 | <code>        let mut left = pawns&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 184 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 185 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 186 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 187 | <code>            let target = square as i32 + step;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 188 | <code>            if !(0..64).contains(&amp;target) &#124;&#124; attacks&#91;enemy&#93; &amp; (1&#95;u64 &lt;&lt; target) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 189 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 190 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 191 | <code>            if attacks&#91;side&#93; &amp; (1&#95;u64 &lt;&lt; square) != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 192 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 193 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 194 | <code>            let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 195 | <code>            let adjacent = file&#95;mask(col - 1) &#124; file&#95;mask(col + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 196 | <code>            let support&#95;rows = PASSED&#95;MASKS&#91;1 - side&#93;&#91;square&#93; &#124; (0xff&#95;u64 &lt;&lt; (8 &#42; (square / 8)));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 197 | <code>            // Isolated pawns have their own penalty. Support can come from an</code> | Source explanation: Isolated pawns have their own penalty. Support can come from an |
| 198 | <code>            // adjacent-file pawn on this rank or behind it.</code> | Source explanation: adjacent-file pawn on this rank or behind it. |
| 199 | <code>            if pawns&#91;side&#93; &amp; adjacent != 0 &amp;&amp; pawns&#91;side&#93; &amp; adjacent &amp; support&#95;rows == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 200 | <code>                let mut penalty = BACKWARD&#95;PAWN;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 201 | <code>                if pawns&#91;enemy&#93; &amp; file&#95;mask(col) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 202 | <code>                    penalty += BACKWARD&#95;OPEN&#95;FILE;</code> | Update the named state using the expression and assignment operator shown. |
| 203 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 204 | <code>                score -= sign &#42; penalty;</code> | Update the named state using the expression and assignment operator shown. |
| 205 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 206 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 207 | <code>        if knights &amp; position.by&#95;side&#91;enemy&#93; != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 208 | <code>            // Files c to f of this side&#x27;s third and fourth ranks: rows 5 and 4 for</code> | Source explanation: Files c to f of this side&#x27;s third and fourth ranks: rows 5 and 4 for |
| 209 | <code>            // White, rows 2 and 3 for Black. One mask keeps the empty squares there</code> | Source explanation: White, rows 2 and 3 for Black. One mask keeps the empty squares there |
| 210 | <code>            // that an enemy pawn attacks, so the loop visits only those.</code> | Source explanation: that an enemy pawn attacks, so the loop visits only those. |
| 211 | <code>            let zone: u64 = if side == 0 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 212 | <code>                0x0000&#95;3C3C&#95;0000&#95;0000</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 213 | <code>            } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 214 | <code>                0x0000&#95;0000&#95;3C3C&#95;0000</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 215 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 216 | <code>            let mut holes = zone &amp; !position.occupied() &amp; attacks&#91;enemy&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 217 | <code>            while holes != 0 {</code> | Repeat this block while its condition remains true. |
| 218 | <code>                let square = holes.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 219 | <code>                holes &amp;= holes - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 220 | <code>                if !pawn&#95;can&#95;challenge(pawns&#91;side&#93;, side, square) {</code> | Test the condition and execute this branch only when it matches. |
| 221 | <code>                    score -= sign &#42; CENTRAL&#95;HOLE;</code> | Update the named state using the expression and assignment operator shown. |
| 222 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 224 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 225 | <code>        let mut left = knights &amp; position.by&#95;side&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 226 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 227 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 228 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 229 | <code>            let view&#95;row = if side == 0 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 230 | <code>                square / 8</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 231 | <code>            } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 232 | <code>                7 - square / 8</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 233 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 234 | <code>            if (2..=4).contains(&amp;view&#95;row)</code> | Test the condition and execute this branch only when it matches. |
| 235 | <code>                &amp;&amp; (1..=6).contains(&amp;(square % 8))</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 236 | <code>                &amp;&amp; !pawn&#95;can&#95;challenge(pawns&#91;enemy&#93;, enemy, square)</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 237 | <code>            {</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 238 | <code>                let mut bonus = KNIGHT&#95;OUTPOST;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 239 | <code>                if attacks&#91;side&#93; &amp; (1&#95;u64 &lt;&lt; square) != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 240 | <code>                    bonus += SUPPORTED&#95;OUTPOST;</code> | Update the named state using the expression and assignment operator shown. |
| 241 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 242 | <code>                score += sign &#42; bonus;</code> | Update the named state using the expression and assignment operator shown. |
| 243 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 244 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 245 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 246 | <code>    score</code> | Evaluate or continue this expression in positional&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 247 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 248 | <code></code> | Blank line separating related declarations or steps. |
| 249 | <code>/// Score nearby pawn cover, tapering its value as pieces leave the board.</code> | Source explanation: Score nearby pawn cover, tapering its value as pieces leave the board. |
| 250 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 251 | <code>fn king&#95;safety(own&#95;pawns: u64, counts: &amp;&#91;i32; 10&#93;, king: i32, side: usize, phase: i32) -&gt; i32 {</code> | Declare king&#95;safety(); its parameters and return type define the interface. |
| 252 | <code>    let row = king / 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 253 | <code>    let view&#95;row = if side == 0 { row } else { 7 - row };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 254 | <code>    if view&#95;row &lt; 5 {</code> | Test the condition and execute this branch only when it matches. |
| 255 | <code>        return 0; // An advanced king uses its piece-square table</code> | Leave the current function with the displayed result. |
| 256 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 257 | <code>    let col = king % 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 258 | <code>    let forward = if side == 0 { -8 } else { 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 259 | <code>    let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 260 | <code>    for c in (col - 1).max(0)..(col + 2).min(8) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 261 | <code>        if own&#95;pawns &amp; (1&#95;u64 &lt;&lt; (king + forward + c - col)) != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 262 | <code>            score += SHIELD&#95;PAWN;</code> | Update the named state using the expression and assignment operator shown. |
| 263 | <code>        } else if own&#95;pawns &amp; (1&#95;u64 &lt;&lt; (king + 2 &#42; forward + c - col)) != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 264 | <code>            score += SHIELD&#95;PAWN / 2;</code> | Update the named state using the expression and assignment operator shown. |
| 265 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 266 | <code>        if counts&#91;(c + 1) as usize&#93; == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 267 | <code>            score -= OPEN&#95;KING&#95;FILE;</code> | Update the named state using the expression and assignment operator shown. |
| 268 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 269 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 270 | <code>    (score &#42; phase).div&#95;euclid(24)</code> | Evaluate or continue this expression in king&#95;safety(); a final expression without a semicolon supplies its block&#x27;s value. |
| 271 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 272 | <code></code> | Blank line separating related declarations or steps. |
| 273 | <code>/// Score the pawns, the rooks that use their files, and the pawn shield of each king,</code> | Source explanation: Score the pawns, the rooks that use their files, and the pawn shield of each king, |
| 274 | <code>/// from White&#x27;s side.</code> | Source explanation: from White&#x27;s side. |
| 275 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 276 | <code>fn pawn&#95;terms(</code> | Declare pawn&#95;terms(); its parameters and return type define the interface. |
| 277 | <code>    pawns: &amp;&#91;u64; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 278 | <code>    rooks: &amp;&#91;u64; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 279 | <code>    kings: &amp;&#91;i32; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 280 | <code>    phase: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 281 | <code>    white: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 282 | <code>) -&gt; i32 {</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 283 | <code>    let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 284 | <code>    // Entry col + 1 counts file col, so the a and h files have an empty file on each side</code> | Source explanation: Entry col + 1 counts file col, so the a and h files have an empty file on each side |
| 285 | <code>    let mut counts = &#91;&#91;0&#95;i32; 10&#93;; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 286 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 287 | <code>        let mut left = pawns&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 288 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 289 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 290 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 291 | <code>            counts&#91;side&#93;&#91;square % 8 + 1&#93; += 1;</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 292 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 293 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 294 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 295 | <code>        let sign = SIGNS&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 296 | <code>        let enemy = 1 - side;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 297 | <code>        for count in counts&#91;side&#93; {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 298 | <code>            score -= sign &#42; DOUBLED&#95;PAWN &#42; (count - 1).max(0);</code> | Update the named state using the expression and assignment operator shown. |
| 299 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 300 | <code>        let mut left = pawns&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 301 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 302 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 303 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 304 | <code>            let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 305 | <code>            let col = square % 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 306 | <code>            if counts&#91;side&#93;&#91;col&#93; == 0 &amp;&amp; counts&#91;side&#93;&#91;col + 2&#93; == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 307 | <code>                score -= sign &#42; ISOLATED&#95;PAWN;</code> | Update the named state using the expression and assignment operator shown. |
| 308 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 309 | <code>            if PASSED&#95;MASKS&#91;side&#93;&#91;square&#93; &amp; pawns&#91;enemy&#93; != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 310 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 311 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 312 | <code>            let steps = if side == 0 { row } else { 7 - row }; // Moves to promotion</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 313 | <code>            score += sign &#42; PASSED&#95;BONUS&#91;steps as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 314 | <code>            // The pawn race: with no pieces left, a pawn whose promotion square the enemy</code> | Source explanation: The pawn race: with no pieces left, a pawn whose promotion square the enemy |
| 315 | <code>            // king cannot reach (the rule of the square) becomes a queen</code> | Source explanation: king cannot reach (the rule of the square) becomes a queen |
| 316 | <code>            let king = kings&#91;enemy&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 317 | <code>            let mut king&#95;steps = (king / 8 - 7 &#42; side as i32)</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 318 | <code>                .abs()</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 319 | <code>                .max((king % 8 - col as i32).abs());</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 320 | <code>            if white != (side == 0) {</code> | Test the condition and execute this branch only when it matches. |
| 321 | <code>                king&#95;steps -= 1; // The enemy king moves first</code> | Update the named state using the expression and assignment operator shown. |
| 322 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 323 | <code>            if phase == 0 &amp;&amp; king&#95;steps &gt; steps.min(5) {</code> | Test the condition and execute this branch only when it matches. |
| 324 | <code>                score += sign &#42; UNSTOPPABLE&#95;PAWN;</code> | Update the named state using the expression and assignment operator shown. |
| 325 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 326 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 327 | <code>        let mut left = rooks&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 328 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 329 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 330 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 331 | <code>            let entry = square % 8 + 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 332 | <code>            if counts&#91;side&#93;&#91;entry&#93; == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 333 | <code>                let open&#95;file = counts&#91;enemy&#93;&#91;entry&#93; == 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 334 | <code>                score += sign</code> | Update the named state using the expression and assignment operator shown. |
| 335 | <code>                    &#42; if open&#95;file {</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 336 | <code>                        ROOK&#95;OPEN&#95;FILE</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 337 | <code>                    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 338 | <code>                        ROOK&#95;HALF&#95;OPEN&#95;FILE</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 339 | <code>                    };</code> | Close the preceding block, expression, call, or collection. |
| 340 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 341 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 342 | <code>        score += sign &#42; king&#95;safety(pawns&#91;side&#93;, &amp;counts&#91;side&#93;, kings&#91;side&#93;, side, phase);</code> | Update the named state using the expression and assignment operator shown. |
| 343 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 344 | <code>    score</code> | Evaluate or continue this expression in pawn&#95;terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 345 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 346 | <code></code> | Blank line separating related declarations or steps. |
| 347 | <code>/// In an endgame where one side is ahead, reward pushing the losing king to the edge and</code> | Source explanation: In an endgame where one side is ahead, reward pushing the losing king to the edge and |
| 348 | <code>/// bringing the winning king close, from White&#x27;s side.</code> | Source explanation: bringing the winning king close, from White&#x27;s side. |
| 349 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 350 | <code>fn mating&#95;bonus(material: i32, kings: &amp;&#91;i32; 2&#93;, phase: i32) -&gt; i32 {</code> | Declare mating&#95;bonus(); its parameters and return type define the interface. |
| 351 | <code>    if phase &gt; 8 &#124;&#124; material.abs() &lt; 400 {</code> | Test the condition and execute this branch only when it matches. |
| 352 | <code>        return 0;</code> | Leave the current function with the displayed result. |
| 353 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 354 | <code>    let winner = if material &gt; 0 { 0 } else { 1 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 355 | <code>    let loser&#95;row = kings&#91;1 - winner&#93; / 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 356 | <code>    let loser&#95;col = kings&#91;1 - winner&#93; % 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 357 | <code>    // 0 in the center, 6 in a corner</code> | Source explanation: 0 in the center, 6 in a corner |
| 358 | <code>    let edge = (3 - loser&#95;row).max(loser&#95;row - 4) + (3 - loser&#95;col).max(loser&#95;col - 4);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 359 | <code>    let distance = (kings&#91;0&#93; / 8 - kings&#91;1&#93; / 8)</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 360 | <code>        .abs()</code> | Evaluate or continue this expression in mating&#95;bonus(); a final expression without a semicolon supplies its block&#x27;s value. |
| 361 | <code>        .max((kings&#91;0&#93; % 8 - kings&#91;1&#93; % 8).abs());</code> | Evaluate or continue this expression in mating&#95;bonus(); a final expression without a semicolon supplies its block&#x27;s value. |
| 362 | <code>    let bonus = 10 &#42; edge + 5 &#42; (7 - distance);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 363 | <code>    if winner == 0 { bonus } else { -bonus }</code> | Test the condition and execute this branch only when it matches. |
| 364 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 365 | <code></code> | Blank line separating related declarations or steps. |
| 366 | <code>/// Return the score for the side to move: positive when it is ahead.</code> | Source explanation: Return the score for the side to move: positive when it is ahead. |
| 367 | <code>pub fn evaluate(position: &amp;Position) -&gt; i32 {</code> | Declare evaluate(); its parameters and return type define the interface. |
| 368 | <code>    let kinds = &amp;position.by&#95;kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 369 | <code>    let occupied = position.occupied();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 370 | <code>    let mut score = 0; // From White&#x27;s side</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 371 | <code>    let mut material = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 372 | <code>    let mut phase = 0; // 24 with every piece on the board, 0 with none left</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 373 | <code>    let mut kings = &#91;0&#95;i32; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 374 | <code>    let pawns = &#91;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 375 | <code>        position.by&#95;side&#91;0&#93; &amp; kinds&#91;PAWN as usize&#93;,</code> | Evaluate or continue this expression in evaluate(); a final expression without a semicolon supplies its block&#x27;s value. |
| 376 | <code>        position.by&#95;side&#91;1&#93; &amp; kinds&#91;PAWN as usize&#93;,</code> | Evaluate or continue this expression in evaluate(); a final expression without a semicolon supplies its block&#x27;s value. |
| 377 | <code>    &#93;;</code> | Close the preceding block, expression, call, or collection. |
| 378 | <code>    let attacks = pawn&#95;attacks(&amp;pawns);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 379 | <code>    let mut rooks = &#91;0&#95;u64; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 380 | <code>    let mut bishop&#95;count = &#91;0&#95;u32; 2&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 381 | <code></code> | Blank line separating related declarations or steps. |
| 382 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 383 | <code>        let sign = SIGNS&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 384 | <code>        let own = position.by&#95;side&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 385 | <code>        // Flip the row for Black: Black&#x27;s a8 is White&#x27;s a1</code> | Source explanation: Flip the row for Black: Black&#x27;s a8 is White&#x27;s a1 |
| 386 | <code>        let flip = if side == 0 { 0 } else { 56 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 387 | <code>        let mut side&#95;score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 388 | <code>        let mut reach&#95;count = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 389 | <code></code> | Blank line separating related declarations or steps. |
| 390 | <code>        rooks&#91;side&#93; = own &amp; kinds&#91;ROOK as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 391 | <code>        kings&#91;side&#93; = (own &amp; kinds&#91;KING as usize&#93;).trailing&#95;zeros() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 392 | <code></code> | Blank line separating related declarations or steps. |
| 393 | <code>        let mut left = pawns&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 394 | <code>        material += sign &#42; 100 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 395 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 396 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 397 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 398 | <code>            side&#95;score += PAWN&#95;TABLE&#91;square ^ flip&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 399 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 400 | <code>        let mut left = own &amp; kinds&#91;KNIGHT as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 401 | <code>        phase += left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 402 | <code>        material += sign &#42; 300 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 403 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 404 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 405 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 406 | <code>            side&#95;score += KNIGHT&#95;TABLE&#91;square ^ flip&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 407 | <code>            reach&#95;count += (KNIGHT&#95;ATTACKS&#91;square&#93; &amp; !own).count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 408 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 409 | <code>        let mut left = own &amp; kinds&#91;BISHOP as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 410 | <code>        bishop&#95;count&#91;side&#93; = left.count&#95;ones();</code> | Update the named state using the expression and assignment operator shown. |
| 411 | <code>        phase += left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 412 | <code>        material += sign &#42; 300 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 413 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 414 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 415 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 416 | <code>            side&#95;score += BISHOP&#95;TABLE&#91;square ^ flip&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 417 | <code>            reach&#95;count += (bishop&#95;attacks(square, occupied) &amp; !own).count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 418 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 419 | <code>        let mut left = rooks&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 420 | <code>        phase += 2 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 421 | <code>        material += sign &#42; 500 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 422 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 423 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 424 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 425 | <code>            side&#95;score += ROOK&#95;TABLE&#91;square ^ flip&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 426 | <code>            reach&#95;count += (rook&#95;attacks(square, occupied) &amp; !own).count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 427 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 428 | <code>        let mut left = own &amp; kinds&#91;QUEEN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 429 | <code>        phase += 4 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 430 | <code>        material += sign &#42; 900 &#42; left.count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 431 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 432 | <code>            let square = left.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 433 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 434 | <code>            let reach = rook&#95;attacks(square, occupied) &#124; bishop&#95;attacks(square, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 435 | <code>            reach&#95;count += (reach &amp; !own).count&#95;ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 436 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 437 | <code>        score += sign &#42; (side&#95;score + MOBILITY&#95;BONUS &#42; reach&#95;count);</code> | Update the named state using the expression and assignment operator shown. |
| 438 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 439 | <code></code> | Blank line separating related declarations or steps. |
| 440 | <code>    // Insufficient material: a lone knight or bishop cannot mate</code> | Source explanation: Insufficient material: a lone knight or bishop cannot mate |
| 441 | <code>    if phase &lt;= 1 &amp;&amp; pawns&#91;0&#93; == 0 &amp;&amp; pawns&#91;1&#93; == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 442 | <code>        return 0;</code> | Leave the current function with the displayed result. |
| 443 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 444 | <code>    let phase = phase.min(24);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 445 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 446 | <code>        let flip = if side == 0 { 0 } else { 56 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 447 | <code>        let view = kings&#91;side&#93; as usize ^ flip;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 448 | <code>        let taper = KING&#95;MIDDLE&#95;TABLE&#91;view&#93; &#42; phase + KING&#95;END&#95;TABLE&#91;view&#93; &#42; (24 - phase);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 449 | <code>        score += (SIGNS&#91;side&#93; &#42; taper).div&#95;euclid(24);</code> | Update the named state using the expression and assignment operator shown. |
| 450 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 451 | <code>    if bishop&#95;count&#91;0&#93; &gt;= 2 {</code> | Test the condition and execute this branch only when it matches. |
| 452 | <code>        score += BISHOP&#95;PAIR;</code> | Update the named state using the expression and assignment operator shown. |
| 453 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 454 | <code>    if bishop&#95;count&#91;1&#93; &gt;= 2 {</code> | Test the condition and execute this branch only when it matches. |
| 455 | <code>        score -= BISHOP&#95;PAIR;</code> | Update the named state using the expression and assignment operator shown. |
| 456 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 457 | <code>    score += material + pawn&#95;terms(&amp;pawns, &amp;rooks, &amp;kings, phase, position.white);</code> | Update the named state using the expression and assignment operator shown. |
| 458 | <code>    score += mating&#95;bonus(material, &amp;kings, phase);</code> | Update the named state using the expression and assignment operator shown. |
| 459 | <code>    score += positional&#95;terms(position, &amp;pawns, &amp;attacks);</code> | Update the named state using the expression and assignment operator shown. |
| 460 | <code>    if position.white { score } else { -score }</code> | Test the condition and execute this branch only when it matches. |
| 461 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### exchange.rs

Estimate a sequence of captures on one square for move ordering and pruning.

[Open source](src/exchange.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Static exchange evaluation (SEE): play out every capture on one square, cheapest</code> | Source explanation: Static exchange evaluation (SEE): play out every capture on one square, cheapest |
| 2 | <code>//! attacker first, and return what the first capture wins or loses.</code> | Source explanation: attacker first, and return what the first capture wins or loses. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use crate::attacks::{KING&#95;ATTACKS, KNIGHT&#95;ATTACKS, PAWN&#95;ATTACKS, bishop&#95;attacks, rook&#95;attacks};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use crate::board::{BISHOP, EMPTY, KING, KNIGHT, Move, PAWN, QUEEN, ROOK, WHITE};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use crate::board::{move&#95;start, move&#95;target};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code>use crate::evaluation::PIECE&#95;VALUES;</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code></code> | Blank line separating related declarations or steps. |
| 9 | <code>/// The board during the exchange, with only what the exchange needs.</code> | Source explanation: The board during the exchange, with only what the exchange needs. |
| 10 | <code>struct ExchangeBoard {</code> | Declare the struct ExchangeBoard used by this module. |
| 11 | <code>    board: &#91;u8; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 12 | <code>    by&#95;kind: &#91;u64; 7&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 13 | <code>    by&#95;side: &#91;u64; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 14 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 15 | <code></code> | Blank line separating related declarations or steps. |
| 16 | <code>impl ExchangeBoard {</code> | Begin the implementation of methods or a trait for the named type. |
| 17 | <code>    /// Lift the piece off the square.</code> | Source explanation: Lift the piece off the square. |
| 18 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 19 | <code>    fn remove(&amp;mut self, square: usize) {</code> | Declare remove(); its parameters and return type define the interface. |
| 20 | <code>        let piece = self.board&#91;square&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 21 | <code>        let bit = 1&#95;u64 &lt;&lt; square;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 22 | <code>        self.board&#91;square&#93; = EMPTY;</code> | Update the named state using the expression and assignment operator shown. |
| 23 | <code>        self.by&#95;kind&#91;(piece &amp; 7) as usize&#93; &amp;= !bit;</code> | Evaluate or continue this expression in remove(); a final expression without a semicolon supplies its block&#x27;s value. |
| 24 | <code>        self.by&#95;side&#91;usize::from(piece &lt; WHITE)&#93; &amp;= !bit;</code> | Evaluate or continue this expression in remove(); a final expression without a semicolon supplies its block&#x27;s value. |
| 25 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 26 | <code></code> | Blank line separating related declarations or steps. |
| 27 | <code>    /// Move the piece on start to the target square, capturing what stands there.</code> | Source explanation: Move the piece on start to the target square, capturing what stands there. |
| 28 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 29 | <code>    fn capture(&amp;mut self, start: usize, target: usize) {</code> | Declare capture(); its parameters and return type define the interface. |
| 30 | <code>        let piece = self.board&#91;start&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 31 | <code>        let bit = 1&#95;u64 &lt;&lt; target;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 32 | <code>        self.remove(target);</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 33 | <code>        self.remove(start);</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 34 | <code>        self.board&#91;target&#93; = piece;</code> | Update the named state using the expression and assignment operator shown. |
| 35 | <code>        self.by&#95;kind&#91;(piece &amp; 7) as usize&#93; &#124;= bit;</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>        self.by&#95;side&#91;usize::from(piece &lt; WHITE)&#93; &#124;= bit;</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 37 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 38 | <code></code> | Blank line separating related declarations or steps. |
| 39 | <code>    /// Return the square of the cheapest piece that attacks the square, or None. A</code> | Source explanation: Return the square of the cheapest piece that attacks the square, or None. A |
| 40 | <code>    /// knight and a bishop are worth the same, and the lower square number goes first.</code> | Source explanation: knight and a bishop are worth the same, and the lower square number goes first. |
| 41 | <code>    #&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 42 | <code>    fn cheapest&#95;attacker(&amp;self, square: usize, by&#95;white: bool) -&gt; Option&lt;usize&gt; {</code> | Declare cheapest&#95;attacker(); its parameters and return type define the interface. |
| 43 | <code>        let side = usize::from(!by&#95;white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 44 | <code>        let own = self.by&#95;side&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 45 | <code>        let kinds = &amp;self.by&#95;kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>        let occupied = self.by&#95;side&#91;0&#93; &#124; self.by&#95;side&#91;1&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>        let found = PAWN&#95;ATTACKS&#91;1 - side&#93;&#91;square&#93; &amp; own &amp; kinds&#91;PAWN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 48 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 49 | <code>            return Some(found.trailing&#95;zeros() as usize);</code> | Leave the current function with the displayed result. |
| 50 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code>        let bishop&#95;lines = bishop&#95;attacks(square, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 52 | <code>        let found = own</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 53 | <code>            &amp; ((KNIGHT&#95;ATTACKS&#91;square&#93; &amp; kinds&#91;KNIGHT as usize&#93;)</code> | Evaluate or continue this expression in cheapest&#95;attacker(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>                &#124; (bishop&#95;lines &amp; kinds&#91;BISHOP as usize&#93;));</code> | Evaluate or continue this expression in cheapest&#95;attacker(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 56 | <code>            return Some(found.trailing&#95;zeros() as usize);</code> | Leave the current function with the displayed result. |
| 57 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 58 | <code>        let rook&#95;lines = rook&#95;attacks(square, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 59 | <code>        let found = own &amp; rook&#95;lines &amp; kinds&#91;ROOK as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 60 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>            return Some(found.trailing&#95;zeros() as usize);</code> | Leave the current function with the displayed result. |
| 62 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 63 | <code>        let found = own &amp; (rook&#95;lines &#124; bishop&#95;lines) &amp; kinds&#91;QUEEN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 64 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 65 | <code>            return Some(found.trailing&#95;zeros() as usize);</code> | Leave the current function with the displayed result. |
| 66 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code>        let found = own &amp; KING&#95;ATTACKS&#91;square&#93; &amp; kinds&#91;KING as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 68 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 69 | <code>            return Some(found.trailing&#95;zeros() as usize);</code> | Leave the current function with the displayed result. |
| 70 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 71 | <code>        None</code> | Evaluate or continue this expression in cheapest&#95;attacker(); a final expression without a semicolon supplies its block&#x27;s value. |
| 72 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code></code> | Blank line separating related declarations or steps. |
| 75 | <code>const KING&#95;VALUE: i32 = 2000; // The king captures last: losing it ends the game</code> | Define KING&#95;VALUE, including its explicit type and initial value. |
| 76 | <code></code> | Blank line separating related declarations or steps. |
| 77 | <code>/// Return the material the capture wins when both sides keep recapturing.</code> | Source explanation: Return the material the capture wins when both sides keep recapturing. |
| 78 | <code>pub fn exchange&#95;score(board: &amp;&#91;u8; 64&#93;, by&#95;kind: &amp;&#91;u64; 7&#93;, by&#95;side: &amp;&#91;u64; 2&#93;, mv: Move) -&gt; i32 {</code> | Declare exchange&#95;score(); its parameters and return type define the interface. |
| 79 | <code>    let start = move&#95;start(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 80 | <code>    let target = move&#95;target(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 81 | <code>    let mut exchange = ExchangeBoard {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 82 | <code>        board: &#42;board,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 83 | <code>        by&#95;kind: &#42;by&#95;kind,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 84 | <code>        by&#95;side: &#42;by&#95;side,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 85 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 86 | <code>    // gains&#91;i&#93; is what the side making capture i has won after it</code> | Source explanation: gains&#91;i&#93; is what the side making capture i has won after it |
| 87 | <code>    let mut gains = &#91;0&#95;i32; 40&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 88 | <code>    let mut count = 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 89 | <code>    gains&#91;0&#93; = PIECE&#95;VALUES&#91;(exchange.board&#91;target&#93; &amp; 7) as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 90 | <code>    let mut white = exchange.board&#91;start&#93; &gt; WHITE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 91 | <code>    if exchange.board&#91;start&#93; &amp; 7 == PAWN &amp;&amp; exchange.board&#91;target&#93; == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 92 | <code>        // En passant removes the pawn behind the empty destination.</code> | Source explanation: En passant removes the pawn behind the empty destination. |
| 93 | <code>        gains&#91;0&#93; = PIECE&#95;VALUES&#91;PAWN as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 94 | <code>        exchange.remove(if white { target + 8 } else { target - 8 });</code> | Evaluate or continue this expression in exchange&#95;score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 95 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 96 | <code>    exchange.capture(start, target);</code> | Evaluate or continue this expression in exchange&#95;score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 97 | <code>    let mut attacker = exchange.cheapest&#95;attacker(target, !white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 98 | <code>    while let Some(from) = attacker {</code> | Repeat this block while its condition remains true. |
| 99 | <code>        // The king may not capture onto a defended square</code> | Source explanation: The king may not capture onto a defended square |
| 100 | <code>        if exchange.board&#91;target&#93; &amp; 7 == KING {</code> | Test the condition and execute this branch only when it matches. |
| 101 | <code>            if count == 1 {</code> | Test the condition and execute this branch only when it matches. |
| 102 | <code>                return -KING&#95;VALUE;</code> | Leave the current function with the displayed result. |
| 103 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 104 | <code>            count -= 1;</code> | Update the named state using the expression and assignment operator shown. |
| 105 | <code>            break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 106 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 107 | <code>        gains&#91;count&#93; = PIECE&#95;VALUES&#91;(exchange.board&#91;target&#93; &amp; 7) as usize&#93; - gains&#91;count - 1&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 108 | <code>        count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 109 | <code>        exchange.capture(from, target);</code> | Evaluate or continue this expression in exchange&#95;score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 110 | <code>        white = !white;</code> | Update the named state using the expression and assignment operator shown. |
| 111 | <code>        attacker = exchange.cheapest&#95;attacker(target, !white);</code> | Update the named state using the expression and assignment operator shown. |
| 112 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 113 | <code>    // Work back from the last capture: each side may stop capturing</code> | Source explanation: Work back from the last capture: each side may stop capturing |
| 114 | <code>    for i in (1..count).rev() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 115 | <code>        gains&#91;i - 1&#93; = -(-gains&#91;i - 1&#93;).max(gains&#91;i&#93;);</code> | Evaluate or continue this expression in exchange&#95;score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 116 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 117 | <code>    gains&#91;0&#93;</code> | Evaluate or continue this expression in exchange&#95;score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 118 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### main.rs

Read UCI commands, maintain game state, choose legal root moves, and report results.

[Open source](src/main.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Talk to a chess GUI such as Lucas Chess through the Universal Chess Interface (UCI).</code> | Source explanation: Talk to a chess GUI such as Lucas Chess through the Universal Chess Interface (UCI). |
| 2 | <code>//! The GUI sends one command per line, and the engine answers each one. Besides what</code> | Source explanation: The GUI sends one command per line, and the engine answers each one. Besides what |
| 3 | <code>//! Lucas Chess sends, &quot;position fen&quot;, &quot;go depth&quot;, &quot;go perft&quot;, and &quot;eval&quot; serve the</code> | Source explanation: Lucas Chess sends, &quot;position fen&quot;, &quot;go depth&quot;, &quot;go perft&quot;, and &quot;eval&quot; serve the |
| 4 | <code>//! workshop designer&#x27;s check that this engine matches Remus5.</code> | Source explanation: workshop designer&#x27;s check that this engine matches Remus5. |
| 5 | <code></code> | Blank line separating related declarations or steps. |
| 6 | <code>mod attacks;</code> | Include a sibling Rust module in this program. |
| 7 | <code>mod board;</code> | Include a sibling Rust module in this program. |
| 8 | <code>mod evaluation;</code> | Include a sibling Rust module in this program. |
| 9 | <code>mod exchange;</code> | Include a sibling Rust module in this program. |
| 10 | <code>mod make&#95;move;</code> | Include a sibling Rust module in this program. |
| 11 | <code>mod move&#95;generation;</code> | Include a sibling Rust module in this program. |
| 12 | <code>mod opening&#95;book;</code> | Include a sibling Rust module in this program. |
| 13 | <code>mod parallel&#95;search;</code> | Include a sibling Rust module in this program. |
| 14 | <code>mod search;</code> | Include a sibling Rust module in this program. |
| 15 | <code>mod search&#95;control;</code> | Include a sibling Rust module in this program. |
| 16 | <code>mod search&#95;metrics;</code> | Include a sibling Rust module in this program. |
| 17 | <code>mod table&#95;metrics;</code> | Include a sibling Rust module in this program. |
| 18 | <code>mod tablebase;</code> | Include a sibling Rust module in this program. |
| 19 | <code>mod transposition&#95;table;</code> | Include a sibling Rust module in this program. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>use std::io::Write;</code> | Bring the named items into scope; no chess search runs here. |
| 22 | <code>use std::time::Instant;</code> | Bring the named items into scope; no chess search runs here. |
| 23 | <code></code> | Blank line separating related declarations or steps. |
| 24 | <code>use attacks::in&#95;check;</code> | Bring the named items into scope; no chess search runs here. |
| 25 | <code>use board::{Move, NO&#95;MOVE, Position, move&#95;text, new&#95;move, parse&#95;square};</code> | Bring the named items into scope; no chess search runs here. |
| 26 | <code>use evaluation::evaluate;</code> | Bring the named items into scope; no chess search runs here. |
| 27 | <code>use make&#95;move::make&#95;move;</code> | Bring the named items into scope; no chess search runs here. |
| 28 | <code>use move&#95;generation::{MoveList, generate&#95;moves};</code> | Bring the named items into scope; no chess search runs here. |
| 29 | <code>use opening&#95;book::{BookLine, book&#95;moves, load&#95;book};</code> | Bring the named items into scope; no chess search runs here. |
| 30 | <code>use search::{DEFAULT&#95;THREADS, MAX&#95;THREADS, Search};</code> | Bring the named items into scope; no chess search runs here. |
| 31 | <code>use search&#95;control::{LOG&#95;ENABLED, Mode, PonderCredit, SEARCH&#95;LOG&#95;ENABLED, log};</code> | Bring the named items into scope; no chess search runs here. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>const STACK&#95;BYTES: usize = 64 &#42; 1024 &#42; 1024; // Room for the deepest search</code> | Define STACK&#95;BYTES, including its explicit type and initial value. |
| 34 | <code>const HIT&#95;GAP: f64 = 2.0; // Seconds from a stopped ponder search to a go that counts as a hit</code> | Define HIT&#95;GAP, including its explicit type and initial value. |
| 35 | <code></code> | Blank line separating related declarations or steps. |
| 36 | <code>/// The position of a ponder search that ended with stop, what that search earned</code> | Source explanation: The position of a ponder search that ended with stop, what that search earned |
| 37 | <code>/// (its duration, depth, score, and signs of doubt), and when it ended.</code> | Source explanation: (its duration, depth, score, and signs of doubt), and when it ended. |
| 38 | <code>struct Pondered {</code> | Declare the struct Pondered used by this module. |
| 39 | <code>    position&#95;words: Vec&lt;String&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 40 | <code>    credit: PonderCredit,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 41 | <code>    ended: Instant,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 42 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 43 | <code></code> | Blank line separating related declarations or steps. |
| 44 | <code>/// Return the number after a word of &quot;go&quot;, such as wtime, or 0.</code> | Source explanation: Return the number after a word of &quot;go&quot;, such as wtime, or 0. |
| 45 | <code>fn read&#95;number(words: &amp;&#91;&amp;str&#93;, name: &amp;str) -&gt; i64 {</code> | Declare read&#95;number(); its parameters and return type define the interface. |
| 46 | <code>    for i in 0..words.len().saturating&#95;sub(1) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 47 | <code>        if words&#91;i&#93; == name {</code> | Test the condition and execute this branch only when it matches. |
| 48 | <code>            return words&#91;i + 1&#93;.parse().unwrap&#95;or(0);</code> | Leave the current function with the displayed result. |
| 49 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 50 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code>    0</code> | Evaluate or continue this expression in read&#95;number(); a final expression without a semicolon supplies its block&#x27;s value. |
| 52 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 53 | <code></code> | Blank line separating related declarations or steps. |
| 54 | <code>/// Return the move that UCI text such as &quot;e7e8q&quot; names.</code> | Source explanation: Return the move that UCI text such as &quot;e7e8q&quot; names. |
| 55 | <code>fn parse&#95;move(text: &amp;str) -&gt; Move {</code> | Declare parse&#95;move(); its parameters and return type define the interface. |
| 56 | <code>    let promotion = text</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 57 | <code>        .chars()</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 58 | <code>        .nth(4)</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 59 | <code>        .and&#95;then(&#124;letter&#124; &quot;.pnbrqk&quot;.find(letter))</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 60 | <code>        .unwrap&#95;or(0) as u8;</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>    new&#95;move(</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>        parse&#95;square(&amp;text&#91;0..2&#93;),</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 63 | <code>        parse&#95;square(&amp;text&#91;2..4&#93;),</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>        promotion,</code> | Evaluate or continue this expression in parse&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>    )</code> | Close the preceding block, expression, call, or collection. |
| 66 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code></code> | Blank line separating related declarations or steps. |
| 68 | <code>/// Return a legal move from the opening book, or NO&#95;MOVE.</code> | Source explanation: Return a legal move from the opening book, or NO&#95;MOVE. |
| 69 | <code>fn legal&#95;book&#95;move(book: &amp;&#91;BookLine&#93;, position: &amp;Position, played: &amp;&#91;&amp;str&#93;) -&gt; Move {</code> | Declare legal&#95;book&#95;move(); its parameters and return type define the interface. |
| 70 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 71 | <code>    generate&#95;moves(position, &amp;mut list);</code> | Evaluate or continue this expression in legal&#95;book&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 72 | <code>    for text in book&#95;moves(book, played, position.white) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 73 | <code>        for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 74 | <code>            if move&#95;text(mv) == text &amp;&amp; !in&#95;check(&amp;make&#95;move(position, mv), position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 75 | <code>                return mv;</code> | Leave the current function with the displayed result. |
| 76 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 77 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 78 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 79 | <code>    NO&#95;MOVE</code> | Evaluate or continue this expression in legal&#95;book&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 80 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 81 | <code></code> | Blank line separating related declarations or steps. |
| 82 | <code>/// Count the leaf positions of the legal move tree.</code> | Source explanation: Count the leaf positions of the legal move tree. |
| 83 | <code>fn perft(position: &amp;Position, depth: u32) -&gt; u64 {</code> | Declare perft(); its parameters and return type define the interface. |
| 84 | <code>    if depth == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 85 | <code>        return 1;</code> | Leave the current function with the displayed result. |
| 86 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 87 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 88 | <code>    generate&#95;moves(position, &amp;mut list);</code> | Evaluate or continue this expression in perft(); a final expression without a semicolon supplies its block&#x27;s value. |
| 89 | <code>    let mut count = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 90 | <code>    for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 91 | <code>        let child = make&#95;move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 92 | <code>        if !in&#95;check(&amp;child, position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 93 | <code>            count += perft(&amp;child, depth - 1);</code> | Update the named state using the expression and assignment operator shown. |
| 94 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 95 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 96 | <code>    count</code> | Evaluate or continue this expression in perft(); a final expression without a semicolon supplies its block&#x27;s value. |
| 97 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 98 | <code></code> | Blank line separating related declarations or steps. |
| 99 | <code>/// Return the preferred legal move, or a legal fallback after an immediate stop.</code> | Source explanation: Return the preferred legal move, or a legal fallback after an immediate stop. |
| 100 | <code>fn legal&#95;choice(position: &amp;Position, preferred: Move) -&gt; Move {</code> | Declare legal&#95;choice(); its parameters and return type define the interface. |
| 101 | <code>    let mut first = NO&#95;MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 103 | <code>    generate&#95;moves(position, &amp;mut list);</code> | Evaluate or continue this expression in legal&#95;choice(); a final expression without a semicolon supplies its block&#x27;s value. |
| 104 | <code>    for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 105 | <code>        if !in&#95;check(&amp;make&#95;move(position, mv), position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 106 | <code>            if mv == preferred {</code> | Test the condition and execute this branch only when it matches. |
| 107 | <code>                return mv;</code> | Leave the current function with the displayed result. |
| 108 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 109 | <code>            if first == NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 110 | <code>                first = mv;</code> | Update the named state using the expression and assignment operator shown. |
| 111 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 112 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 113 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 114 | <code>    first</code> | Evaluate or continue this expression in legal&#95;choice(); a final expression without a semicolon supplies its block&#x27;s value. |
| 115 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 116 | <code></code> | Blank line separating related declarations or steps. |
| 117 | <code>fn run() {</code> | Declare run(); its parameters and return type define the interface. |
| 118 | <code>    let book = load&#95;book();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 119 | <code>    let mut tablebase = tablebase::Tablebase::new(tablebase::ENDPOINT);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 120 | <code>    let mut use&#95;tablebase = true;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 121 | <code>    let mut use&#95;ponder = false;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 122 | <code>    let mut ponder&#95;diagnostics = false;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 123 | <code>    let mut search = Search::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 124 | <code>    let mut position = Position::start();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 125 | <code>    let mut played: Vec&lt;String&gt; = Vec::new(); // The moves of a game from the start</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 126 | <code>    let mut use&#95;book = true;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 127 | <code>    let mut game&#95;history: Vec&lt;u64&gt; = Vec::new(); // The keys of the positions before this one</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 128 | <code>    let mut out = std::io::stdout();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 129 | <code>    let mut pondered: Option&lt;Pondered&gt; = None;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 130 | <code>    let mut position&#95;words = vec!&#91;&quot;position&quot;.to&#95;string(), &quot;startpos&quot;.to&#95;string()&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 131 | <code></code> | Blank line separating related declarations or steps. |
| 132 | <code>    loop {</code> | Repeat until an explicit break, return, or other control transfer ends the loop. |
| 133 | <code>        let line = search.control().next&#95;command();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 134 | <code>        let words: Vec&lt;&amp;str&gt; = line.split&#95;whitespace().collect();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 135 | <code>        if words.is&#95;empty() {</code> | Test the condition and execute this branch only when it matches. |
| 136 | <code>            continue;</code> | Skip the rest of this iteration and advance the loop. |
| 137 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 138 | <code>        match words&#91;0&#93; {</code> | Select an arm by matching the expression against the listed patterns. |
| 139 | <code>            &quot;uci&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 140 | <code>                let &#95; = writeln!(out, &quot;id name Remus8&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 141 | <code>                let &#95; = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 142 | <code>                    out,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 143 | <code>                    &quot;option name Hash type combo default 512 var 512 var 1024 var 2048&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 144 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 145 | <code>                let &#95; = writeln!(out, &quot;option name LichessTablebase type check default true&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 146 | <code>                let &#95; = writeln!(out, &quot;option name Ponder type check default false&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 147 | <code>                let &#95; = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 148 | <code>                    out,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 149 | <code>                    &quot;option name PonderHitMinPercent type spin default 25 min 0 max 100&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 150 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 151 | <code>                let &#95; = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 152 | <code>                    out,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 153 | <code>                    &quot;option name SearchDiagnostics type check default false&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 154 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>                let &#95; = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 156 | <code>                    out,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 157 | <code>                    &quot;option name Threads type spin default {DEFAULT&#95;THREADS} min 1 max {MAX&#95;THREADS}&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 158 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 159 | <code>                let &#95; = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 160 | <code>                    out,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 161 | <code>                    &quot;option name PonderDiagnostics type check default false&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 163 | <code>                let &#95; = writeln!(out, &quot;uciok&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 164 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 165 | <code>            &quot;setoption&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 166 | <code>                pondered = None;</code> | Update the named state using the expression and assignment operator shown. |
| 167 | <code>                if words.len() == 5 &amp;&amp; words&#91;1&#93; == &quot;name&quot; &amp;&amp; words&#91;3&#93; == &quot;value&quot; {</code> | Test the condition and execute this branch only when it matches. |
| 168 | <code>                    let enabled = words&#91;4&#93;.eq&#95;ignore&#95;ascii&#95;case(&quot;true&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 169 | <code>                    match words&#91;2&#93;.to&#95;ascii&#95;lowercase().as&#95;str() {</code> | Select an arm by matching the expression against the listed patterns. |
| 170 | <code>                        &quot;lichesstablebase&quot; =&gt; use&#95;tablebase = enabled,</code> | Handle this matching pattern with the expression or block on its right. |
| 171 | <code>                        &quot;ponder&quot; =&gt; use&#95;ponder = enabled,</code> | Handle this matching pattern with the expression or block on its right. |
| 172 | <code>                        &quot;hash&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 173 | <code>                            if let Ok(mib @ (512 &#124; 1024 &#124; 2048)) = words&#91;4&#93;.parse::&lt;usize&gt;() {</code> | Test the condition and execute this branch only when it matches. |
| 174 | <code>                                search.set&#95;hash(mib);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 175 | <code>                            }</code> | Close the preceding block, expression, call, or collection. |
| 176 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 177 | <code>                        &quot;ponderhitminpercent&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 178 | <code>                            search.control().hit&#95;min&#95;percent =</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>                                words&#91;4&#93;.parse::&lt;u32&gt;().unwrap&#95;or(25).min(100);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 180 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 181 | <code>                        &quot;searchdiagnostics&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 182 | <code>                            search.diagnostics = enabled;</code> | Update the named state using the expression and assignment operator shown. |
| 183 | <code>                            SEARCH&#95;LOG&#95;ENABLED.store(enabled, std::sync::atomic::Ordering::Relaxed);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 184 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 185 | <code>                        &quot;ponderdiagnostics&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 186 | <code>                            ponder&#95;diagnostics = enabled;</code> | Update the named state using the expression and assignment operator shown. |
| 187 | <code>                            LOG&#95;ENABLED.store(enabled, std::sync::atomic::Ordering::Relaxed);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 188 | <code>                            log(&quot;PonderDiagnostics on&quot;);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 189 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 190 | <code>                        &quot;threads&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 191 | <code>                            search.set&#95;threads(words&#91;4&#93;.parse().unwrap&#95;or(DEFAULT&#95;THREADS))</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 192 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 193 | <code>                        &#95; =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 194 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 195 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 196 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 197 | <code>            &quot;ucinewgame&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 198 | <code>                pondered = None;</code> | Update the named state using the expression and assignment operator shown. |
| 199 | <code>                position&#95;words = vec!&#91;&quot;position&quot;.to&#95;string(), &quot;startpos&quot;.to&#95;string()&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 200 | <code>                position = Position::start();</code> | Update the named state using the expression and assignment operator shown. |
| 201 | <code>                played.clear();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 202 | <code>                game&#95;history.clear();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 203 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 204 | <code>            &quot;isready&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 205 | <code>                let &#95; = writeln!(out, &quot;readyok&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 206 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 207 | <code>            &quot;position&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 208 | <code>                position&#95;words = words.iter().map(&#124;word&#124; (&#42;word).to&#95;string()).collect();</code> | Update the named state using the expression and assignment operator shown. |
| 209 | <code>                // &quot;position startpos moves e2e4 e7e5&quot; or &quot;position fen ... moves e2e4&quot;</code> | Source explanation: &quot;position startpos moves e2e4 e7e5&quot; or &quot;position fen ... moves e2e4&quot; |
| 210 | <code>                let moves&#95;at = words.iter().position(&#124;word&#124; &#42;word == &quot;moves&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 211 | <code>                use&#95;book = words.get(1) == Some(&amp;&quot;startpos&quot;);</code> | Update the named state using the expression and assignment operator shown. |
| 212 | <code>                position = if use&#95;book {</code> | Update the named state using the expression and assignment operator shown. |
| 213 | <code>                    Position::start()</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 214 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 215 | <code>                    Position::from&#95;fen(&amp;words&#91;2..moves&#95;at.unwrap&#95;or(words.len())&#93;)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 216 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 217 | <code>                played.clear();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 218 | <code>                game&#95;history.clear();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 219 | <code>                if let Some(at) = moves&#95;at {</code> | Test the condition and execute this branch only when it matches. |
| 220 | <code>                    for text in &amp;words&#91;at + 1..&#93; {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 221 | <code>                        game&#95;history.push(position.key);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 222 | <code>                        played.push((&#42;text).to&#95;string());</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 223 | <code>                        position = make&#95;move(&amp;position, parse&#95;move(text));</code> | Update the named state using the expression and assignment operator shown. |
| 224 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 225 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 226 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 227 | <code>            &quot;go&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 228 | <code>                let previous&#95;ponder = pondered.take();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 229 | <code>                if words.contains(&amp;&quot;perft&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 230 | <code>                    let count = perft(&amp;position, read&#95;number(&amp;words, &quot;perft&quot;) as u32);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 231 | <code>                    let &#95; = writeln!(out, &quot;perft {count}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 232 | <code>                    continue;</code> | Skip the rest of this iteration and advance the loop. |
| 233 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 234 | <code>                let depth&#95;limit = if words.contains(&amp;&quot;depth&quot;) {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 235 | <code>                    Some(read&#95;number(&amp;words, &quot;depth&quot;) as i32)</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 236 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 237 | <code>                    None</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 238 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 239 | <code>                let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 240 | <code>                let clock&#95;left = read&#95;number(&amp;words, if white { &quot;wtime&quot; } else { &quot;btime&quot; }) as f64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 241 | <code>                let increment = read&#95;number(&amp;words, if white { &quot;winc&quot; } else { &quot;binc&quot; }) as f64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 242 | <code>                // Plan a thirtieth of the clock plus half the increment, and never more</code> | Source explanation: Plan a thirtieth of the clock plus half the increment, and never more |
| 243 | <code>                // than three times that or a fifth of the clock</code> | Source explanation: than three times that or a fifth of the clock |
| 244 | <code>                let mut seconds = 2.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 245 | <code>                let mut longest = 2.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 246 | <code>                if clock&#95;left &gt; 0.0 {</code> | Test the condition and execute this branch only when it matches. |
| 247 | <code>                    seconds = (clock&#95;left / 30.0 + increment / 2.0) / 1000.0;</code> | Update the named state using the expression and assignment operator shown. |
| 248 | <code>                    longest = (3.0 &#42; seconds).min(clock&#95;left / 5000.0);</code> | Update the named state using the expression and assignment operator shown. |
| 249 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 250 | <code>                if words.contains(&amp;&quot;movetime&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 251 | <code>                    seconds = read&#95;number(&amp;words, &quot;movetime&quot;) as f64 / 1000.0;</code> | Update the named state using the expression and assignment operator shown. |
| 252 | <code>                    longest = seconds;</code> | Update the named state using the expression and assignment operator shown. |
| 253 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 254 | <code>                let mode = if words.contains(&amp;&quot;ponder&quot;) {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 255 | <code>                    Mode::Ponder</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 256 | <code>                } else if words.contains(&amp;&quot;infinite&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 257 | <code>                    Mode::Infinite</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 258 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 259 | <code>                    Mode::Normal</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 260 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 261 | <code>                // A go on the position just pondered, right after its stop, is a hit</code> | Source explanation: A go on the position just pondered, right after its stop, is a hit |
| 262 | <code>                let mut credit = None;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 263 | <code>                if let Some(last) = previous&#95;ponder</code> | Test the condition and execute this branch only when it matches. |
| 264 | <code>                    &amp;&amp; mode == Mode::Normal</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 265 | <code>                    &amp;&amp; depth&#95;limit.is&#95;none()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 266 | <code>                    &amp;&amp; last.position&#95;words == position&#95;words</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 267 | <code>                    &amp;&amp; last.ended.elapsed().as&#95;secs&#95;f64() &lt; HIT&#95;GAP</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 268 | <code>                {</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 269 | <code>                    credit = Some(last.credit);</code> | Update the named state using the expression and assignment operator shown. |
| 270 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 271 | <code>                search.control().begin(</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 272 | <code>                    mode,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 273 | <code>                    if depth&#95;limit.is&#95;some() {</code> | Test the condition and execute this branch only when it matches. |
| 274 | <code>                        f64::INFINITY</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 275 | <code>                    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 276 | <code>                        longest</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 277 | <code>                    },</code> | Close the preceding block, expression, call, or collection. |
| 278 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 279 | <code>                // A hit may spend extra time only from a clock with a minute in hand</code> | Source explanation: A hit may spend extra time only from a clock with a minute in hand |
| 280 | <code>                search.control().can&#95;reinvest =</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 281 | <code>                    clock&#95;left &gt;= 60&#95;000.0 &amp;&amp; !words.contains(&amp;&quot;movetime&quot;);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 282 | <code>                if let Some(credit) = credit {</code> | Test the condition and execute this branch only when it matches. |
| 283 | <code>                    search.control().credit&#95;ponder(credit);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 284 | <code>                    log(&amp;format!(</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 285 | <code>                        &quot;stop and go on the pondered position: hit after {:.1} s of pondering to depth {}&quot;,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 286 | <code>                        credit.seconds.as&#95;secs&#95;f64(),</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 287 | <code>                        credit.depth</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 288 | <code>                    ));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 289 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 290 | <code>                let started = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 291 | <code>                search.control().log&#95;search(&amp;format!(</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 292 | <code>                    &quot;begin root&#95;key={:016x} white={} halfmove={} history&#95;plies={} plan&#95;s={seconds:.3} hard&#95;s={longest:.3} command={line}&quot;,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 293 | <code>                    position.key, position.white, position.halfmove, game&#95;history.len()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 294 | <code>                ));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 295 | <code>                log(&amp;format!(</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 296 | <code>                    &quot;plan seconds={seconds:.2} longest={longest:.2} ponder={}&quot;,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 297 | <code>                    mode == Mode::Ponder</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 298 | <code>                ));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 299 | <code>                let mut mv = NO&#95;MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 300 | <code>                let mut source = &quot;native&quot;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 301 | <code>                if use&#95;tablebase</code> | Test the condition and execute this branch only when it matches. |
| 302 | <code>                    &amp;&amp; depth&#95;limit.is&#95;none()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 303 | <code>                    &amp;&amp; mode == Mode::Normal</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 304 | <code>                    &amp;&amp; !search.control().was&#95;ponder</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 305 | <code>                {</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 306 | <code>                    mv = tablebase.lookup(&amp;position, &amp;game&#95;history, longest);</code> | Update the named state using the expression and assignment operator shown. |
| 307 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 308 | <code>                if mv != NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 309 | <code>                    source = &quot;tablebase&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 310 | <code>                    let &#95; = writeln!(out, &quot;info string Lichess tablebase&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 311 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 312 | <code>                // A book move also answers a ponder request and a hit: the engine</code> | Source explanation: A book move also answers a ponder request and a hit: the engine |
| 313 | <code>                // then waits for the hit or stop without searching</code> | Source explanation: then waits for the hit or stop without searching |
| 314 | <code>                if mv == NO&#95;MOVE &amp;&amp; use&#95;book &amp;&amp; depth&#95;limit.is&#95;none() &amp;&amp; mode != Mode::Infinite {</code> | Test the condition and execute this branch only when it matches. |
| 315 | <code>                    let texts: Vec&lt;&amp;str&gt; = played.iter().map(String::as&#95;str).collect();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 316 | <code>                    mv = legal&#95;book&#95;move(&amp;book, &amp;position, &amp;texts);</code> | Update the named state using the expression and assignment operator shown. |
| 317 | <code>                    if mv != NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 318 | <code>                        source = &quot;book&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 319 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 320 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 321 | <code>                search.control().source = source;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 322 | <code>                if mv == NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 323 | <code>                    let elapsed = started.elapsed().as&#95;secs&#95;f64();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 324 | <code>                    mv = search.think(</code> | Update the named state using the expression and assignment operator shown. |
| 325 | <code>                        &amp;position,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 326 | <code>                        &amp;game&#95;history,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 327 | <code>                        (seconds - elapsed).max(0.0),</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 328 | <code>                        (longest - elapsed).max(0.0),</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 329 | <code>                        depth&#95;limit,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 330 | <code>                    );</code> | Close the preceding block, expression, call, or collection. |
| 331 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 332 | <code>                mv = legal&#95;choice(&amp;position, mv);</code> | Update the named state using the expression and assignment operator shown. |
| 333 | <code>                search.control().wait();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 334 | <code>                if let Some(credit) = search.control().ponder&#95;credit() {</code> | Test the condition and execute this branch only when it matches. |
| 335 | <code>                    pondered = Some(Pondered {</code> | Update the named state using the expression and assignment operator shown. |
| 336 | <code>                        position&#95;words: position&#95;words.clone(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 337 | <code>                        credit,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 338 | <code>                        ended: Instant::now(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 339 | <code>                    });</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 340 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 341 | <code>                if ponder&#95;diagnostics {</code> | Test the condition and execute this branch only when it matches. |
| 342 | <code>                    search.control().report();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 343 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 344 | <code>                // With no legal move (checkmate or stalemate) UCI says &quot;0000&quot;</code> | Source explanation: With no legal move (checkmate or stalemate) UCI says &quot;0000&quot; |
| 345 | <code>                let mut text = if mv == NO&#95;MOVE {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 346 | <code>                    &quot;0000&quot;.to&#95;string()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 347 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 348 | <code>                    move&#95;text(mv)</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 349 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 350 | <code>                if use&#95;ponder &amp;&amp; mv != NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 351 | <code>                    let mut reply = search.ponder&#95;reply(&amp;position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 352 | <code>                    let child = make&#95;move(&amp;position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 353 | <code>                    if reply == NO&#95;MOVE &amp;&amp; use&#95;book {</code> | Test the condition and execute this branch only when it matches. |
| 354 | <code>                        let mut continuation = played.clone();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 355 | <code>                        continuation.push(move&#95;text(mv));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 356 | <code>                        let texts: Vec&lt;&amp;str&gt; = continuation.iter().map(String::as&#95;str).collect();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 357 | <code>                        reply = legal&#95;book&#95;move(&amp;book, &amp;child, &amp;texts);</code> | Update the named state using the expression and assignment operator shown. |
| 358 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 359 | <code>                    reply = legal&#95;choice(&amp;child, reply);</code> | Update the named state using the expression and assignment operator shown. |
| 360 | <code>                    if reply != NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 361 | <code>                        text.push&#95;str(&amp;format!(&quot; ponder {}&quot;, move&#95;text(reply)));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 362 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 363 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 364 | <code>                let &#95; = writeln!(out, &quot;bestmove {text}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 365 | <code>                let &#95; = out.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 366 | <code>                log(&amp;format!(&quot;&gt; bestmove {text}&quot;));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 367 | <code>                search</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 368 | <code>                    .control()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 369 | <code>                    .log&#95;search(&amp;format!(&quot;result source={source} bestmove {text}&quot;));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 370 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 371 | <code>            &quot;eval&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 372 | <code>                let &#95; = writeln!(out, &quot;eval {}&quot;, evaluate(&amp;position));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 373 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 374 | <code>            &quot;quit&quot; =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 375 | <code>            &#95; =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 376 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 377 | <code>        let &#95; = out.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 378 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 379 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 380 | <code></code> | Blank line separating related declarations or steps. |
| 381 | <code>fn main() {</code> | Declare main(); its parameters and return type define the interface. |
| 382 | <code>    // The search calls itself once per ply, so it runs on a thread with a large stack</code> | Source explanation: The search calls itself once per ply, so it runs on a thread with a large stack |
| 383 | <code>    let worker = std::thread::Builder::new()</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 384 | <code>        .stack&#95;size(STACK&#95;BYTES)</code> | Evaluate or continue this expression in main(); a final expression without a semicolon supplies its block&#x27;s value. |
| 385 | <code>        .spawn(run);</code> | Evaluate or continue this expression in main(); a final expression without a semicolon supplies its block&#x27;s value. |
| 386 | <code>    if let Ok(handle) = worker {</code> | Test the condition and execute this branch only when it matches. |
| 387 | <code>        let &#95; = handle.join();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 388 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 389 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### make_move.rs

Copy a position and apply captures, castling, promotion, and state changes.

[Open source](src/make_move.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Play a move. As in Remus5, make&#95;move() copies the position, plays the move on the</code> | Source explanation: Play a move. As in Remus5, make&#95;move() copies the position, plays the move on the |
| 2 | <code>//! copy, and returns the new position, so the search never takes a move back.</code> | Source explanation: copy, and returns the new position, so the search never takes a move back. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use crate::board::{EMPTY, KEYS, KING, Move, PAWN, Position, WHITE};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use crate::board::{move&#95;promotion, move&#95;start, move&#95;target};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>// The castling rights that a move from or to each square turns off for good</code> | Source explanation: The castling rights that a move from or to each square turns off for good |
| 8 | <code>const fn build&#95;rights() -&gt; &#91;u8; 64&#93; {</code> | Declare build&#95;rights(); its parameters and return type define the interface. |
| 9 | <code>    let mut table = &#91;0; 64&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 10 | <code>    table&#91;60&#93; = 3; // e1</code> | Update the named state using the expression and assignment operator shown. |
| 11 | <code>    table&#91;63&#93; = 1; // h1</code> | Update the named state using the expression and assignment operator shown. |
| 12 | <code>    table&#91;56&#93; = 2; // a1</code> | Update the named state using the expression and assignment operator shown. |
| 13 | <code>    table&#91;4&#93; = 12; // e8</code> | Update the named state using the expression and assignment operator shown. |
| 14 | <code>    table&#91;7&#93; = 4; // h8</code> | Update the named state using the expression and assignment operator shown. |
| 15 | <code>    table&#91;0&#93; = 8; // a8</code> | Update the named state using the expression and assignment operator shown. |
| 16 | <code>    table</code> | Evaluate or continue this expression in build&#95;rights(); a final expression without a semicolon supplies its block&#x27;s value. |
| 17 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 18 | <code></code> | Blank line separating related declarations or steps. |
| 19 | <code>static RIGHTS&#95;OF&#95;SQUARE: &#91;u8; 64&#93; = build&#95;rights();</code> | Define RIGHTS&#95;OF&#95;SQUARE, including its explicit type and initial value. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>/// Return the position after the move, including the special moves.</code> | Source explanation: Return the position after the move, including the special moves. |
| 22 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 23 | <code>pub fn make&#95;move(position: &amp;Position, mv: Move) -&gt; Position {</code> | Declare make&#95;move(); its parameters and return type define the interface. |
| 24 | <code>    let mut next = &#42;position;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 25 | <code>    let start = move&#95;start(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 26 | <code>    let target = move&#95;target(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 27 | <code>    let promotion = move&#95;promotion(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 28 | <code>    let mut piece = next.board&#91;start&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 29 | <code>    let kind = piece &amp; 7;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 30 | <code>    let is&#95;pawn = kind == PAWN;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 31 | <code>    let distance = start.abs&#95;diff(target);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>    // A capture or pawn move restarts the fifty-move count</code> | Source explanation: A capture or pawn move restarts the fifty-move count |
| 34 | <code>    if is&#95;pawn &#124;&#124; next.board&#91;target&#93; != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 35 | <code>        next.halfmove = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 36 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 37 | <code>        next.halfmove += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 38 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 39 | <code>    // En passant: the captured pawn stands beside the pawn that moves</code> | Source explanation: En passant: the captured pawn stands beside the pawn that moves |
| 40 | <code>    if is&#95;pawn &amp;&amp; target as i8 == next.en&#95;passant {</code> | Test the condition and execute this branch only when it matches. |
| 41 | <code>        next.remove&#95;piece(start / 8 &#42; 8 + target % 8);</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 42 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 43 | <code>    // Castling: the king moves two squares, and the rook jumps to the square it crossed</code> | Source explanation: Castling: the king moves two squares, and the rook jumps to the square it crossed |
| 44 | <code>    if kind == KING &amp;&amp; distance == 2 {</code> | Test the condition and execute this branch only when it matches. |
| 45 | <code>        let rook = if target &gt; start { start + 3 } else { start - 4 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>        let rook&#95;piece = next.board&#91;rook&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>        next.remove&#95;piece(rook);</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 48 | <code>        next.put&#95;piece((start + target) / 2, rook&#95;piece);</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 49 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 50 | <code>    // A two-square pawn move lets an enemy pawn capture it en passant on the next move</code> | Source explanation: A two-square pawn move lets an enemy pawn capture it en passant on the next move |
| 51 | <code>    if next.en&#95;passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 52 | <code>        next.key ^= KEYS.en&#95;passant&#91;next.en&#95;passant as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 54 | <code>    next.en&#95;passant = -1;</code> | Update the named state using the expression and assignment operator shown. |
| 55 | <code>    if is&#95;pawn &amp;&amp; distance == 16 {</code> | Test the condition and execute this branch only when it matches. |
| 56 | <code>        let skipped = (start + target) / 2;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 57 | <code>        next.en&#95;passant = skipped as i8;</code> | Update the named state using the expression and assignment operator shown. |
| 58 | <code>        next.key ^= KEYS.en&#95;passant&#91;skipped&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 59 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 60 | <code>    if promotion != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>        piece = promotion + (piece &amp; WHITE);</code> | Update the named state using the expression and assignment operator shown. |
| 62 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 63 | <code>    if next.board&#91;target&#93; != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 64 | <code>        next.remove&#95;piece(target);</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 66 | <code>    next.remove&#95;piece(start);</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>    next.put&#95;piece(target, piece);</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code></code> | Blank line separating related declarations or steps. |
| 69 | <code>    let rights = next.castling &amp; !(RIGHTS&#95;OF&#95;SQUARE&#91;start&#93; &#124; RIGHTS&#95;OF&#95;SQUARE&#91;target&#93;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 70 | <code>    if rights != next.castling {</code> | Test the condition and execute this branch only when it matches. |
| 71 | <code>        next.key ^= KEYS.castling&#91;next.castling as usize&#93; ^ KEYS.castling&#91;rights as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 72 | <code>        next.castling = rights;</code> | Update the named state using the expression and assignment operator shown. |
| 73 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code>    next.white = !next.white;</code> | Update the named state using the expression and assignment operator shown. |
| 75 | <code>    next.key ^= KEYS.black;</code> | Update the named state using the expression and assignment operator shown. |
| 76 | <code>    next</code> | Evaluate or continue this expression in make&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 77 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 78 | <code></code> | Blank line separating related declarations or steps. |
| 79 | <code>/// Return the position with the other side to move and no en passant square.</code> | Source explanation: Return the position with the other side to move and no en passant square. |
| 80 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 81 | <code>pub fn make&#95;null&#95;move(position: &amp;Position) -&gt; Position {</code> | Declare make&#95;null&#95;move(); its parameters and return type define the interface. |
| 82 | <code>    let mut next = &#42;position;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 83 | <code>    if next.en&#95;passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 84 | <code>        next.key ^= KEYS.en&#95;passant&#91;next.en&#95;passant as usize&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 85 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 86 | <code>    next.en&#95;passant = -1;</code> | Update the named state using the expression and assignment operator shown. |
| 87 | <code>    next.halfmove += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 88 | <code>    next.white = !next.white;</code> | Update the named state using the expression and assignment operator shown. |
| 89 | <code>    next.key ^= KEYS.black;</code> | Update the named state using the expression and assignment operator shown. |
| 90 | <code>    next</code> | Evaluate or continue this expression in make&#95;null&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 91 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### move_generation.rs

Generate candidate moves before the native king-safety filter.

[Open source](src/move_generation.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! List the moves of the side to move. As in Remus5 the list holds pseudo-legal moves:</code> | Source explanation: List the moves of the side to move. As in Remus5 the list holds pseudo-legal moves: |
| 2 | <code>//! some may leave the mover&#x27;s own king in check, and the search skips those after it</code> | Source explanation: some may leave the mover&#x27;s own king in check, and the search skips those after it |
| 3 | <code>//! makes them. The search sorts each list, so the order of generation does not matter.</code> | Source explanation: makes them. The search sorts each list, so the order of generation does not matter. |
| 4 | <code></code> | Blank line separating related declarations or steps. |
| 5 | <code>use std::mem::MaybeUninit;</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>use crate::attacks::{</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code>    KING&#95;ATTACKS, KNIGHT&#95;ATTACKS, PAWN&#95;ATTACKS, bishop&#95;attacks, is&#95;attacked, rook&#95;attacks,</code> | Continue the enclosing declaration or initialization expression. |
| 9 | <code>};</code> | Close the preceding block, expression, call, or collection. |
| 10 | <code>use crate::board::{BISHOP, EMPTY, KING, KNIGHT, Move, PAWN, Position, QUEEN, ROOK, new&#95;move};</code> | Bring the named items into scope; no chess search runs here. |
| 11 | <code></code> | Blank line separating related declarations or steps. |
| 12 | <code>/// A fixed-size list on the stack whose unused slots stay unwritten. The search makes</code> | Source explanation: A fixed-size list on the stack whose unused slots stay unwritten. The search makes |
| 13 | <code>/// two of these at every position, and most positions use only a few slots, so filling</code> | Source explanation: two of these at every position, and most positions use only a few slots, so filling |
| 14 | <code>/// them with zeros first would cost more than the moves themselves.</code> | Source explanation: them with zeros first would cost more than the moves themselves. |
| 15 | <code>pub struct StackList&lt;T: Copy, const SIZE: usize&gt; {</code> | Declare the struct StackList used by this module. |
| 16 | <code>    items: &#91;MaybeUninit&lt;T&gt;; SIZE&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 17 | <code>    count: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 18 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 19 | <code></code> | Blank line separating related declarations or steps. |
| 20 | <code>impl&lt;T: Copy, const SIZE: usize&gt; StackList&lt;T, SIZE&gt; {</code> | Begin the implementation of methods or a trait for the named type. |
| 21 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 22 | <code>    pub fn new() -&gt; Self {</code> | Declare new(); its parameters and return type define the interface. |
| 23 | <code>        StackList {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 24 | <code>            items: &#91;MaybeUninit::uninit(); SIZE&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 25 | <code>            count: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 26 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 27 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 28 | <code></code> | Blank line separating related declarations or steps. |
| 29 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 30 | <code>    pub fn add(&amp;mut self, item: T) {</code> | Declare add(); its parameters and return type define the interface. |
| 31 | <code>        self.items&#91;self.count&#93; = MaybeUninit::new(item);</code> | Update the named state using the expression and assignment operator shown. |
| 32 | <code>        self.count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 33 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 34 | <code></code> | Blank line separating related declarations or steps. |
| 35 | <code>    /// The items added so far, ready to read or sort.</code> | Source explanation: The items added so far, ready to read or sort. |
| 36 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 37 | <code>    pub fn as&#95;mut&#95;slice(&amp;mut self) -&gt; &amp;mut &#91;T&#93; {</code> | Declare as&#95;mut&#95;slice(); its parameters and return type define the interface. |
| 38 | <code>        // Slots 0 up to count hold items that add() wrote</code> | Source explanation: Slots 0 up to count hold items that add() wrote |
| 39 | <code>        unsafe { std::slice::from&#95;raw&#95;parts&#95;mut(self.items.as&#95;mut&#95;ptr().cast::&lt;T&gt;(), self.count) }</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 40 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 41 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code></code> | Blank line separating related declarations or steps. |
| 43 | <code>/// A list of moves, room for more than the 218 any chess position allows.</code> | Source explanation: A list of moves, room for more than the 218 any chess position allows. |
| 44 | <code>pub type MoveList = StackList&lt;Move, 256&gt;;</code> | Declare the type MoveList used by this module. |
| 45 | <code></code> | Blank line separating related declarations or steps. |
| 46 | <code>impl MoveList {</code> | Begin the implementation of methods or a trait for the named type. |
| 47 | <code>    /// Add one move for each bit of the targets.</code> | Source explanation: Add one move for each bit of the targets. |
| 48 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 49 | <code>    fn add&#95;targets(&amp;mut self, start: usize, mut targets: u64) {</code> | Declare add&#95;targets(); its parameters and return type define the interface. |
| 50 | <code>        while targets != 0 {</code> | Repeat this block while its condition remains true. |
| 51 | <code>            let target = targets.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 52 | <code>            targets &amp;= targets - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>            self.add(new&#95;move(start, target, 0));</code> | Evaluate or continue this expression in add&#95;targets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 55 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code></code> | Blank line separating related declarations or steps. |
| 57 | <code>    /// Add a pawn move, or its four promotions on the last rank.</code> | Source explanation: Add a pawn move, or its four promotions on the last rank. |
| 58 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 59 | <code>    fn add&#95;pawn&#95;move(&amp;mut self, start: usize, target: usize) {</code> | Declare add&#95;pawn&#95;move(); its parameters and return type define the interface. |
| 60 | <code>        if !(8..56).contains(&amp;target) {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>            self.add(new&#95;move(start, target, QUEEN));</code> | Evaluate or continue this expression in add&#95;pawn&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>            self.add(new&#95;move(start, target, ROOK));</code> | Evaluate or continue this expression in add&#95;pawn&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 63 | <code>            self.add(new&#95;move(start, target, BISHOP));</code> | Evaluate or continue this expression in add&#95;pawn&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>            self.add(new&#95;move(start, target, KNIGHT));</code> | Evaluate or continue this expression in add&#95;pawn&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 66 | <code>            self.add(new&#95;move(start, target, 0));</code> | Evaluate or continue this expression in add&#95;pawn&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 68 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 69 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 70 | <code></code> | Blank line separating related declarations or steps. |
| 71 | <code>/// Add the moves of the side&#x27;s knights, bishops, rooks, queens, and king onto the</code> | Source explanation: Add the moves of the side&#x27;s knights, bishops, rooks, queens, and king onto the |
| 72 | <code>/// squares in the mask.</code> | Source explanation: squares in the mask. |
| 73 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 74 | <code>fn add&#95;piece&#95;moves(position: &amp;Position, own: u64, occupied: u64, mask: u64, list: &amp;mut MoveList) {</code> | Declare add&#95;piece&#95;moves(); its parameters and return type define the interface. |
| 75 | <code>    let kinds = &amp;position.by&#95;kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 76 | <code>    let mut pieces = own &amp; kinds&#91;KNIGHT as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 77 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 78 | <code>        let start = pieces.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 79 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 80 | <code>        list.add&#95;targets(start, KNIGHT&#95;ATTACKS&#91;start&#93; &amp; mask);</code> | Evaluate or continue this expression in add&#95;piece&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 82 | <code>    let mut pieces = own &amp; kinds&#91;BISHOP as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 83 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 84 | <code>        let start = pieces.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 85 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 86 | <code>        list.add&#95;targets(start, bishop&#95;attacks(start, occupied) &amp; mask);</code> | Evaluate or continue this expression in add&#95;piece&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 87 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 88 | <code>    let mut pieces = own &amp; kinds&#91;ROOK as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 89 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 90 | <code>        let start = pieces.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 91 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 92 | <code>        list.add&#95;targets(start, rook&#95;attacks(start, occupied) &amp; mask);</code> | Evaluate or continue this expression in add&#95;piece&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 93 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code>    let mut pieces = own &amp; kinds&#91;QUEEN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 95 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 96 | <code>        let start = pieces.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 97 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 98 | <code>        let reach = rook&#95;attacks(start, occupied) &#124; bishop&#95;attacks(start, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 99 | <code>        list.add&#95;targets(start, reach &amp; mask);</code> | Evaluate or continue this expression in add&#95;piece&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 100 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 101 | <code>    let king = (own &amp; kinds&#91;KING as usize&#93;).trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>    list.add&#95;targets(king, KING&#95;ATTACKS&#91;king&#93; &amp; mask);</code> | Evaluate or continue this expression in add&#95;piece&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 103 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 104 | <code></code> | Blank line separating related declarations or steps. |
| 105 | <code>/// Add the castling moves. The search checks the square where the king lands.</code> | Source explanation: Add the castling moves. The search checks the square where the king lands. |
| 106 | <code>fn add&#95;castling&#95;moves(position: &amp;Position, list: &amp;mut MoveList) {</code> | Declare add&#95;castling&#95;moves(); its parameters and return type define the interface. |
| 107 | <code>    let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 108 | <code>    let board = &amp;position.board;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 109 | <code>    let king: usize = if white { 60 } else { 4 }; // e1 or e8</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 110 | <code>    let kingside: u8 = if white { 1 } else { 4 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 111 | <code>    let queenside = kingside &#42; 2;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 112 | <code>    let kingside&#95;open =</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 113 | <code>        position.castling &amp; kingside != 0 &amp;&amp; board&#91;king + 1&#93; == EMPTY &amp;&amp; board&#91;king + 2&#93; == EMPTY;</code> | Evaluate or continue this expression in add&#95;castling&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 114 | <code>    let queenside&#95;open = position.castling &amp; queenside != 0</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 115 | <code>        &amp;&amp; board&#91;king - 3&#93; == EMPTY</code> | Evaluate or continue this expression in add&#95;castling&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 116 | <code>        &amp;&amp; board&#91;king - 2&#93; == EMPTY</code> | Evaluate or continue this expression in add&#95;castling&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 117 | <code>        &amp;&amp; board&#91;king - 1&#93; == EMPTY;</code> | Evaluate or continue this expression in add&#95;castling&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 118 | <code>    if !(kingside&#95;open &#124;&#124; queenside&#95;open) &#124;&#124; is&#95;attacked(position, king, !white) {</code> | Test the condition and execute this branch only when it matches. |
| 119 | <code>        return;</code> | Leave the current function with the displayed result. |
| 120 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 121 | <code>    if kingside&#95;open &amp;&amp; !is&#95;attacked(position, king + 1, !white) {</code> | Test the condition and execute this branch only when it matches. |
| 122 | <code>        list.add(new&#95;move(king, king + 2, 0));</code> | Evaluate or continue this expression in add&#95;castling&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 123 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 124 | <code>    if queenside&#95;open &amp;&amp; !is&#95;attacked(position, king - 1, !white) {</code> | Test the condition and execute this branch only when it matches. |
| 125 | <code>        list.add(new&#95;move(king, king - 2, 0));</code> | Evaluate or continue this expression in add&#95;castling&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 126 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 127 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 128 | <code></code> | Blank line separating related declarations or steps. |
| 129 | <code>/// List the pseudo-legal moves of the side to move.</code> | Source explanation: List the pseudo-legal moves of the side to move. |
| 130 | <code>pub fn generate&#95;moves(position: &amp;Position, list: &amp;mut MoveList) {</code> | Declare generate&#95;moves(); its parameters and return type define the interface. |
| 131 | <code>    let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 132 | <code>    let side = usize::from(!white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 133 | <code>    let own = position.by&#95;side&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 134 | <code>    let enemy = position.by&#95;side&#91;1 - side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 135 | <code>    let occupied = own &#124; enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 136 | <code>    let mut pawns = own &amp; position.by&#95;kind&#91;PAWN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 137 | <code>    // A pawn may capture on the en passant square as well as onto an enemy piece</code> | Source explanation: A pawn may capture on the en passant square as well as onto an enemy piece |
| 138 | <code>    let mut pawn&#95;targets = enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 139 | <code>    if position.en&#95;passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 140 | <code>        pawn&#95;targets &#124;= 1&#95;u64 &lt;&lt; position.en&#95;passant;</code> | Update the named state using the expression and assignment operator shown. |
| 141 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 142 | <code>    let first&#95;row = if white { 6 } else { 1 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 143 | <code>    while pawns != 0 {</code> | Repeat this block while its condition remains true. |
| 144 | <code>        let start = pawns.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 145 | <code>        pawns &amp;= pawns - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 146 | <code>        // White pawns move toward row 0, and one row is 8 squares</code> | Source explanation: White pawns move toward row 0, and one row is 8 squares |
| 147 | <code>        let ahead = if white { start - 8 } else { start + 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 148 | <code>        if position.board&#91;ahead&#93; == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 149 | <code>            list.add&#95;pawn&#95;move(start, ahead);</code> | Evaluate or continue this expression in generate&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 150 | <code>            if start / 8 == first&#95;row {</code> | Test the condition and execute this branch only when it matches. |
| 151 | <code>                let two&#95;ahead = if white { ahead - 8 } else { ahead + 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 152 | <code>                if position.board&#91;two&#95;ahead&#93; == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 153 | <code>                    list.add(new&#95;move(start, two&#95;ahead, 0));</code> | Evaluate or continue this expression in generate&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 154 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 156 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 157 | <code>        let mut captures = PAWN&#95;ATTACKS&#91;side&#93;&#91;start&#93; &amp; pawn&#95;targets;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>        while captures != 0 {</code> | Repeat this block while its condition remains true. |
| 159 | <code>            let target = captures.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 160 | <code>            captures &amp;= captures - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 161 | <code>            list.add&#95;pawn&#95;move(start, target);</code> | Evaluate or continue this expression in generate&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 163 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 164 | <code>    add&#95;piece&#95;moves(position, own, occupied, !own, list);</code> | Evaluate or continue this expression in generate&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 165 | <code>    if position.castling != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 166 | <code>        add&#95;castling&#95;moves(position, list);</code> | Evaluate or continue this expression in generate&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 167 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 168 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 169 | <code></code> | Blank line separating related declarations or steps. |
| 170 | <code>/// List only the moves quiescence can search: captures and queen promotions. Remus5</code> | Source explanation: List only the moves quiescence can search: captures and queen promotions. Remus5 |
| 171 | <code>/// sorts every move and stops at the first quiet one, which tries these same moves.</code> | Source explanation: sorts every move and stops at the first quiet one, which tries these same moves. |
| 172 | <code>pub fn generate&#95;captures(position: &amp;Position, list: &amp;mut MoveList) {</code> | Declare generate&#95;captures(); its parameters and return type define the interface. |
| 173 | <code>    let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 174 | <code>    let side = usize::from(!white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 175 | <code>    let own = position.by&#95;side&#91;side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 176 | <code>    let enemy = position.by&#95;side&#91;1 - side&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 177 | <code>    let occupied = own &#124; enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 178 | <code>    let mut pawn&#95;targets = enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 179 | <code>    if position.en&#95;passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 180 | <code>        pawn&#95;targets &#124;= 1&#95;u64 &lt;&lt; position.en&#95;passant;</code> | Update the named state using the expression and assignment operator shown. |
| 181 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 182 | <code>    let mut pawns = own &amp; position.by&#95;kind&#91;PAWN as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 183 | <code>    while pawns != 0 {</code> | Repeat this block while its condition remains true. |
| 184 | <code>        let start = pawns.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 185 | <code>        pawns &amp;= pawns - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 186 | <code>        let ahead = if white { start - 8 } else { start + 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 187 | <code>        if !(8..56).contains(&amp;ahead) &amp;&amp; position.board&#91;ahead&#93; == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 188 | <code>            list.add(new&#95;move(start, ahead, QUEEN));</code> | Evaluate or continue this expression in generate&#95;captures(); a final expression without a semicolon supplies its block&#x27;s value. |
| 189 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 190 | <code>        let mut captures = PAWN&#95;ATTACKS&#91;side&#93;&#91;start&#93; &amp; pawn&#95;targets;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 191 | <code>        while captures != 0 {</code> | Repeat this block while its condition remains true. |
| 192 | <code>            let target = captures.trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 193 | <code>            captures &amp;= captures - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 194 | <code>            list.add&#95;pawn&#95;move(start, target);</code> | Evaluate or continue this expression in generate&#95;captures(); a final expression without a semicolon supplies its block&#x27;s value. |
| 195 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 196 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 197 | <code>    add&#95;piece&#95;moves(position, own, occupied, enemy, list);</code> | Evaluate or continue this expression in generate&#95;captures(); a final expression without a semicolon supplies its block&#x27;s value. |
| 198 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### opening_book.rs

Load embedded opening continuations for validation at the UCI boundary.

[Open source](src/opening_book.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Play a stored opening line while the game follows one. The compiler copies</code> | Source explanation: Play a stored opening line while the game follows one. The compiler copies |
| 2 | <code>//! opening&#95;book.txt into the program, so the book cannot go missing.</code> | Source explanation: opening&#95;book.txt into the program, so the book cannot go missing. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use std::time::{SystemTime, UNIX&#95;EPOCH};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code></code> | Blank line separating related declarations or steps. |
| 6 | <code>const BOOK&#95;TEXT: &amp;str = include&#95;str!(&quot;../opening&#95;book.txt&quot;);</code> | Define BOOK&#95;TEXT, including its explicit type and initial value. |
| 7 | <code></code> | Blank line separating related declarations or steps. |
| 8 | <code>/// One line of the book: the side it defends (&quot;&quot; for both) and its moves.</code> | Source explanation: One line of the book: the side it defends (&quot;&quot; for both) and its moves. |
| 9 | <code>pub struct BookLine {</code> | Declare the struct BookLine used by this module. |
| 10 | <code>    side: &amp;&#x27;static str,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 11 | <code>    moves: Vec&lt;&amp;&#x27;static str&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 12 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 13 | <code></code> | Blank line separating related declarations or steps. |
| 14 | <code>/// Read the book.</code> | Source explanation: Read the book. |
| 15 | <code>pub fn load&#95;book() -&gt; Vec&lt;BookLine&gt; {</code> | Declare load&#95;book(); its parameters and return type define the interface. |
| 16 | <code>    let mut book = Vec::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 17 | <code>    for text in BOOK&#95;TEXT.lines() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 18 | <code>        let mut side = &quot;&quot;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 19 | <code>        let mut line = text;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 20 | <code>        if let Some(rest) = text.strip&#95;prefix(&quot;White:&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 21 | <code>            side = &quot;White&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 22 | <code>            line = rest;</code> | Update the named state using the expression and assignment operator shown. |
| 23 | <code>        } else if let Some(rest) = text.strip&#95;prefix(&quot;Black:&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 24 | <code>            side = &quot;Black&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 25 | <code>            line = rest;</code> | Update the named state using the expression and assignment operator shown. |
| 26 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 27 | <code>        if !line.trim().is&#95;empty() &amp;&amp; !line.starts&#95;with(&#x27;#&#x27;) {</code> | Test the condition and execute this branch only when it matches. |
| 28 | <code>            book.push(BookLine {</code> | Evaluate or continue this expression in load&#95;book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 29 | <code>                side,</code> | Evaluate or continue this expression in load&#95;book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>                moves: line.split&#95;whitespace().collect(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 31 | <code>            });</code> | Evaluate or continue this expression in load&#95;book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 32 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 33 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 34 | <code>    book</code> | Evaluate or continue this expression in load&#95;book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 35 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 36 | <code></code> | Blank line separating related declarations or steps. |
| 37 | <code>/// Return a number that differs from one call to the next.</code> | Source explanation: Return a number that differs from one call to the next. |
| 38 | <code>fn clock&#95;seed() -&gt; u64 {</code> | Declare clock&#95;seed(); its parameters and return type define the interface. |
| 39 | <code>    let since = SystemTime::now().duration&#95;since(UNIX&#95;EPOCH);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 40 | <code>    since.map&#95;or(2016, &#124;time&#124; time.as&#95;nanos() as u64) &#124; 1</code> | Evaluate or continue this expression in clock&#95;seed(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code></code> | Blank line separating related declarations or steps. |
| 43 | <code>/// Return the book&#x27;s next moves after the moves played, in random order.</code> | Source explanation: Return the book&#x27;s next moves after the moves played, in random order. |
| 44 | <code>pub fn book&#95;moves(book: &amp;&#91;BookLine&#93;, played: &amp;&#91;&amp;str&#93;, white: bool) -&gt; Vec&lt;&amp;&#x27;static str&gt; {</code> | Declare book&#95;moves(); its parameters and return type define the interface. |
| 45 | <code>    let mut moves = Vec::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>    for line in book {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 47 | <code>        let defends = line.side.is&#95;empty() &#124;&#124; (line.side == &quot;White&quot;) == white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 48 | <code>        if defends &amp;&amp; line.moves.len() &gt; played.len() &amp;&amp; line.moves&#91;..played.len()&#93; == &#42;played {</code> | Test the condition and execute this branch only when it matches. |
| 49 | <code>            moves.push(line.moves&#91;played.len()&#93;);</code> | Evaluate or continue this expression in book&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 50 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code>    // Shuffle with a small xorshift generator seeded from the clock</code> | Source explanation: Shuffle with a small xorshift generator seeded from the clock |
| 53 | <code>    let mut state = clock&#95;seed();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 54 | <code>    for i in (1..moves.len()).rev() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 55 | <code>        state ^= state &lt;&lt; 13;</code> | Update the named state using the expression and assignment operator shown. |
| 56 | <code>        state ^= state &gt;&gt; 7;</code> | Update the named state using the expression and assignment operator shown. |
| 57 | <code>        state ^= state &lt;&lt; 17;</code> | Update the named state using the expression and assignment operator shown. |
| 58 | <code>        moves.swap(i, (state % (i as u64 + 1)) as usize);</code> | Evaluate or continue this expression in book&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 59 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 60 | <code>    moves</code> | Evaluate or continue this expression in book&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### parallel_search.rs

Coordinate worker searches and shared search tasks.

[Open source](src/parallel_search.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! YBWC splits use reserved idle workers and restricted descendant helping.</code> | Source explanation: YBWC splits use reserved idle workers and restricted descendant helping. |
| 2 | <code>//! Waiting owners may claim unassigned work strictly below their own split.</code> | Source explanation: Waiting owners may claim unassigned work strictly below their own split. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::Relaxed};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use std::sync::mpsc::Sender;</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use std::sync::{Arc, Condvar, Mutex, Weak};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code></code> | Blank line separating related declarations or steps. |
| 8 | <code>use crate::board::{Move, Position};</code> | Bring the named items into scope; no chess search runs here. |
| 9 | <code>use crate::search&#95;metrics::SearchMetrics;</code> | Bring the named items into scope; no chess search runs here. |
| 10 | <code>use crate::table&#95;metrics::TableMetrics;</code> | Bring the named items into scope; no chess search runs here. |
| 11 | <code></code> | Blank line separating related declarations or steps. |
| 12 | <code>pub const SPLIT&#95;DEPTH: i32 = 5; // Smaller subtrees stay on the current worker</code> | Define SPLIT&#95;DEPTH, including its explicit type and initial value. |
| 13 | <code>pub const SPLIT&#95;PLY: usize = 16; // Bound split metadata along checking lines</code> | Define SPLIT&#95;PLY, including its explicit type and initial value. |
| 14 | <code></code> | Blank line separating related declarations or steps. |
| 15 | <code>#&#91;derive(Clone, Copy)&#93;</code> | Compiler attribute controlling the following declaration. |
| 16 | <code>pub struct Candidate {</code> | Declare the struct Candidate used by this module. |
| 17 | <code>    pub mv: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 18 | <code>    pub reduce: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 19 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>pub enum Job {</code> | Declare the enum Job used by this module. |
| 22 | <code>    Search(Arc&lt;Split&gt;),</code> | Continue the enclosing declaration or initialization expression. |
| 23 | <code>    Quit,</code> | Continue the enclosing declaration or initialization expression. |
| 24 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 25 | <code></code> | Blank line separating related declarations or steps. |
| 26 | <code>pub struct Pool {</code> | Declare the struct Pool used by this module. |
| 27 | <code>    idle: Mutex&lt;Vec&lt;Sender&lt;Job&gt;&gt;&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 28 | <code>    available: AtomicUsize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 29 | <code>    waiting: AtomicUsize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 30 | <code>    help: Mutex&lt;HelpRegistry&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 31 | <code>    pub generation: AtomicU64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 32 | <code>    pub splits: AtomicU64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 33 | <code>    pub tasks: AtomicU64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 34 | <code>    pub cutoffs: AtomicU64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 35 | <code>    table&#95;metrics: Mutex&lt;TableMetrics&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 36 | <code>    worker&#95;metrics: Mutex&lt;&#91;SearchMetrics; 64&#93;&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 37 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 38 | <code></code> | Blank line separating related declarations or steps. |
| 39 | <code>#&#91;derive(Default)&#93;</code> | Compiler attribute controlling the following declaration. |
| 40 | <code>struct HelpRegistry {</code> | Declare the struct HelpRegistry used by this module. |
| 41 | <code>    waiting: Vec&lt;Arc&lt;Split&gt;&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 42 | <code>    open: Vec&lt;Weak&lt;Split&gt;&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 43 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 44 | <code></code> | Blank line separating related declarations or steps. |
| 45 | <code>/// Registration precedes execution and lasts through restoration of worker state.</code> | Source explanation: Registration precedes execution and lasts through restoration of worker state. |
| 46 | <code>pub struct HelpTask {</code> | Declare the struct HelpTask used by this module. |
| 47 | <code>    pub split: Arc&lt;Split&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 48 | <code>    pub candidate: Candidate,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 49 | <code>    pub alpha: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 50 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code></code> | Blank line separating related declarations or steps. |
| 52 | <code>impl Drop for HelpTask {</code> | Begin the implementation of methods or a trait for the named type. |
| 53 | <code>    fn drop(&amp;mut self) {</code> | Declare drop(); its parameters and return type define the interface. |
| 54 | <code>        self.split.helper&#95;done();</code> | Evaluate or continue this expression in drop(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 57 | <code></code> | Blank line separating related declarations or steps. |
| 58 | <code>impl Pool {</code> | Begin the implementation of methods or a trait for the named type. |
| 59 | <code>    pub fn new() -&gt; Self {</code> | Declare new(); its parameters and return type define the interface. |
| 60 | <code>        Self {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>            idle: Mutex::new(Vec::new()),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 62 | <code>            available: AtomicUsize::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 63 | <code>            waiting: AtomicUsize::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 64 | <code>            help: Mutex::new(HelpRegistry::default()),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 65 | <code>            generation: AtomicU64::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 66 | <code>            splits: AtomicU64::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 67 | <code>            tasks: AtomicU64::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 68 | <code>            cutoffs: AtomicU64::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 69 | <code>            table&#95;metrics: Mutex::new(TableMetrics::default()),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 70 | <code>            worker&#95;metrics: Mutex::new(&#91;SearchMetrics::default(); 64&#93;),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 71 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 72 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code></code> | Blank line separating related declarations or steps. |
| 74 | <code>    pub fn add&#95;worker&#95;metrics(&amp;self, id: usize, metrics: SearchMetrics) {</code> | Declare add&#95;worker&#95;metrics(); its parameters and return type define the interface. |
| 75 | <code>        self.worker&#95;metrics.lock().unwrap()&#91;id&#93;.add(metrics);</code> | Evaluate or continue this expression in add&#95;worker&#95;metrics(); a final expression without a semicolon supplies its block&#x27;s value. |
| 76 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 77 | <code></code> | Blank line separating related declarations or steps. |
| 78 | <code>    pub fn worker&#95;metrics(&amp;self) -&gt; &#91;SearchMetrics; 64&#93; {</code> | Declare worker&#95;metrics(); its parameters and return type define the interface. |
| 79 | <code>        &#42;self.worker&#95;metrics.lock().unwrap()</code> | Evaluate or continue this expression in worker&#95;metrics(); a final expression without a semicolon supplies its block&#x27;s value. |
| 80 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 81 | <code></code> | Blank line separating related declarations or steps. |
| 82 | <code>    pub fn add&#95;table&#95;metrics(&amp;self, metrics: TableMetrics) {</code> | Declare add&#95;table&#95;metrics(); its parameters and return type define the interface. |
| 83 | <code>        self.table&#95;metrics.lock().unwrap().add(metrics);</code> | Evaluate or continue this expression in add&#95;table&#95;metrics(); a final expression without a semicolon supplies its block&#x27;s value. |
| 84 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 85 | <code></code> | Blank line separating related declarations or steps. |
| 86 | <code>    pub fn table&#95;metrics(&amp;self) -&gt; TableMetrics {</code> | Declare table&#95;metrics(); its parameters and return type define the interface. |
| 87 | <code>        &#42;self.table&#95;metrics.lock().unwrap()</code> | Evaluate or continue this expression in table&#95;metrics(); a final expression without a semicolon supplies its block&#x27;s value. |
| 88 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 89 | <code></code> | Blank line separating related declarations or steps. |
| 90 | <code>    pub fn has&#95;idle(&amp;self) -&gt; bool {</code> | Declare has&#95;idle(); its parameters and return type define the interface. |
| 91 | <code>        self.available.load(Relaxed) != 0</code> | Evaluate or continue this expression in has&#95;idle(); a final expression without a semicolon supplies its block&#x27;s value. |
| 92 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 93 | <code></code> | Blank line separating related declarations or steps. |
| 94 | <code>    pub fn has&#95;waiter(&amp;self, parent: Option&lt;&amp;Arc&lt;Split&gt;&gt;) -&gt; bool {</code> | Declare has&#95;waiter(); its parameters and return type define the interface. |
| 95 | <code>        if self.waiting.load(Relaxed) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 96 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 97 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 98 | <code>        let Some(parent) = parent else { return false };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 99 | <code>        self.help.lock().unwrap().waiting.iter().any(&#124;scope&#124; {</code> | Evaluate or continue this expression in has&#95;waiter(); a final expression without a semicolon supplies its block&#x27;s value. |
| 100 | <code>            !scope.is&#95;canceled() &amp;&amp; (Arc::ptr&#95;eq(parent, scope) &#124;&#124; parent.descends&#95;from(scope))</code> | Evaluate or continue this expression in has&#95;waiter(); a final expression without a semicolon supplies its block&#x27;s value. |
| 101 | <code>        })</code> | Evaluate or continue this expression in has&#95;waiter(); a final expression without a semicolon supplies its block&#x27;s value. |
| 102 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 103 | <code></code> | Blank line separating related declarations or steps. |
| 104 | <code>    pub fn begin&#95;wait(&amp;self, scope: &amp;Arc&lt;Split&gt;) {</code> | Declare begin&#95;wait(); its parameters and return type define the interface. |
| 105 | <code>        let mut help = self.help.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 106 | <code>        help.waiting.push(scope.clone());</code> | Evaluate or continue this expression in begin&#95;wait(); a final expression without a semicolon supplies its block&#x27;s value. |
| 107 | <code>        self.waiting.store(help.waiting.len(), Relaxed);</code> | Evaluate or continue this expression in begin&#95;wait(); a final expression without a semicolon supplies its block&#x27;s value. |
| 108 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 109 | <code></code> | Blank line separating related declarations or steps. |
| 110 | <code>    pub fn end&#95;wait(&amp;self, scope: &amp;Arc&lt;Split&gt;) {</code> | Declare end&#95;wait(); its parameters and return type define the interface. |
| 111 | <code>        let mut help = self.help.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 112 | <code>        help.waiting.retain(&#124;item&#124; !Arc::ptr&#95;eq(item, scope));</code> | Evaluate or continue this expression in end&#95;wait(); a final expression without a semicolon supplies its block&#x27;s value. |
| 113 | <code>        self.waiting.store(help.waiting.len(), Relaxed);</code> | Evaluate or continue this expression in end&#95;wait(); a final expression without a semicolon supplies its block&#x27;s value. |
| 114 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 115 | <code></code> | Blank line separating related declarations or steps. |
| 116 | <code>    pub fn publish(&amp;self, split: &amp;Arc&lt;Split&gt;) {</code> | Declare publish(); its parameters and return type define the interface. |
| 117 | <code>        self.help.lock().unwrap().open.push(Arc::downgrade(split));</code> | Evaluate or continue this expression in publish(); a final expression without a semicolon supplies its block&#x27;s value. |
| 118 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 119 | <code></code> | Blank line separating related declarations or steps. |
| 120 | <code>    pub fn retire(&amp;self, split: &amp;Arc&lt;Split&gt;) {</code> | Declare retire(); its parameters and return type define the interface. |
| 121 | <code>        self.help.lock().unwrap().open.retain(&#124;item&#124; {</code> | Evaluate or continue this expression in retire(); a final expression without a semicolon supplies its block&#x27;s value. |
| 122 | <code>            item.upgrade()</code> | Evaluate or continue this expression in retire(); a final expression without a semicolon supplies its block&#x27;s value. |
| 123 | <code>                .is&#95;some&#95;and(&#124;item&#124; !Arc::ptr&#95;eq(&amp;item, split))</code> | Evaluate or continue this expression in retire(); a final expression without a semicolon supplies its block&#x27;s value. |
| 124 | <code>        });</code> | Evaluate or continue this expression in retire(); a final expression without a semicolon supplies its block&#x27;s value. |
| 125 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 126 | <code></code> | Blank line separating related declarations or steps. |
| 127 | <code>    /// Registry-to-split is the only lock order; callers hold no split lock.</code> | Source explanation: Registry-to-split is the only lock order; callers hold no split lock. |
| 128 | <code>    pub fn try&#95;help(&amp;self, scope: &amp;Arc&lt;Split&gt;) -&gt; Option&lt;HelpTask&gt; {</code> | Declare try&#95;help(); its parameters and return type define the interface. |
| 129 | <code>        if scope.is&#95;canceled() {</code> | Test the condition and execute this branch only when it matches. |
| 130 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 131 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 132 | <code>        let help = self.help.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 133 | <code>        for item in help.open.iter().rev() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 134 | <code>            let Some(split) = item.upgrade() else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 135 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 136 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 137 | <code>            if !split.descends&#95;from(scope) {</code> | Test the condition and execute this branch only when it matches. |
| 138 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 139 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 140 | <code>            let mut state = split.progress.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 141 | <code>            if state.closed &#124;&#124; split.is&#95;canceled() &#124;&#124; state.next &gt;= split.moves.len() {</code> | Test the condition and execute this branch only when it matches. |
| 142 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 143 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 144 | <code>            let candidate = split.moves&#91;state.next&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 145 | <code>            state.next += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 146 | <code>            state.helpers += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 147 | <code>            let alpha = state.alpha;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 148 | <code>            drop(state);</code> | Evaluate or continue this expression in try&#95;help(); a final expression without a semicolon supplies its block&#x27;s value. |
| 149 | <code>            return Some(HelpTask {</code> | Leave the current function with the displayed result. |
| 150 | <code>                split,</code> | Evaluate or continue this expression in try&#95;help(); a final expression without a semicolon supplies its block&#x27;s value. |
| 151 | <code>                candidate,</code> | Evaluate or continue this expression in try&#95;help(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>                alpha,</code> | Evaluate or continue this expression in try&#95;help(); a final expression without a semicolon supplies its block&#x27;s value. |
| 153 | <code>            });</code> | Evaluate or continue this expression in try&#95;help(); a final expression without a semicolon supplies its block&#x27;s value. |
| 154 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>        None</code> | Evaluate or continue this expression in try&#95;help(); a final expression without a semicolon supplies its block&#x27;s value. |
| 156 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 157 | <code></code> | Blank line separating related declarations or steps. |
| 158 | <code>    pub fn release(&amp;self, sender: Sender&lt;Job&gt;) {</code> | Declare release(); its parameters and return type define the interface. |
| 159 | <code>        let mut idle = self.idle.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 160 | <code>        idle.push(sender);</code> | Evaluate or continue this expression in release(); a final expression without a semicolon supplies its block&#x27;s value. |
| 161 | <code>        self.available.store(idle.len(), Relaxed);</code> | Evaluate or continue this expression in release(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 163 | <code></code> | Blank line separating related declarations or steps. |
| 164 | <code>    pub fn reserve(&amp;self, limit: usize) -&gt; Vec&lt;Sender&lt;Job&gt;&gt; {</code> | Declare reserve(); its parameters and return type define the interface. |
| 165 | <code>        let mut idle = self.idle.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 166 | <code>        let count = limit.min(idle.len());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 167 | <code>        let start = idle.len() - count;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 168 | <code>        let reserved = idle.split&#95;off(start);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 169 | <code>        self.available.store(idle.len(), Relaxed);</code> | Evaluate or continue this expression in reserve(); a final expression without a semicolon supplies its block&#x27;s value. |
| 170 | <code>        reserved</code> | Evaluate or continue this expression in reserve(); a final expression without a semicolon supplies its block&#x27;s value. |
| 171 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 172 | <code></code> | Blank line separating related declarations or steps. |
| 173 | <code>    pub fn clear(&amp;self) {</code> | Declare clear(); its parameters and return type define the interface. |
| 174 | <code>        self.idle.lock().unwrap().clear();</code> | Evaluate or continue this expression in clear(); a final expression without a semicolon supplies its block&#x27;s value. |
| 175 | <code>        self.available.store(0, Relaxed);</code> | Evaluate or continue this expression in clear(); a final expression without a semicolon supplies its block&#x27;s value. |
| 176 | <code>        &#42;self.help.lock().unwrap() = HelpRegistry::default();</code> | Evaluate or continue this expression in clear(); a final expression without a semicolon supplies its block&#x27;s value. |
| 177 | <code>        self.waiting.store(0, Relaxed);</code> | Evaluate or continue this expression in clear(); a final expression without a semicolon supplies its block&#x27;s value. |
| 178 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 179 | <code></code> | Blank line separating related declarations or steps. |
| 180 | <code>    pub fn begin(&amp;self) {</code> | Declare begin(); its parameters and return type define the interface. |
| 181 | <code>        self.generation.fetch&#95;add(1, Relaxed);</code> | Evaluate or continue this expression in begin(); a final expression without a semicolon supplies its block&#x27;s value. |
| 182 | <code>        self.splits.store(0, Relaxed);</code> | Evaluate or continue this expression in begin(); a final expression without a semicolon supplies its block&#x27;s value. |
| 183 | <code>        self.tasks.store(0, Relaxed);</code> | Evaluate or continue this expression in begin(); a final expression without a semicolon supplies its block&#x27;s value. |
| 184 | <code>        self.cutoffs.store(0, Relaxed);</code> | Evaluate or continue this expression in begin(); a final expression without a semicolon supplies its block&#x27;s value. |
| 185 | <code>        &#42;self.table&#95;metrics.lock().unwrap() = TableMetrics::default();</code> | Evaluate or continue this expression in begin(); a final expression without a semicolon supplies its block&#x27;s value. |
| 186 | <code>        &#42;self.worker&#95;metrics.lock().unwrap() = &#91;SearchMetrics::default(); 64&#93;;</code> | Evaluate or continue this expression in begin(); a final expression without a semicolon supplies its block&#x27;s value. |
| 187 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 188 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 189 | <code></code> | Blank line separating related declarations or steps. |
| 190 | <code>pub struct Progress {</code> | Declare the struct Progress used by this module. |
| 191 | <code>    pub next: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 192 | <code>    pub helpers: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 193 | <code>    pub closed: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 194 | <code>    pub alpha: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 195 | <code>    pub score: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 196 | <code>    pub best: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 197 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 198 | <code></code> | Blank line separating related declarations or steps. |
| 199 | <code>pub struct Split {</code> | Declare the struct Split used by this module. |
| 200 | <code>    pub position: Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 201 | <code>    pub history: Vec&lt;u64&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 202 | <code>    pub moves: Vec&lt;Candidate&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 203 | <code>    pub depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 204 | <code>    pub ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 205 | <code>    pub beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 206 | <code>    pub parent: Option&lt;Arc&lt;Split&gt;&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 207 | <code>    pub canceled: AtomicBool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 208 | <code>    pub research: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 209 | <code>    pub progress: Mutex&lt;Progress&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 210 | <code>    pub changed: Condvar,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 211 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 212 | <code></code> | Blank line separating related declarations or steps. |
| 213 | <code>impl Split {</code> | Begin the implementation of methods or a trait for the named type. |
| 214 | <code>    /// Strict ancestry prevents helping an ancestor, sibling, or unrelated root.</code> | Source explanation: Strict ancestry prevents helping an ancestor, sibling, or unrelated root. |
| 215 | <code>    pub fn descends&#95;from(&amp;self, ancestor: &amp;Arc&lt;Split&gt;) -&gt; bool {</code> | Declare descends&#95;from(); its parameters and return type define the interface. |
| 216 | <code>        let mut parent = self.parent.as&#95;ref();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 217 | <code>        while let Some(split) = parent {</code> | Repeat this block while its condition remains true. |
| 218 | <code>            if Arc::ptr&#95;eq(split, ancestor) {</code> | Test the condition and execute this branch only when it matches. |
| 219 | <code>                return true;</code> | Leave the current function with the displayed result. |
| 220 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 221 | <code>            parent = split.parent.as&#95;ref();</code> | Update the named state using the expression and assignment operator shown. |
| 222 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code>        false</code> | Evaluate or continue this expression in descends&#95;from(); a final expression without a semicolon supplies its block&#x27;s value. |
| 224 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 225 | <code>    /// A cancellation affects this split and descendants, not unrelated nodes.</code> | Source explanation: A cancellation affects this split and descendants, not unrelated nodes. |
| 226 | <code>    pub fn is&#95;canceled(&amp;self) -&gt; bool {</code> | Declare is&#95;canceled(); its parameters and return type define the interface. |
| 227 | <code>        let mut scope = Some(self);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 228 | <code>        while let Some(split) = scope {</code> | Repeat this block while its condition remains true. |
| 229 | <code>            if split.canceled.load(Relaxed) {</code> | Test the condition and execute this branch only when it matches. |
| 230 | <code>                return true;</code> | Leave the current function with the displayed result. |
| 231 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 232 | <code>            scope = split.parent.as&#95;deref();</code> | Update the named state using the expression and assignment operator shown. |
| 233 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 234 | <code>        false</code> | Evaluate or continue this expression in is&#95;canceled(); a final expression without a semicolon supplies its block&#x27;s value. |
| 235 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 236 | <code></code> | Blank line separating related declarations or steps. |
| 237 | <code>    /// Claim a move and the alpha snapshot used for both probe and re-search.</code> | Source explanation: Claim a move and the alpha snapshot used for both probe and re-search. |
| 238 | <code>    pub fn claim(&amp;self) -&gt; Option&lt;(Candidate, i32)&gt; {</code> | Declare claim(); its parameters and return type define the interface. |
| 239 | <code>        let mut state = self.progress.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 240 | <code>        if state.closed &#124;&#124; self.is&#95;canceled() &#124;&#124; state.next &gt;= self.moves.len() {</code> | Test the condition and execute this branch only when it matches. |
| 241 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 242 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 243 | <code>        let candidate = self.moves&#91;state.next&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 244 | <code>        state.next += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 245 | <code>        Some((candidate, state.alpha))</code> | Evaluate or continue this expression in claim(); a final expression without a semicolon supplies its block&#x27;s value. |
| 246 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 247 | <code></code> | Blank line separating related declarations or steps. |
| 248 | <code>    /// Publish a verified full-depth result in the parent&#x27;s score convention.</code> | Source explanation: Publish a verified full-depth result in the parent&#x27;s score convention. |
| 249 | <code>    pub fn publish(&amp;self, mv: Move, score: i32) -&gt; Option&lt;bool&gt; {</code> | Declare publish(); its parameters and return type define the interface. |
| 250 | <code>        let mut state = self.progress.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 251 | <code>        if self.is&#95;canceled() {</code> | Test the condition and execute this branch only when it matches. |
| 252 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 253 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 254 | <code>        if score &gt; state.score {</code> | Test the condition and execute this branch only when it matches. |
| 255 | <code>            state.score = score;</code> | Update the named state using the expression and assignment operator shown. |
| 256 | <code>            state.best = mv;</code> | Update the named state using the expression and assignment operator shown. |
| 257 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 258 | <code>        state.alpha = state.alpha.max(score);</code> | Update the named state using the expression and assignment operator shown. |
| 259 | <code>        if score &gt;= self.beta {</code> | Test the condition and execute this branch only when it matches. |
| 260 | <code>            self.canceled.store(true, Relaxed);</code> | Evaluate or continue this expression in publish(); a final expression without a semicolon supplies its block&#x27;s value. |
| 261 | <code>            self.changed.notify&#95;all();</code> | Evaluate or continue this expression in publish(); a final expression without a semicolon supplies its block&#x27;s value. |
| 262 | <code>            return Some(true);</code> | Leave the current function with the displayed result. |
| 263 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 264 | <code>        Some(false)</code> | Evaluate or continue this expression in publish(); a final expression without a semicolon supplies its block&#x27;s value. |
| 265 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 266 | <code></code> | Blank line separating related declarations or steps. |
| 267 | <code>    /// Called after a helper flushes its nodes and releases all task state.</code> | Source explanation: Called after a helper flushes its nodes and releases all task state. |
| 268 | <code>    pub fn helper&#95;done(&amp;self) {</code> | Declare helper&#95;done(); its parameters and return type define the interface. |
| 269 | <code>        let mut state = self.progress.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 270 | <code>        state.helpers -= 1;</code> | Update the named state using the expression and assignment operator shown. |
| 271 | <code>        self.changed.notify&#95;all();</code> | Evaluate or continue this expression in helper&#95;done(); a final expression without a semicolon supplies its block&#x27;s value. |
| 272 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 273 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### search.rs

Native move selection, iterative deepening, alpha-beta, quiescence, and ordering.

[Open source](src/search.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Native alpha-beta search with Young Brothers Wait sibling splitting.</code> | Source explanation: Native alpha-beta search with Young Brothers Wait sibling splitting. |
| 2 | <code>//! One iterative-deepening owner coordinates persistent workers and a shared table.</code> | Source explanation: One iterative-deepening owner coordinates persistent workers and a shared table. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use std::io::Write;</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::Relaxed};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use std::sync::{Arc, Condvar, Mutex, mpsc};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code>use std::thread::JoinHandle;</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code>use std::time::{Duration, Instant};</code> | Bring the named items into scope; no chess search runs here. |
| 9 | <code></code> | Blank line separating related declarations or steps. |
| 10 | <code>use crate::attacks::{QUEEN&#95;LINES, in&#95;check};</code> | Bring the named items into scope; no chess search runs here. |
| 11 | <code>use crate::board::{BISHOP, KING, KNIGHT, ROOK};</code> | Bring the named items into scope; no chess search runs here. |
| 12 | <code>use crate::board::{</code> | Bring the named items into scope; no chess search runs here. |
| 13 | <code>    EMPTY, MATE&#95;SCORE, Move, NO&#95;MOVE, PAWN, Position, QUEEN, move&#95;promotion, move&#95;start,</code> | Continue the enclosing declaration or initialization expression. |
| 14 | <code>    move&#95;target, move&#95;text,</code> | Continue the enclosing declaration or initialization expression. |
| 15 | <code>};</code> | Close the preceding block, expression, call, or collection. |
| 16 | <code>use crate::evaluation::{PIECE&#95;VALUES, evaluate};</code> | Bring the named items into scope; no chess search runs here. |
| 17 | <code>use crate::exchange::exchange&#95;score;</code> | Bring the named items into scope; no chess search runs here. |
| 18 | <code>use crate::make&#95;move::{make&#95;move, make&#95;null&#95;move};</code> | Bring the named items into scope; no chess search runs here. |
| 19 | <code>use crate::move&#95;generation::{MoveList, StackList, generate&#95;captures, generate&#95;moves};</code> | Bring the named items into scope; no chess search runs here. |
| 20 | <code>use crate::parallel&#95;search::{Candidate, Job, Pool, Progress, SPLIT&#95;DEPTH, SPLIT&#95;PLY, Split};</code> | Bring the named items into scope; no chess search runs here. |
| 21 | <code>use crate::search&#95;control::SearchControl;</code> | Bring the named items into scope; no chess search runs here. |
| 22 | <code>use crate::search&#95;metrics::{Research, SearchMetrics};</code> | Bring the named items into scope; no chess search runs here. |
| 23 | <code>use crate::table&#95;metrics::TableMetrics;</code> | Bring the named items into scope; no chess search runs here. |
| 24 | <code>use crate::transposition&#95;table::{EXACT, LOWER, Table, UPPER};</code> | Bring the named items into scope; no chess search runs here. |
| 25 | <code></code> | Blank line separating related declarations or steps. |
| 26 | <code>const FUTILITY&#95;MARGIN: i32 = 200; // A quiet move seldom gains more than this</code> | Define FUTILITY&#95;MARGIN, including its explicit type and initial value. |
| 27 | <code>const DELTA&#95;MARGIN: i32 = 200; // A capture seldom gains more than its victim plus this</code> | Define DELTA&#95;MARGIN, including its explicit type and initial value. |
| 28 | <code>const ASPIRATION&#95;WINDOW: i32 = 50; // The first window around the last score</code> | Define ASPIRATION&#95;WINDOW, including its explicit type and initial value. |
| 29 | <code>const CAPTURE: i32 = 10&#95;000; // Every capture and queen promotion scores above this</code> | Define CAPTURE, including its explicit type and initial value. |
| 30 | <code>const HISTORY&#95;LIMIT: i32 = 3999; // Quiet history stays below both killer moves</code> | Define HISTORY&#95;LIMIT, including its explicit type and initial value. |
| 31 | <code>const MAX&#95;PLY: usize = 128; // Killer moves are kept for this many plies</code> | Define MAX&#95;PLY, including its explicit type and initial value. |
| 32 | <code>const MAX&#95;DEPTH: i32 = 64; // Iterative deepening stops here</code> | Define MAX&#95;DEPTH, including its explicit type and initial value. |
| 33 | <code>const EARLY&#95;PICKS: usize = 3; // Moves chosen one at a time before the rest are sorted</code> | Define EARLY&#95;PICKS, including its explicit type and initial value. |
| 34 | <code>const MAX&#95;HELP&#95;DEPTH: usize = 2; // Bound suspended search frames on each worker stack</code> | Define MAX&#95;HELP&#95;DEPTH, including its explicit type and initial value. |
| 35 | <code>pub const DEFAULT&#95;THREADS: usize = 4; // Search threads, including the main thread</code> | Define DEFAULT&#95;THREADS, including its explicit type and initial value. |
| 36 | <code>pub const MAX&#95;THREADS: usize = 64; // The largest Threads value accepted</code> | Define MAX&#95;THREADS, including its explicit type and initial value. |
| 37 | <code></code> | Blank line separating related declarations or steps. |
| 38 | <code>/// The sort entries of one position&#x27;s moves.</code> | Source explanation: The sort entries of one position&#x27;s moves. |
| 39 | <code>type EntryList = StackList&lt;u64, 256&gt;;</code> | Declare the type EntryList used by this module. |
| 40 | <code></code> | Blank line separating related declarations or steps. |
| 41 | <code>/// The search stops by returning this error up through every call.</code> | Source explanation: The search stops by returning this error up through every call. |
| 42 | <code>struct SearchTimeout;</code> | Declare the struct SearchTimeout used by this module. |
| 43 | <code></code> | Blank line separating related declarations or steps. |
| 44 | <code>/// What one search thread keeps from one position to the next.</code> | Source explanation: What one search thread keeps from one position to the next. |
| 45 | <code>struct Worker {</code> | Declare the struct Worker used by this module. |
| 46 | <code>    nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 47 | <code>    metrics: TableMetrics,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 48 | <code>    work: SearchMetrics,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 49 | <code>    research&#95;level: u32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 50 | <code>    help&#95;depth: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 51 | <code>    pending&#95;nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 52 | <code>    generation: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 53 | <code>    pool: Arc&lt;Pool&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 54 | <code>    branch: Option&lt;Arc&lt;Split&gt;&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 55 | <code>    control: Option&lt;SearchControl&gt;, // Only the main thread reads commands and the clock</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 56 | <code>    stop: Arc&lt;AtomicBool&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 57 | <code>    total&#95;nodes: Arc&lt;AtomicU64&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 58 | <code>    best&#95;move: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 59 | <code>    history: Vec&lt;u64&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 60 | <code>    killers: &#91;&#91;Move; 2&#93;; MAX&#95;PLY&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 61 | <code>    quiet&#95;history: &#91;&#91;&#91;i32; 64&#93;; 64&#93;; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 62 | <code>    table: Arc&lt;Table&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 63 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 64 | <code></code> | Blank line separating related declarations or steps. |
| 65 | <code>/// The main thread&#x27;s worker, its helpers, and the table they share.</code> | Source explanation: The main thread&#x27;s worker, its helpers, and the table they share. |
| 66 | <code>pub struct Search {</code> | Declare the struct Search used by this module. |
| 67 | <code>    main: Worker,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 68 | <code>    helpers: Vec&lt;(mpsc::Sender&lt;Job&gt;, JoinHandle&lt;()&gt;)&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 69 | <code>    pool: Arc&lt;Pool&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 70 | <code>    stop: Arc&lt;AtomicBool&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 71 | <code>    total&#95;nodes: Arc&lt;AtomicU64&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 72 | <code>    table: Arc&lt;Table&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 73 | <code>    pub diagnostics: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 74 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 75 | <code></code> | Blank line separating related declarations or steps. |
| 76 | <code>/// Return a list entry that sorts by priority and then by move, as Remus5 sorts the</code> | Source explanation: Return a list entry that sorts by priority and then by move, as Remus5 sorts the |
| 77 | <code>/// pairs (priority, move).</code> | Source explanation: pairs (priority, move). |
| 78 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 79 | <code>fn sort&#95;entry(priority: i32, mv: Move) -&gt; u64 {</code> | Declare sort&#95;entry(); its parameters and return type define the interface. |
| 80 | <code>    (u64::from(priority as u32) &lt;&lt; 16) &#124; u64::from(mv)</code> | Evaluate or continue this expression in sort&#95;entry(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 82 | <code></code> | Blank line separating related declarations or steps. |
| 83 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 84 | <code>fn entry&#95;move(entry: u64) -&gt; Move {</code> | Declare entry&#95;move(); its parameters and return type define the interface. |
| 85 | <code>    entry as Move</code> | Evaluate or continue this expression in entry&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 86 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 87 | <code></code> | Blank line separating related declarations or steps. |
| 88 | <code>/// Return the captured kind, including a pawn taken en passant, or EMPTY.</code> | Source explanation: Return the captured kind, including a pawn taken en passant, or EMPTY. |
| 89 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 90 | <code>fn captured&#95;kind(position: &amp;Position, mv: Move) -&gt; u8 {</code> | Declare captured&#95;kind(); its parameters and return type define the interface. |
| 91 | <code>    if position.board&#91;move&#95;start(mv)&#93; &amp; 7 == PAWN &amp;&amp; move&#95;target(mv) as i8 == position.en&#95;passant {</code> | Test the condition and execute this branch only when it matches. |
| 92 | <code>        return PAWN;</code> | Leave the current function with the displayed result. |
| 93 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code>    position.board&#91;move&#95;target(mv)&#93; &amp; 7</code> | Evaluate or continue this expression in captured&#95;kind(); a final expression without a semicolon supplies its block&#x27;s value. |
| 95 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 96 | <code></code> | Blank line separating related declarations or steps. |
| 97 | <code>/// The priority of a move before the table, killers, and history have their say.</code> | Source explanation: The priority of a move before the table, killers, and history have their say. |
| 98 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 99 | <code>fn capture&#95;priority(position: &amp;Position, mv: Move) -&gt; i32 {</code> | Declare capture&#95;priority(); its parameters and return type define the interface. |
| 100 | <code>    let mut priority = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 101 | <code>    let victim = captured&#95;kind(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>    if victim != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 103 | <code>        let attacker = PIECE&#95;VALUES&#91;(position.board&#91;move&#95;start(mv)&#93; &amp; 7) as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 104 | <code>        priority = CAPTURE + 10 &#42; PIECE&#95;VALUES&#91;victim as usize&#93; - attacker;</code> | Update the named state using the expression and assignment operator shown. |
| 105 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 106 | <code>    if move&#95;promotion(mv) == QUEEN {</code> | Test the condition and execute this branch only when it matches. |
| 107 | <code>        priority += CAPTURE + 9000;</code> | Update the named state using the expression and assignment operator shown. |
| 108 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 109 | <code>    priority</code> | Evaluate or continue this expression in capture&#95;priority(); a final expression without a semicolon supplies its block&#x27;s value. |
| 110 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 111 | <code></code> | Blank line separating related declarations or steps. |
| 112 | <code>/// Fill the entries with the moves of the position and their priorities. The entries</code> | Source explanation: Fill the entries with the moves of the position and their priorities. The entries |
| 113 | <code>/// stand in the order of generation: place&#95;next() or a sort puts the likely best first.</code> | Source explanation: stand in the order of generation: place&#95;next() or a sort puts the likely best first. |
| 114 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 115 | <code>fn ordered&#95;moves(</code> | Declare ordered&#95;moves(); its parameters and return type define the interface. |
| 116 | <code>    position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 117 | <code>    first: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 118 | <code>    killers: &#91;Move; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 119 | <code>    quiet&#95;history: &amp;&#91;&#91;i32; 64&#93;; 64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 120 | <code>    entries: &amp;mut EntryList,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 121 | <code>) {</code> | Evaluate or continue this expression in ordered&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 122 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 123 | <code>    generate&#95;moves(position, &amp;mut list);</code> | Evaluate or continue this expression in ordered&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 124 | <code>    for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 125 | <code>        let mut priority = capture&#95;priority(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 126 | <code>        if captured&#95;kind(position, mv) == EMPTY &amp;&amp; move&#95;promotion(mv) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 127 | <code>            priority = quiet&#95;history&#91;move&#95;start(mv)&#93;&#91;move&#95;target(mv)&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 128 | <code>            if mv == killers&#91;0&#93; {</code> | Test the condition and execute this branch only when it matches. |
| 129 | <code>                priority = 5000;</code> | Update the named state using the expression and assignment operator shown. |
| 130 | <code>            } else if mv == killers&#91;1&#93; {</code> | Test the condition and execute this branch only when it matches. |
| 131 | <code>                priority = 4000;</code> | Update the named state using the expression and assignment operator shown. |
| 132 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 133 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 134 | <code>        if mv == first {</code> | Test the condition and execute this branch only when it matches. |
| 135 | <code>            priority = 1&#95;000&#95;000;</code> | Update the named state using the expression and assignment operator shown. |
| 136 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 137 | <code>        entries.add(sort&#95;entry(priority, mv));</code> | Evaluate or continue this expression in ordered&#95;moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 138 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 139 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 140 | <code></code> | Blank line separating related declarations or steps. |
| 141 | <code>/// Put the entry that belongs at this index in its place, so the moves come out in</code> | Source explanation: Put the entry that belongs at this index in its place, so the moves come out in |
| 142 | <code>/// the order a full sort gives. Each of the first few indexes takes the largest entry</code> | Source explanation: the order a full sort gives. Each of the first few indexes takes the largest entry |
| 143 | <code>/// left, which costs one pass over the list. The index after those sorts the rest.</code> | Source explanation: left, which costs one pass over the list. The index after those sorts the rest. |
| 144 | <code>/// Most cutoffs come from the first few moves, and then the sort never runs.</code> | Source explanation: Most cutoffs come from the first few moves, and then the sort never runs. |
| 145 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 146 | <code>fn place&#95;next(entries: &amp;mut &#91;u64&#93;, index: usize) {</code> | Declare place&#95;next(); its parameters and return type define the interface. |
| 147 | <code>    if index &lt; EARLY&#95;PICKS {</code> | Test the condition and execute this branch only when it matches. |
| 148 | <code>        let mut best = index;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 149 | <code>        for other in index + 1..entries.len() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 150 | <code>            if entries&#91;other&#93; &gt; entries&#91;best&#93; {</code> | Test the condition and execute this branch only when it matches. |
| 151 | <code>                best = other;</code> | Update the named state using the expression and assignment operator shown. |
| 152 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 153 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 154 | <code>        entries.swap(index, best);</code> | Evaluate or continue this expression in place&#95;next(); a final expression without a semicolon supplies its block&#x27;s value. |
| 155 | <code>    } else if index == EARLY&#95;PICKS {</code> | Test the condition and execute this branch only when it matches. |
| 156 | <code>        entries&#91;index..&#93;.sort&#95;unstable&#95;by(&#124;a, b&#124; b.cmp(a));</code> | Evaluate or continue this expression in place&#95;next(); a final expression without a semicolon supplies its block&#x27;s value. |
| 157 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 158 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 159 | <code></code> | Blank line separating related declarations or steps. |
| 160 | <code>/// Return true if the side to move has a legal move. The caller says whether that</code> | Source explanation: Return true if the side to move has a legal move. The caller says whether that |
| 161 | <code>/// side is in check.</code> | Source explanation: side is in check. |
| 162 | <code>fn has&#95;legal&#95;move(position: &amp;Position, check: bool) -&gt; bool {</code> | Declare has&#95;legal&#95;move(); its parameters and return type define the interface. |
| 163 | <code>    if !check {</code> | Test the condition and execute this branch only when it matches. |
| 164 | <code>        // Out of check, a move can leave its own king attacked only by moving the</code> | Source explanation: Out of check, a move can leave its own king attacked only by moving the |
| 165 | <code>        // king, by capturing en passant, or by leaving a line through the king. A</code> | Source explanation: king, by capturing en passant, or by leaving a line through the king. A |
| 166 | <code>        // pawn off those lines with an empty square ahead has a legal step, and three</code> | Source explanation: pawn off those lines with an empty square ahead has a legal step, and three |
| 167 | <code>        // bitboard operations find one without generating a move.</code> | Source explanation: bitboard operations find one without generating a move. |
| 168 | <code>        let own = position.by&#95;side&#91;usize::from(!position.white)&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 169 | <code>        let king = (own &amp; position.by&#95;kind&#91;KING as usize&#93;).trailing&#95;zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 170 | <code>        let free = own &amp; position.by&#95;kind&#91;PAWN as usize&#93; &amp; !QUEEN&#95;LINES&#91;king &amp; 63&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 171 | <code>        let ahead = if position.white { free &gt;&gt; 8 } else { free &lt;&lt; 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 172 | <code>        if ahead &amp; !position.occupied() != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 173 | <code>            return true;</code> | Leave the current function with the displayed result. |
| 174 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 175 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 176 | <code>    // Stop at the first move that leaves the moving side&#x27;s king safe</code> | Source explanation: Stop at the first move that leaves the moving side&#x27;s king safe |
| 177 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 178 | <code>    generate&#95;moves(position, &amp;mut list);</code> | Evaluate or continue this expression in has&#95;legal&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>    for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 180 | <code>        if !in&#95;check(&amp;make&#95;move(position, mv), position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 181 | <code>            return true;</code> | Leave the current function with the displayed result. |
| 182 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 183 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 184 | <code>    false</code> | Evaluate or continue this expression in has&#95;legal&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 185 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 186 | <code></code> | Blank line separating related declarations or steps. |
| 187 | <code>/// Return true if the side has a piece other than its king and pawns.</code> | Source explanation: Return true if the side has a piece other than its king and pawns. |
| 188 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 189 | <code>fn has&#95;pieces(position: &amp;Position) -&gt; bool {</code> | Declare has&#95;pieces(); its parameters and return type define the interface. |
| 190 | <code>    let kinds = &amp;position.by&#95;kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 191 | <code>    let pieces = kinds&#91;KNIGHT as usize&#93;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 192 | <code>        &#124; kinds&#91;BISHOP as usize&#93;</code> | Evaluate or continue this expression in has&#95;pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 193 | <code>        &#124; kinds&#91;ROOK as usize&#93;</code> | Evaluate or continue this expression in has&#95;pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 194 | <code>        &#124; kinds&#91;QUEEN as usize&#93;;</code> | Evaluate or continue this expression in has&#95;pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 195 | <code>    pieces &amp; position.by&#95;side&#91;usize::from(!position.white)&#93; != 0</code> | Evaluate or continue this expression in has&#95;pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 196 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 197 | <code></code> | Blank line separating related declarations or steps. |
| 198 | <code>impl Search {</code> | Begin the implementation of methods or a trait for the named type. |
| 199 | <code>    pub fn new() -&gt; Search {</code> | Declare new(); its parameters and return type define the interface. |
| 200 | <code>        let stop = Arc::new(AtomicBool::new(false));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 201 | <code>        let total&#95;nodes = Arc::new(AtomicU64::new(0));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 202 | <code>        let table = Arc::new(Table::new());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 203 | <code>        let pool = Arc::new(Pool::new());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 204 | <code>        let main = Worker::new(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 205 | <code>            Some(SearchControl::new()),</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 206 | <code>            stop.clone(),</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 207 | <code>            total&#95;nodes.clone(),</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 208 | <code>            table.clone(),</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 209 | <code>            pool.clone(),</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 210 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 211 | <code>        let mut search = Search {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 212 | <code>            main,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 213 | <code>            helpers: Vec::new(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 214 | <code>            pool,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 215 | <code>            stop,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 216 | <code>            total&#95;nodes,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 217 | <code>            table,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 218 | <code>            diagnostics: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 219 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 220 | <code>        search.set&#95;threads(DEFAULT&#95;THREADS);</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 221 | <code>        search</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 222 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code></code> | Blank line separating related declarations or steps. |
| 224 | <code>    pub fn control(&amp;mut self) -&gt; &amp;mut SearchControl {</code> | Declare control(); its parameters and return type define the interface. |
| 225 | <code>        self.main.main&#95;control()</code> | Evaluate or continue this expression in control(); a final expression without a semicolon supplies its block&#x27;s value. |
| 226 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 227 | <code></code> | Blank line separating related declarations or steps. |
| 228 | <code>    /// Resize only between searches, after every split has drained its helpers.</code> | Source explanation: Resize only between searches, after every split has drained its helpers. |
| 229 | <code>    pub fn set&#95;threads(&amp;mut self, threads: usize) {</code> | Declare set&#95;threads(); its parameters and return type define the interface. |
| 230 | <code>        self.shutdown&#95;helpers();</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 231 | <code>        for id in 1..threads.clamp(1, MAX&#95;THREADS) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 232 | <code>            let (sender, receiver) = mpsc::channel();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 233 | <code>            let return&#95;sender = sender.clone();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 234 | <code>            let mut worker = Worker::new(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 235 | <code>                None,</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 236 | <code>                self.stop.clone(),</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 237 | <code>                self.total&#95;nodes.clone(),</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 238 | <code>                self.table.clone(),</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 239 | <code>                self.pool.clone(),</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 240 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 241 | <code>            let started = std::thread::Builder::new()</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 242 | <code>                .name(format!(&quot;remus7-{id}&quot;))</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 243 | <code>                .stack&#95;size(crate::STACK&#95;BYTES)</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 244 | <code>                .spawn(move &#124;&#124; {</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 245 | <code>                    while let Ok(job) = receiver.recv() {</code> | Repeat this block while its condition remains true. |
| 246 | <code>                        match job {</code> | Select an arm by matching the expression against the listed patterns. |
| 247 | <code>                            Job::Search(split) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 248 | <code>                                let generation = worker.pool.generation.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 249 | <code>                                if worker.generation != generation {</code> | Test the condition and execute this branch only when it matches. |
| 250 | <code>                                    worker.begin&#95;search();</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 251 | <code>                                    worker.generation = generation;</code> | Update the named state using the expression and assignment operator shown. |
| 252 | <code>                                }</code> | Close the preceding block, expression, call, or collection. |
| 253 | <code>                                let active = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 254 | <code>                                let nodes = worker.nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 255 | <code>                                worker.work = SearchMetrics::default();</code> | Update the named state using the expression and assignment operator shown. |
| 256 | <code>                                worker.research&#95;level = u32::from(split.research);</code> | Update the named state using the expression and assignment operator shown. |
| 257 | <code>                                worker.metrics = TableMetrics::default();</code> | Update the named state using the expression and assignment operator shown. |
| 258 | <code>                                worker.history.clone&#95;from(&amp;split.history);</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 259 | <code>                                worker.branch = Some(split.clone());</code> | Update the named state using the expression and assignment operator shown. |
| 260 | <code>                                worker.search&#95;split(&amp;split);</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 261 | <code>                                worker.branch = None;</code> | Update the named state using the expression and assignment operator shown. |
| 262 | <code>                                worker.history.clear();</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 263 | <code>                                worker.flush&#95;nodes();</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 264 | <code>                                worker.work.nodes = worker.nodes - nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 265 | <code>                                worker.work.jobs = 1;</code> | Update the named state using the expression and assignment operator shown. |
| 266 | <code>                                worker.work.active&#95;ns = (active.elapsed().as&#95;nanos() as u64)</code> | Update the named state using the expression and assignment operator shown. |
| 267 | <code>                                    .saturating&#95;sub(worker.work.wait&#95;ns);</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 268 | <code>                                if split.research {</code> | Test the condition and execute this branch only when it matches. |
| 269 | <code>                                    worker.work.research&#95;nodes = worker.work.nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 270 | <code>                                }</code> | Close the preceding block, expression, call, or collection. |
| 271 | <code>                                worker.pool.add&#95;worker&#95;metrics(id, worker.work);</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 272 | <code>                                worker.pool.add&#95;table&#95;metrics(worker.metrics);</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 273 | <code>                                worker.pool.release(return&#95;sender.clone());</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 274 | <code>                                split.helper&#95;done();</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 275 | <code>                            }</code> | Close the preceding block, expression, call, or collection. |
| 276 | <code>                            Job::Quit =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 277 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 278 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 279 | <code>                });</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 280 | <code>            match started {</code> | Select an arm by matching the expression against the listed patterns. |
| 281 | <code>                Ok(handle) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 282 | <code>                    self.pool.release(sender.clone());</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 283 | <code>                    self.helpers.push((sender, handle));</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 284 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 285 | <code>                Err(&#95;) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 286 | <code>                    println!(</code> | Format and send text to the output destination shown. |
| 287 | <code>                        &quot;info string worker startup limited Threads to {}&quot;,</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 288 | <code>                        self.helpers.len() + 1</code> | Evaluate or continue this expression in set&#95;threads(); a final expression without a semicolon supplies its block&#x27;s value. |
| 289 | <code>                    );</code> | Close the preceding block, expression, call, or collection. |
| 290 | <code>                    break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 291 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 292 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 293 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 294 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 295 | <code></code> | Blank line separating related declarations or steps. |
| 296 | <code>    /// Drain and join the pool before releasing any reference to the shared table.</code> | Source explanation: Drain and join the pool before releasing any reference to the shared table. |
| 297 | <code>    pub fn set&#95;hash(&amp;mut self, mib: usize) {</code> | Declare set&#95;hash(); its parameters and return type define the interface. |
| 298 | <code>        if self.table.size&#95;mib() != mib {</code> | Test the condition and execute this branch only when it matches. |
| 299 | <code>            let threads = self.helpers.len() + 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 300 | <code>            self.shutdown&#95;helpers();</code> | Evaluate or continue this expression in set&#95;hash(); a final expression without a semicolon supplies its block&#x27;s value. |
| 301 | <code>            self.main.table = Arc::new(Table::disabled());</code> | Update the named state using the expression and assignment operator shown. |
| 302 | <code>            self.table = self.main.table.clone();</code> | Update the named state using the expression and assignment operator shown. |
| 303 | <code>            self.table = Arc::new(Table::with&#95;size(mib));</code> | Update the named state using the expression and assignment operator shown. |
| 304 | <code>            self.main.table = self.table.clone();</code> | Update the named state using the expression and assignment operator shown. |
| 305 | <code>            self.set&#95;threads(threads);</code> | Evaluate or continue this expression in set&#95;hash(); a final expression without a semicolon supplies its block&#x27;s value. |
| 306 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 307 | <code>        crate::search&#95;control::search&#95;log(&amp;format!(</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 308 | <code>            &quot;hash requested&#95;mib={mib} hash&#95;mib={}&quot;,</code> | Evaluate or continue this expression in set&#95;hash(); a final expression without a semicolon supplies its block&#x27;s value. |
| 309 | <code>            self.table.size&#95;mib()</code> | Evaluate or continue this expression in set&#95;hash(); a final expression without a semicolon supplies its block&#x27;s value. |
| 310 | <code>        ));</code> | Evaluate or continue this expression in set&#95;hash(); a final expression without a semicolon supplies its block&#x27;s value. |
| 311 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 312 | <code></code> | Blank line separating related declarations or steps. |
| 313 | <code>    fn shutdown&#95;helpers(&amp;mut self) {</code> | Declare shutdown&#95;helpers(); its parameters and return type define the interface. |
| 314 | <code>        for (sender, &#95;) in &amp;self.helpers {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 315 | <code>            let &#95; = sender.send(Job::Quit);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 316 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 317 | <code>        for (&#95;, handle) in self.helpers.drain(..) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 318 | <code>            handle.join().expect(&quot;search worker failed&quot;);</code> | Evaluate or continue this expression in shutdown&#95;helpers(); a final expression without a semicolon supplies its block&#x27;s value. |
| 319 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 320 | <code>        self.pool.clear();</code> | Evaluate or continue this expression in shutdown&#95;helpers(); a final expression without a semicolon supplies its block&#x27;s value. |
| 321 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 322 | <code></code> | Blank line separating related declarations or steps. |
| 323 | <code>    pub fn ponder&#95;reply(&amp;self, position: &amp;Position, mv: Move) -&gt; Move {</code> | Declare ponder&#95;reply(); its parameters and return type define the interface. |
| 324 | <code>        self.main.ponder&#95;reply(position, mv)</code> | Evaluate or continue this expression in ponder&#95;reply(); a final expression without a semicolon supplies its block&#x27;s value. |
| 325 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 326 | <code></code> | Blank line separating related declarations or steps. |
| 327 | <code>    pub fn think(</code> | Declare think(); its parameters and return type define the interface. |
| 328 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 329 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 330 | <code>        game&#95;history: &amp;&#91;u64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 331 | <code>        seconds: f64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 332 | <code>        longest: f64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 333 | <code>        depth&#95;limit: Option&lt;i32&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 334 | <code>    ) -&gt; Move {</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 335 | <code>        let started = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 336 | <code>        self.table.new&#95;search();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 337 | <code>        self.stop.store(false, Relaxed);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 338 | <code>        self.total&#95;nodes.store(0, Relaxed);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 339 | <code>        self.pool.begin();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 340 | <code>        let active = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 341 | <code>        let mv = self</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 342 | <code>            .main</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 343 | <code>            .think(position, game&#95;history, seconds, longest, depth&#95;limit);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 344 | <code>        self.main.work.active&#95;ns =</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 345 | <code>            (active.elapsed().as&#95;nanos() as u64).saturating&#95;sub(self.main.work.wait&#95;ns);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 346 | <code>        self.main.work.jobs = 1;</code> | Update the named state using the expression and assignment operator shown. |
| 347 | <code>        self.main.work.nodes = self.main.nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 348 | <code>        self.stop.store(true, Relaxed);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 349 | <code>        self.main.flush&#95;nodes();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 350 | <code>        let total = self.total&#95;nodes.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 351 | <code>        self.main.main&#95;control().nodes = total;</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 352 | <code>        let mut metrics = self.main.metrics;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 353 | <code>        metrics.add(self.pool.table&#95;metrics());</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 354 | <code>        let hash&#95;mib = self.table.size&#95;mib();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 355 | <code>        let worker&#95;metrics = self.pool.worker&#95;metrics();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 356 | <code>        let mut work = self.main.work;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 357 | <code>        for helper in worker&#95;metrics.iter().skip(1).take(self.helpers.len()) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 358 | <code>            work.add(&#42;helper);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 359 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 360 | <code>        if self.diagnostics {</code> | Test the condition and execute this branch only when it matches. |
| 361 | <code>            let summary = format!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 362 | <code>                &quot;info string ybwc workers={} splits={} tasks={} cutoffs={} nodes={total}&quot;,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 363 | <code>                self.helpers.len() + 1,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 364 | <code>                self.pool.splits.load(Relaxed),</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 365 | <code>                self.pool.tasks.load(Relaxed),</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 366 | <code>                self.pool.cutoffs.load(Relaxed)</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 367 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 368 | <code>            println!(&quot;{summary}&quot;);</code> | Format and send text to the output destination shown. |
| 369 | <code>            let seconds = started.elapsed().as&#95;secs&#95;f64();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 370 | <code>            let hash&#95;fill&#95;pct = self.table.fill&#95;percentage();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 371 | <code>            for (id, helper) in worker&#95;metrics</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 372 | <code>                .iter()</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 373 | <code>                .enumerate()</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 374 | <code>                .take(self.helpers.len() + 1)</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 375 | <code>            {</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 376 | <code>                let metrics = if id == 0 { self.main.work } else { &#42;helper };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 377 | <code>                self.main</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 378 | <code>                    .main&#95;control()</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 379 | <code>                    .log&#95;search(&amp;format!(&quot;worker id={id} {metrics}&quot;));</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 380 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 381 | <code>            let active&#95;pct = 100.0 &#42; work.active&#95;ns as f64</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 382 | <code>                / (seconds.max(0.000&#95;001) &#42; 1e9 &#42; (self.helpers.len() + 1) as f64);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 383 | <code>            self.main.main&#95;control().log&#95;search(&amp;format!(</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 384 | <code>                &quot;{summary} native&#95;ms={:.1} nps={:.0} hash&#95;mib={hash&#95;mib} hash&#95;fill&#95;pct={hash&#95;fill&#95;pct:.2} {metrics} {work} worker&#95;active&#95;pct={active&#95;pct:.2}&quot;,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 385 | <code>                seconds &#42; 1000.0,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 386 | <code>                total as f64 / seconds.max(0.000&#95;001)</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 387 | <code>            ));</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 388 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 389 | <code>        mv</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 390 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 391 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 392 | <code></code> | Blank line separating related declarations or steps. |
| 393 | <code>impl Drop for Search {</code> | Begin the implementation of methods or a trait for the named type. |
| 394 | <code>    fn drop(&amp;mut self) {</code> | Declare drop(); its parameters and return type define the interface. |
| 395 | <code>        self.shutdown&#95;helpers();</code> | Evaluate or continue this expression in drop(); a final expression without a semicolon supplies its block&#x27;s value. |
| 396 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 397 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 398 | <code></code> | Blank line separating related declarations or steps. |
| 399 | <code>impl Worker {</code> | Begin the implementation of methods or a trait for the named type. |
| 400 | <code>    fn new(</code> | Declare new(); its parameters and return type define the interface. |
| 401 | <code>        control: Option&lt;SearchControl&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 402 | <code>        stop: Arc&lt;AtomicBool&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 403 | <code>        total&#95;nodes: Arc&lt;AtomicU64&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 404 | <code>        table: Arc&lt;Table&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 405 | <code>        pool: Arc&lt;Pool&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 406 | <code>    ) -&gt; Worker {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 407 | <code>        Worker {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 408 | <code>            nodes: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 409 | <code>            metrics: TableMetrics::default(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 410 | <code>            work: SearchMetrics::default(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 411 | <code>            research&#95;level: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 412 | <code>            help&#95;depth: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 413 | <code>            pending&#95;nodes: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 414 | <code>            generation: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 415 | <code>            pool,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 416 | <code>            branch: None,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 417 | <code>            control,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 418 | <code>            stop,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 419 | <code>            total&#95;nodes,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 420 | <code>            best&#95;move: NO&#95;MOVE,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 421 | <code>            history: Vec::with&#95;capacity(1024),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 422 | <code>            killers: &#91;&#91;NO&#95;MOVE; 2&#93;; MAX&#95;PLY&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 423 | <code>            quiet&#95;history: &#91;&#91;&#91;0; 64&#93;; 64&#93;; 2&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 424 | <code>            table,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 425 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 426 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 427 | <code></code> | Blank line separating related declarations or steps. |
| 428 | <code>    /// The main thread&#x27;s control; only the main worker reads commands and the clock.</code> | Source explanation: The main thread&#x27;s control; only the main worker reads commands and the clock. |
| 429 | <code>    fn main&#95;control(&amp;mut self) -&gt; &amp;mut SearchControl {</code> | Declare main&#95;control(); its parameters and return type define the interface. |
| 430 | <code>        self.control</code> | Evaluate or continue this expression in main&#95;control(); a final expression without a semicolon supplies its block&#x27;s value. |
| 431 | <code>            .as&#95;mut()</code> | Evaluate or continue this expression in main&#95;control(); a final expression without a semicolon supplies its block&#x27;s value. |
| 432 | <code>            .expect(&quot;only the main worker owns the control&quot;)</code> | Evaluate or continue this expression in main&#95;control(); a final expression without a semicolon supplies its block&#x27;s value. |
| 433 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 434 | <code></code> | Blank line separating related declarations or steps. |
| 435 | <code>    /// The positions all threads have searched: the shared total, which grows 1,024 at</code> | Source explanation: The positions all threads have searched: the shared total, which grows 1,024 at |
| 436 | <code>    /// a time, plus this thread&#x27;s positions since its last addition.</code> | Source explanation: a time, plus this thread&#x27;s positions since its last addition. |
| 437 | <code>    fn reported&#95;nodes(&amp;self) -&gt; u64 {</code> | Declare reported&#95;nodes(); its parameters and return type define the interface. |
| 438 | <code>        self.total&#95;nodes.load(Relaxed) + self.pending&#95;nodes</code> | Evaluate or continue this expression in reported&#95;nodes(); a final expression without a semicolon supplies its block&#x27;s value. |
| 439 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 440 | <code></code> | Blank line separating related declarations or steps. |
| 441 | <code>    fn flush&#95;nodes(&amp;mut self) {</code> | Declare flush&#95;nodes(); its parameters and return type define the interface. |
| 442 | <code>        self.total&#95;nodes.fetch&#95;add(self.pending&#95;nodes, Relaxed);</code> | Evaluate or continue this expression in flush&#95;nodes(); a final expression without a semicolon supplies its block&#x27;s value. |
| 443 | <code>        self.pending&#95;nodes = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 444 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 445 | <code></code> | Blank line separating related declarations or steps. |
| 446 | <code>    /// Only the main worker consumes UCI commands; all workers observe cancellation.</code> | Source explanation: Only the main worker consumes UCI commands; all workers observe cancellation. |
| 447 | <code>    fn poll(&amp;mut self) -&gt; Result&lt;(), SearchTimeout&gt; {</code> | Declare poll(); its parameters and return type define the interface. |
| 448 | <code>        let total = self.reported&#95;nodes();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 449 | <code>        if let Some(control) = &amp;mut self.control</code> | Test the condition and execute this branch only when it matches. |
| 450 | <code>            &amp;&amp; control.poll(total)</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 451 | <code>        {</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 452 | <code>            self.stop.store(true, Relaxed);</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 453 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 454 | <code>        if self.stop.load(Relaxed)</code> | Test the condition and execute this branch only when it matches. |
| 455 | <code>            &#124;&#124; self</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 456 | <code>                .branch</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 457 | <code>                .as&#95;ref()</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 458 | <code>                .is&#95;some&#95;and(&#124;branch&#124; branch.is&#95;canceled())</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 459 | <code>        {</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 460 | <code>            return Err(SearchTimeout);</code> | Leave the current function with the displayed result. |
| 461 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 462 | <code>        Ok(())</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 463 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 464 | <code></code> | Blank line separating related declarations or steps. |
| 465 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 466 | <code>    fn count&#95;node(&amp;mut self) -&gt; Result&lt;(), SearchTimeout&gt; {</code> | Declare count&#95;node(); its parameters and return type define the interface. |
| 467 | <code>        self.nodes += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 468 | <code>        self.pending&#95;nodes += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 469 | <code>        if self.pending&#95;nodes &gt;= 1024 {</code> | Test the condition and execute this branch only when it matches. |
| 470 | <code>            self.flush&#95;nodes();</code> | Evaluate or continue this expression in count&#95;node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 471 | <code>            self.poll()?;</code> | Evaluate or continue this expression in count&#95;node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 472 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 473 | <code>        Ok(())</code> | Evaluate or continue this expression in count&#95;node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 474 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 475 | <code></code> | Blank line separating related declarations or steps. |
| 476 | <code>    /// Count verification attempts; nested attempts contribute nodes only once per</code> | Source explanation: Count verification attempts; nested attempts contribute nodes only once per |
| 477 | <code>    /// worker. Helper threads account their own work rather than borrowing node totals.</code> | Source explanation: worker. Helper threads account their own work rather than borrowing node totals. |
| 478 | <code>    #&#91;allow(clippy::too&#95;many&#95;arguments)&#93;</code> | Compiler attribute controlling the following declaration. |
| 479 | <code>    fn research(</code> | Declare research(); its parameters and return type define the interface. |
| 480 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in research(); a final expression without a semicolon supplies its block&#x27;s value. |
| 481 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 482 | <code>        depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 483 | <code>        ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 484 | <code>        alpha: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 485 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 486 | <code>        kind: Research,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 487 | <code>    ) -&gt; Result&lt;i32, SearchTimeout&gt; {</code> | Evaluate or continue this expression in research(); a final expression without a semicolon supplies its block&#x27;s value. |
| 488 | <code>        match kind {</code> | Select an arm by matching the expression against the listed patterns. |
| 489 | <code>            Research::Pvs =&gt; self.work.pvs&#95;researches += 1,</code> | Handle this matching pattern with the expression or block on its right. |
| 490 | <code>            Research::Lmr =&gt; self.work.lmr&#95;researches += 1,</code> | Handle this matching pattern with the expression or block on its right. |
| 491 | <code>            Research::Aspiration =&gt; self.work.aspiration&#95;researches += 1,</code> | Handle this matching pattern with the expression or block on its right. |
| 492 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 493 | <code>        let nodes = self.nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 494 | <code>        self.research&#95;level += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 495 | <code>        let result = self.alpha&#95;beta(position, depth, ply, alpha, beta, true);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 496 | <code>        self.research&#95;level -= 1;</code> | Update the named state using the expression and assignment operator shown. |
| 497 | <code>        if self.research&#95;level == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 498 | <code>            self.work.research&#95;nodes += self.nodes - nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 499 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 500 | <code>        result</code> | Evaluate or continue this expression in research(); a final expression without a semicolon supplies its block&#x27;s value. |
| 501 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 502 | <code></code> | Blank line separating related declarations or steps. |
| 503 | <code>    /// Prepare for a new search: recent cutoffs matter more than older ones.</code> | Source explanation: Prepare for a new search: recent cutoffs matter more than older ones. |
| 504 | <code>    fn begin&#95;search(&amp;mut self) {</code> | Declare begin&#95;search(); its parameters and return type define the interface. |
| 505 | <code>        self.nodes = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 506 | <code>        self.metrics = TableMetrics::default();</code> | Update the named state using the expression and assignment operator shown. |
| 507 | <code>        self.work = SearchMetrics::default();</code> | Update the named state using the expression and assignment operator shown. |
| 508 | <code>        self.research&#95;level = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 509 | <code>        self.pending&#95;nodes = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 510 | <code>        self.best&#95;move = NO&#95;MOVE;</code> | Update the named state using the expression and assignment operator shown. |
| 511 | <code>        for side in &amp;mut self.quiet&#95;history {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 512 | <code>            for row in side {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 513 | <code>                for entry in row {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 514 | <code>                    &#42;entry /= 2;</code> | Update the named state using the expression and assignment operator shown. |
| 515 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 516 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 517 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 518 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 519 | <code></code> | Blank line separating related declarations or steps. |
| 520 | <code>    /// Search siblings using the alpha snapshot captured when each move is claimed.</code> | Source explanation: Search siblings using the alpha snapshot captured when each move is claimed. |
| 521 | <code>    /// A successful reduced probe receives full-depth verification before publication.</code> | Source explanation: A successful reduced probe receives full-depth verification before publication. |
| 522 | <code>    fn search&#95;split(&amp;mut self, split: &amp;Arc&lt;Split&gt;) {</code> | Declare search&#95;split(); its parameters and return type define the interface. |
| 523 | <code>        while self.poll().is&#95;ok() {</code> | Repeat this block while its condition remains true. |
| 524 | <code>            let Some((candidate, alpha)) = split.claim() else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 525 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 526 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 527 | <code>            if !self.search&#95;split&#95;move(split, candidate, alpha) {</code> | Test the condition and execute this branch only when it matches. |
| 528 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 529 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 530 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 531 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 532 | <code></code> | Blank line separating related declarations or steps. |
| 533 | <code>    fn search&#95;split&#95;move(&amp;mut self, split: &amp;Arc&lt;Split&gt;, candidate: Candidate, alpha: i32) -&gt; bool {</code> | Declare search&#95;split&#95;move(); its parameters and return type define the interface. |
| 534 | <code>        self.pool.tasks.fetch&#95;add(1, Relaxed);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 535 | <code>        if self.poll().is&#95;err() {</code> | Test the condition and execute this branch only when it matches. |
| 536 | <code>            if split.is&#95;canceled() {</code> | Test the condition and execute this branch only when it matches. |
| 537 | <code>                self.work.cutoff&#95;discarded&#95;tasks += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 538 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 539 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 540 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 541 | <code>        let child = make&#95;move(&amp;split.position, candidate.mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 542 | <code>        self.table.prefetch(child.key);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 543 | <code>        let length = self.history.len();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 544 | <code>        let nodes = self.nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 545 | <code>        let accounted = self.work.cutoff&#95;discarded&#95;nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 546 | <code>        let stopped = self.work.stop&#95;discarded&#95;nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 547 | <code>        let result = self.search&#95;sibling(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 548 | <code>            &amp;child,</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 549 | <code>            split.depth,</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 550 | <code>            split.ply,</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 551 | <code>            alpha,</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 552 | <code>            split.beta,</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 553 | <code>            candidate.reduce,</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 554 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 555 | <code>        self.history.truncate(length);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 556 | <code>        if split.is&#95;canceled() {</code> | Test the condition and execute this branch only when it matches. |
| 557 | <code>            self.work.cutoff&#95;discarded&#95;tasks += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 558 | <code>            self.work.cutoff&#95;discarded&#95;nodes +=</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 559 | <code>                (self.nodes - nodes).saturating&#95;sub(self.work.cutoff&#95;discarded&#95;nodes - accounted);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 560 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 561 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 562 | <code>        let Ok(score) = result else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 563 | <code>            self.work.stop&#95;discarded&#95;nodes +=</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 564 | <code>                (self.nodes - nodes).saturating&#95;sub(self.work.stop&#95;discarded&#95;nodes - stopped);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 565 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 566 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 567 | <code>        match split.publish(candidate.mv, score) {</code> | Select an arm by matching the expression against the listed patterns. |
| 568 | <code>            Some(true) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 569 | <code>                self.pool.cutoffs.fetch&#95;add(1, Relaxed);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 570 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 571 | <code>            Some(false) =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 572 | <code>            None =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 573 | <code>                self.work.cutoff&#95;discarded&#95;tasks += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 574 | <code>                self.work.cutoff&#95;discarded&#95;nodes += (self.nodes - nodes)</code> | Update the named state using the expression and assignment operator shown. |
| 575 | <code>                    .saturating&#95;sub(self.work.cutoff&#95;discarded&#95;nodes - accounted);</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 576 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 577 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 578 | <code>        true</code> | Evaluate or continue this expression in search&#95;split&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 579 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 580 | <code></code> | Blank line separating related declarations or steps. |
| 581 | <code>    /// Borrow one descendant task, preserving the suspended search&#x27;s full context.</code> | Source explanation: Borrow one descendant task, preserving the suspended search&#x27;s full context. |
| 582 | <code>    fn help&#95;descendant(&amp;mut self, scope: &amp;Arc&lt;Split&gt;) -&gt; bool {</code> | Declare help&#95;descendant(); its parameters and return type define the interface. |
| 583 | <code>        if self.help&#95;depth &gt;= MAX&#95;HELP&#95;DEPTH &#124;&#124; self.poll().is&#95;err() {</code> | Test the condition and execute this branch only when it matches. |
| 584 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 585 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 586 | <code>        let Some(task) = self.pool.try&#95;help(scope) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 587 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 588 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 589 | <code>        self.pool.end&#95;wait(scope);</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 590 | <code>        let started = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 591 | <code>        let waits = self.work.wait&#95;ns;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 592 | <code>        let nodes = self.nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 593 | <code>        let history = std::mem::replace(&amp;mut self.history, task.split.history.clone());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 594 | <code>        let branch = self.branch.replace(task.split.clone());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 595 | <code>        let research = self.research&#95;level;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 596 | <code>        let best = self.best&#95;move;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 597 | <code>        self.research&#95;level = u32::from(task.split.research);</code> | Update the named state using the expression and assignment operator shown. |
| 598 | <code>        self.help&#95;depth += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 599 | <code>        self.work.help&#95;tasks += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 600 | <code>        self.search&#95;split&#95;move(&amp;task.split, task.candidate, task.alpha);</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 601 | <code>        self.help&#95;depth -= 1;</code> | Update the named state using the expression and assignment operator shown. |
| 602 | <code>        if self.help&#95;depth == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 603 | <code>            self.work.help&#95;nodes += self.nodes - nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 604 | <code>            self.work.help&#95;ns +=</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 605 | <code>                (started.elapsed().as&#95;nanos() as u64).saturating&#95;sub(self.work.wait&#95;ns - waits);</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 606 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 607 | <code>        if research == 0 &amp;&amp; task.split.research {</code> | Test the condition and execute this branch only when it matches. |
| 608 | <code>            self.work.research&#95;nodes += self.nodes - nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 609 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 610 | <code>        self.research&#95;level = research;</code> | Update the named state using the expression and assignment operator shown. |
| 611 | <code>        self.best&#95;move = best;</code> | Update the named state using the expression and assignment operator shown. |
| 612 | <code>        self.history = history;</code> | Update the named state using the expression and assignment operator shown. |
| 613 | <code>        self.branch = branch;</code> | Update the named state using the expression and assignment operator shown. |
| 614 | <code>        self.flush&#95;nodes();</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 615 | <code>        // Completion is visible only after this worker restores its suspended state.</code> | Source explanation: Completion is visible only after this worker restores its suspended state. |
| 616 | <code>        drop(task);</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 617 | <code>        self.pool.begin&#95;wait(scope);</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 618 | <code>        true</code> | Evaluate or continue this expression in help&#95;descendant(); a final expression without a semicolon supplies its block&#x27;s value. |
| 619 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 620 | <code></code> | Blank line separating related declarations or steps. |
| 621 | <code>    /// Help only descendants; cancellation drains participants without starting work.</code> | Source explanation: Help only descendants; cancellation drains participants without starting work. |
| 622 | <code>    fn drain&#95;split(&amp;mut self, split: &amp;Arc&lt;Split&gt;) {</code> | Declare drain&#95;split(); its parameters and return type define the interface. |
| 623 | <code>        let can&#95;help = self.help&#95;depth &lt; MAX&#95;HELP&#95;DEPTH;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 624 | <code>        if can&#95;help {</code> | Test the condition and execute this branch only when it matches. |
| 625 | <code>            self.pool.begin&#95;wait(split);</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 626 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 627 | <code>        loop {</code> | Repeat until an explicit break, return, or other control transfer ends the loop. |
| 628 | <code>            let mut state = split.progress.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 629 | <code>            if state.helpers == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 630 | <code>                state.closed = true;</code> | Update the named state using the expression and assignment operator shown. |
| 631 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 632 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 633 | <code>            drop(state);</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 634 | <code>            let &#95; = self.poll();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 635 | <code>            let canceled = self.stop.load(Relaxed) &#124;&#124; split.is&#95;canceled();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 636 | <code>            if !canceled &amp;&amp; self.help&#95;descendant(split) {</code> | Test the condition and execute this branch only when it matches. |
| 637 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 638 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 639 | <code>            let waiting = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 640 | <code>            state = split.progress.lock().unwrap();</code> | Update the named state using the expression and assignment operator shown. |
| 641 | <code>            if state.helpers != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 642 | <code>                let (next, &#95;) = split</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 643 | <code>                    .changed</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 644 | <code>                    .wait&#95;timeout(state, Duration::from&#95;millis(1))</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 645 | <code>                    .unwrap();</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 646 | <code>                state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 647 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 648 | <code>            drop(state);</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 649 | <code>            let elapsed = waiting.elapsed().as&#95;nanos() as u64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 650 | <code>            self.work.wait&#95;ns += elapsed;</code> | Update the named state using the expression and assignment operator shown. |
| 651 | <code>            // Classify by state at the start of the wait, with at most 1 ms resolution.</code> | Source explanation: Classify by state at the start of the wait, with at most 1 ms resolution. |
| 652 | <code>            if canceled {</code> | Test the condition and execute this branch only when it matches. |
| 653 | <code>                self.work.cancel&#95;wait&#95;ns += elapsed;</code> | Update the named state using the expression and assignment operator shown. |
| 654 | <code>            } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 655 | <code>                self.work.result&#95;wait&#95;ns += elapsed;</code> | Update the named state using the expression and assignment operator shown. |
| 656 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 657 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 658 | <code>        if can&#95;help {</code> | Test the condition and execute this branch only when it matches. |
| 659 | <code>            self.pool.end&#95;wait(split);</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 660 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 661 | <code>        self.pool.retire(split);</code> | Evaluate or continue this expression in drain&#95;split(); a final expression without a semicolon supplies its block&#x27;s value. |
| 662 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 663 | <code></code> | Blank line separating related declarations or steps. |
| 664 | <code>    fn search&#95;sibling(</code> | Declare search&#95;sibling(); its parameters and return type define the interface. |
| 665 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 666 | <code>        child: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 667 | <code>        depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 668 | <code>        ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 669 | <code>        alpha: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 670 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 671 | <code>        reduce: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 672 | <code>    ) -&gt; Result&lt;i32, SearchTimeout&gt; {</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 673 | <code>        let mut score = -self.alpha&#95;beta(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 674 | <code>            child,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 675 | <code>            depth - 1 - i32::from(reduce),</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 676 | <code>            ply + 1,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 677 | <code>            -alpha - 1,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 678 | <code>            -alpha,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 679 | <code>            true,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 680 | <code>        )?;</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 681 | <code>        if score &gt; alpha &amp;&amp; (reduce &#124;&#124; score &lt; beta) {</code> | Test the condition and execute this branch only when it matches. |
| 682 | <code>            score = -self.research(</code> | Update the named state using the expression and assignment operator shown. |
| 683 | <code>                child,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 684 | <code>                depth - 1,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 685 | <code>                ply + 1,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 686 | <code>                -beta,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 687 | <code>                -alpha,</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 688 | <code>                if reduce { Research::Lmr } else { Research::Pvs },</code> | Test the condition and execute this branch only when it matches. |
| 689 | <code>            )?;</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 690 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 691 | <code>        Ok(score)</code> | Evaluate or continue this expression in search&#95;sibling(); a final expression without a semicolon supplies its block&#x27;s value. |
| 692 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 693 | <code></code> | Blank line separating related declarations or steps. |
| 694 | <code>    /// Reserve real idle workers, then search and drain this split before returning.</code> | Source explanation: Reserve real idle workers, then search and drain this split before returning. |
| 695 | <code>    #&#91;allow(clippy::too&#95;many&#95;arguments)&#93;</code> | Compiler attribute controlling the following declaration. |
| 696 | <code>    fn split&#95;remaining(</code> | Declare split&#95;remaining(); its parameters and return type define the interface. |
| 697 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 698 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 699 | <code>        entries: &amp;mut &#91;u64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 700 | <code>        depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 701 | <code>        ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 702 | <code>        alpha: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 703 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 704 | <code>        best&#95;score: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 705 | <code>        best: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 706 | <code>        check: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 707 | <code>    ) -&gt; Option&lt;Result&lt;(i32, Move, usize), SearchTimeout&gt;&gt; {</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 708 | <code>        if depth &lt; SPLIT&#95;DEPTH</code> | Test the condition and execute this branch only when it matches. |
| 709 | <code>            &#124;&#124; ply &gt; SPLIT&#95;PLY</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 710 | <code>            &#124;&#124; (!self.pool.has&#95;idle() &amp;&amp; !self.pool.has&#95;waiter(self.branch.as&#95;ref()))</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 711 | <code>        {</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 712 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 713 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 714 | <code>        // place&#95;next() orders only the moves the search has reached, and a split</code> | Source explanation: place&#95;next() orders only the moves the search has reached, and a split |
| 715 | <code>        // hands out every move left, so those go into order here</code> | Source explanation: hands out every move left, so those go into order here |
| 716 | <code>        entries.sort&#95;unstable&#95;by(&#124;a, b&#124; b.cmp(a));</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 717 | <code>        let mut candidates = Vec::with&#95;capacity(entries.len());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 718 | <code>        let mut legal&#95;count = 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 719 | <code>        for &amp;entry in entries.iter() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 720 | <code>            let mv = entry&#95;move(entry);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 721 | <code>            let child = make&#95;move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 722 | <code>            if in&#95;check(&amp;child, position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 723 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 724 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 725 | <code>            legal&#95;count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 726 | <code>            let quiet = captured&#95;kind(position, mv) == EMPTY &amp;&amp; move&#95;promotion(mv) == 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 727 | <code>            let calm = quiet &amp;&amp; !check &amp;&amp; !in&#95;check(&amp;child, !position.white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 728 | <code>            let reduce = depth &gt;= 3 &amp;&amp; legal&#95;count &gt; 3 &amp;&amp; calm &amp;&amp; !self.killers&#91;ply&#93;.contains(&amp;mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 729 | <code>            candidates.push(Candidate { mv, reduce });</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 730 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 731 | <code>        if candidates.len() &lt; 2 {</code> | Test the condition and execute this branch only when it matches. |
| 732 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 733 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 734 | <code>        let helpers = self.pool.reserve(candidates.len() - 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 735 | <code>        if helpers.is&#95;empty() &amp;&amp; !self.pool.has&#95;waiter(self.branch.as&#95;ref()) {</code> | Test the condition and execute this branch only when it matches. |
| 736 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 737 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 738 | <code>        let split = Arc::new(Split {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 739 | <code>            position: &#42;position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 740 | <code>            history: self.history.clone(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 741 | <code>            moves: candidates,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 742 | <code>            depth,</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 743 | <code>            ply,</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 744 | <code>            beta,</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 745 | <code>            parent: self.branch.clone(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 746 | <code>            canceled: AtomicBool::new(false),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 747 | <code>            research: self.research&#95;level != 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 748 | <code>            progress: Mutex::new(Progress {</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 749 | <code>                next: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 750 | <code>                helpers: helpers.len(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 751 | <code>                closed: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 752 | <code>                alpha,</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 753 | <code>                score: best&#95;score,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 754 | <code>                best,</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 755 | <code>            }),</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 756 | <code>            changed: Condvar::new(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 757 | <code>        });</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 758 | <code>        self.pool.splits.fetch&#95;add(1, Relaxed);</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 759 | <code>        self.pool.publish(&amp;split);</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 760 | <code>        for helper in helpers {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 761 | <code>            if helper.send(Job::Search(split.clone())).is&#95;err() {</code> | Test the condition and execute this branch only when it matches. |
| 762 | <code>                split.helper&#95;done();</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 763 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 764 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 765 | <code>        let parent = self.branch.replace(split.clone());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 766 | <code>        self.search&#95;split(&amp;split);</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 767 | <code>        self.branch = parent;</code> | Update the named state using the expression and assignment operator shown. |
| 768 | <code>        self.drain&#95;split(&amp;split);</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 769 | <code>        let state = split.progress.lock().unwrap();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 770 | <code>        if split.is&#95;canceled() {</code> | Test the condition and execute this branch only when it matches. |
| 771 | <code>            self.work.cutoff&#95;unclaimed&#95;tasks += (split.moves.len() - state.next) as u64;</code> | Update the named state using the expression and assignment operator shown. |
| 772 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 773 | <code>        let result = (state.score, state.best, legal&#95;count);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 774 | <code>        drop(state);</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 775 | <code>        Some(self.poll().map(&#124;()&#124; result))</code> | Evaluate or continue this expression in split&#95;remaining(); a final expression without a semicolon supplies its block&#x27;s value. |
| 776 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 777 | <code></code> | Blank line separating related declarations or steps. |
| 778 | <code>    /// Search captures and queen promotions, or every legal evasion in check.</code> | Source explanation: Search captures and queen promotions, or every legal evasion in check. |
| 779 | <code>    /// Checkmate and stalemate take precedence over standing pat or pruning.</code> | Source explanation: Checkmate and stalemate take precedence over standing pat or pruning. |
| 780 | <code>    fn quiescence(</code> | Declare quiescence(); its parameters and return type define the interface. |
| 781 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 782 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 783 | <code>        mut alpha: i32,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 784 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 785 | <code>        ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 786 | <code>    ) -&gt; Result&lt;i32, SearchTimeout&gt; {</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 787 | <code>        self.count&#95;node()?;</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 788 | <code>        let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 789 | <code>        let check = in&#95;check(position, white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 790 | <code>        if !has&#95;legal&#95;move(position, check) {</code> | Test the condition and execute this branch only when it matches. |
| 791 | <code>            return Ok(if check { -(MATE&#95;SCORE - ply as i32) } else { 0 });</code> | Leave the current function with the displayed result. |
| 792 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 793 | <code>        if self.is&#95;draw(position) {</code> | Test the condition and execute this branch only when it matches. |
| 794 | <code>            return Ok(0);</code> | Leave the current function with the displayed result. |
| 795 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 796 | <code>        let stand&#95;pat = evaluate(position);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 797 | <code>        if ply &gt;= MAX&#95;PLY {</code> | Test the condition and execute this branch only when it matches. |
| 798 | <code>            return Ok(stand&#95;pat); // Bound the stack during long checking sequences</code> | Leave the current function with the displayed result. |
| 799 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 800 | <code>        if !check {</code> | Test the condition and execute this branch only when it matches. |
| 801 | <code>            if stand&#95;pat &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 802 | <code>                return Ok(stand&#95;pat);</code> | Leave the current function with the displayed result. |
| 803 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 804 | <code>            alpha = alpha.max(stand&#95;pat);</code> | Update the named state using the expression and assignment operator shown. |
| 805 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 806 | <code></code> | Blank line separating related declarations or steps. |
| 807 | <code>        let mut entries = EntryList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 808 | <code>        if check {</code> | Test the condition and execute this branch only when it matches. |
| 809 | <code>            ordered&#95;moves(</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 810 | <code>                position,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 811 | <code>                NO&#95;MOVE,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 812 | <code>                &#91;NO&#95;MOVE; 2&#93;,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 813 | <code>                &amp;self.quiet&#95;history&#91;usize::from(!white)&#93;,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 814 | <code>                &amp;mut entries,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 815 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 816 | <code>            entries.as&#95;mut&#95;slice().sort&#95;unstable&#95;by(&#124;a, b&#124; b.cmp(a));</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 817 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 818 | <code>            let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 819 | <code>            generate&#95;captures(position, &amp;mut list);</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 820 | <code>            for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 821 | <code>                let priority = capture&#95;priority(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 822 | <code>                if priority &gt;= CAPTURE {</code> | Test the condition and execute this branch only when it matches. |
| 823 | <code>                    entries.add(sort&#95;entry(priority, mv));</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 824 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 825 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 826 | <code>            entries.as&#95;mut&#95;slice().sort&#95;unstable&#95;by(&#124;a, b&#124; b.cmp(a));</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 827 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 828 | <code>        for &amp;mut entry in entries.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 829 | <code>            let mv = entry&#95;move(entry);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 830 | <code>            let child = make&#95;move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 831 | <code>            if in&#95;check(&amp;child, white) {</code> | Test the condition and execute this branch only when it matches. |
| 832 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 833 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 834 | <code>            if !check &amp;&amp; move&#95;promotion(mv) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 835 | <code>                let victim = PIECE&#95;VALUES&#91;captured&#95;kind(position, mv) as usize&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 836 | <code>                if stand&#95;pat + victim + DELTA&#95;MARGIN &lt; alpha {</code> | Test the condition and execute this branch only when it matches. |
| 837 | <code>                    continue;</code> | Skip the rest of this iteration and advance the loop. |
| 838 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 839 | <code>                let exchange =</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 840 | <code>                    exchange&#95;score(&amp;position.board, &amp;position.by&#95;kind, &amp;position.by&#95;side, mv);</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 841 | <code>                if exchange &lt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 842 | <code>                    continue;</code> | Skip the rest of this iteration and advance the loop. |
| 843 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 844 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 845 | <code>            // Evasions can repeat a position, so quiescence keeps the path too.</code> | Source explanation: Evasions can repeat a position, so quiescence keeps the path too. |
| 846 | <code>            self.history.push(position.key);</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 847 | <code>            let result = self.quiescence(&amp;child, -beta, -alpha, ply + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 848 | <code>            self.history.pop();</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 849 | <code>            let score = -result?;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 850 | <code>            if score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 851 | <code>                return Ok(score);</code> | Leave the current function with the displayed result. |
| 852 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 853 | <code>            alpha = alpha.max(score);</code> | Update the named state using the expression and assignment operator shown. |
| 854 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 855 | <code>        Ok(alpha)</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 856 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 857 | <code></code> | Blank line separating related declarations or steps. |
| 858 | <code>    /// Return true for the fifty-move rule or a repeated position.</code> | Source explanation: Return true for the fifty-move rule or a repeated position. |
| 859 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 860 | <code>    fn is&#95;draw(&amp;self, position: &amp;Position) -&gt; bool {</code> | Declare is&#95;draw(); its parameters and return type define the interface. |
| 861 | <code>        if position.halfmove &gt;= 100 {</code> | Test the condition and execute this branch only when it matches. |
| 862 | <code>            return true;</code> | Leave the current function with the displayed result. |
| 863 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 864 | <code>        // The slice history&#91;len - halfmove:&#93; of Remus5, where a negative start counts</code> | Source explanation: The slice history&#91;len - halfmove:&#93; of Remus5, where a negative start counts |
| 865 | <code>        // back from the end of the list</code> | Source explanation: back from the end of the list |
| 866 | <code>        let length = self.history.len() as i64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 867 | <code>        let mut start = length - i64::from(position.halfmove);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 868 | <code>        if start &lt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 869 | <code>            start = (length + start).max(0);</code> | Update the named state using the expression and assignment operator shown. |
| 870 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 871 | <code>        self.history&#91;start as usize..&#93;.contains(&amp;position.key)</code> | Evaluate or continue this expression in is&#95;draw(); a final expression without a semicolon supplies its block&#x27;s value. |
| 872 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 873 | <code></code> | Blank line separating related declarations or steps. |
| 874 | <code>    /// Return the score for the side to move, searching depth moves ahead.</code> | Source explanation: Return the score for the side to move, searching depth moves ahead. |
| 875 | <code>    fn alpha&#95;beta(</code> | Declare alpha&#95;beta(); its parameters and return type define the interface. |
| 876 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 877 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 878 | <code>        mut depth: i32,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 879 | <code>        ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 880 | <code>        mut alpha: i32,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 881 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 882 | <code>        can&#95;null: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 883 | <code>    ) -&gt; Result&lt;i32, SearchTimeout&gt; {</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 884 | <code>        self.count&#95;node()?;</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 885 | <code>        let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 886 | <code>        let key = position.key;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 887 | <code>        let check = in&#95;check(position, white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 888 | <code>        if ply &gt; 0 &amp;&amp; self.is&#95;draw(position) {</code> | Test the condition and execute this branch only when it matches. |
| 889 | <code>            if check &amp;&amp; !has&#95;legal&#95;move(position, true) {</code> | Test the condition and execute this branch only when it matches. |
| 890 | <code>                return Ok(-(MATE&#95;SCORE - ply as i32));</code> | Leave the current function with the displayed result. |
| 891 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 892 | <code>            return Ok(0);</code> | Leave the current function with the displayed result. |
| 893 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 894 | <code>        if ply &gt;= MAX&#95;PLY {</code> | Test the condition and execute this branch only when it matches. |
| 895 | <code>            return self.quiescence(position, alpha, beta, ply);</code> | Leave the current function with the displayed result. |
| 896 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 897 | <code>        if check {</code> | Test the condition and execute this branch only when it matches. |
| 898 | <code>            depth += 1; // Never stop the search in check</code> | Update the named state using the expression and assignment operator shown. |
| 899 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 900 | <code>        if depth &lt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 901 | <code>            return self.quiescence(position, alpha, beta, ply);</code> | Leave the current function with the displayed result. |
| 902 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 903 | <code>        let mut first = NO&#95;MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 904 | <code>        self.metrics.probes += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 905 | <code>        if let Some(stored) = self.table.probe(key, ply as i32) {</code> | Test the condition and execute this branch only when it matches. |
| 906 | <code>            self.metrics.hits += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 907 | <code>            first = stored.mv;</code> | Update the named state using the expression and assignment operator shown. |
| 908 | <code>            if ply &gt; 0</code> | Test the condition and execute this branch only when it matches. |
| 909 | <code>                &amp;&amp; stored.depth &gt;= depth</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 910 | <code>                &amp;&amp; (stored.bound == EXACT</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 911 | <code>                    &#124;&#124; (stored.bound == LOWER &amp;&amp; stored.score &gt;= beta)</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 912 | <code>                    &#124;&#124; (stored.bound == UPPER &amp;&amp; stored.score &lt;= alpha))</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 913 | <code>            {</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 914 | <code>                self.metrics.cutoffs += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 915 | <code>                self.metrics.refreshes += u64::from(self.table.refresh(key, &amp;stored));</code> | Update the named state using the expression and assignment operator shown. |
| 916 | <code>                return Ok(stored.score);</code> | Leave the current function with the displayed result. |
| 917 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 918 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 919 | <code>        // Only two tests read the score, both out of check: null move at depth 3 and</code> | Source explanation: Only two tests read the score, both out of check: null move at depth 3 and |
| 920 | <code>        // up, and futility at depth 1. Every other node skips the evaluation.</code> | Source explanation: up, and futility at depth 1. Every other node skips the evaluation. |
| 921 | <code>        let needs&#95;score = !check &amp;&amp; (depth == 1 &#124;&#124; (depth &gt;= 3 &amp;&amp; can&#95;null &amp;&amp; ply &gt; 0));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 922 | <code>        let static&#95;score = if needs&#95;score { evaluate(position) } else { 0 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 923 | <code>        // Null move: let the opponent move twice. If the side to move still stands above</code> | Source explanation: Null move: let the opponent move twice. If the side to move still stands above |
| 924 | <code>        // beta with a shallower search, its real moves will too.</code> | Source explanation: beta with a shallower search, its real moves will too. |
| 925 | <code>        if can&#95;null</code> | Test the condition and execute this branch only when it matches. |
| 926 | <code>            &amp;&amp; !check</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 927 | <code>            &amp;&amp; ply &gt; 0</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 928 | <code>            &amp;&amp; depth &gt;= 3</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 929 | <code>            &amp;&amp; static&#95;score &gt;= beta</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 930 | <code>            &amp;&amp; has&#95;pieces(position)</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 931 | <code>            &amp;&amp; has&#95;legal&#95;move(position, false)</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 932 | <code>        {</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 933 | <code>            let null = make&#95;null&#95;move(position);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 934 | <code>            self.table.prefetch(null.key);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 935 | <code>            self.history.push(key);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 936 | <code>            let result = self.alpha&#95;beta(&amp;null, depth - 3, ply + 1, -beta, -beta + 1, false);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 937 | <code>            self.history.pop();</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 938 | <code>            if -result? &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 939 | <code>                return Ok(beta);</code> | Leave the current function with the displayed result. |
| 940 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 941 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 942 | <code>        self.history.push(key);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 943 | <code>        let original&#95;alpha = alpha;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 944 | <code>        let result = (&#124;&#124; {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 945 | <code>            let mut best&#95;score = -MATE&#95;SCORE - 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 946 | <code>            let mut best = NO&#95;MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 947 | <code>            let mut legal&#95;count = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 948 | <code>            let mut entries = EntryList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 949 | <code>            ordered&#95;moves(</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 950 | <code>                position,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 951 | <code>                first,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 952 | <code>                self.killers&#91;ply&#93;,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 953 | <code>                &amp;self.quiet&#95;history&#91;usize::from(!white)&#93;,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 954 | <code>                &amp;mut entries,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 955 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 956 | <code>            let entries = entries.as&#95;mut&#95;slice();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 957 | <code>            for index in 0..entries.len() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 958 | <code>                place&#95;next(entries, index);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 959 | <code>                let mv = entry&#95;move(entries&#91;index&#93;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 960 | <code>                let child = make&#95;move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 961 | <code>                if depth &gt; 1 {</code> | Test the condition and execute this branch only when it matches. |
| 962 | <code>                    // The child looks itself up in the table; at depth 1 it goes</code> | Source explanation: The child looks itself up in the table; at depth 1 it goes |
| 963 | <code>                    // straight to quiescence, which does not</code> | Source explanation: straight to quiescence, which does not |
| 964 | <code>                    self.table.prefetch(child.key);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 965 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 966 | <code>                if in&#95;check(&amp;child, white) {</code> | Test the condition and execute this branch only when it matches. |
| 967 | <code>                    continue; // Illegal: it leaves the king attacked</code> | Skip the rest of this iteration and advance the loop. |
| 968 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 969 | <code>                legal&#95;count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 970 | <code>                let quiet = captured&#95;kind(position, mv) == EMPTY &amp;&amp; move&#95;promotion(mv) == 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 971 | <code>                let calm = quiet &amp;&amp; !check &amp;&amp; !in&#95;check(&amp;child, !white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 972 | <code>                // Futility: one move from the leaves, a quiet move cannot lift a position far</code> | Source explanation: Futility: one move from the leaves, a quiet move cannot lift a position far |
| 973 | <code>                // below alpha</code> | Source explanation: below alpha |
| 974 | <code>                if depth == 1 &amp;&amp; calm &amp;&amp; legal&#95;count &gt; 1 &amp;&amp; static&#95;score + FUTILITY&#95;MARGIN &lt;= alpha</code> | Test the condition and execute this branch only when it matches. |
| 975 | <code>                {</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 976 | <code>                    continue;</code> | Skip the rest of this iteration and advance the loop. |
| 977 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 978 | <code>                let mut score;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 979 | <code>                if legal&#95;count == 1 {</code> | Test the condition and execute this branch only when it matches. |
| 980 | <code>                    score = -self.alpha&#95;beta(&amp;child, depth - 1, ply + 1, -beta, -alpha, true)?;</code> | Update the named state using the expression and assignment operator shown. |
| 981 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 982 | <code>                    // Late move reduction: a late quiet move gets one ply less</code> | Source explanation: Late move reduction: a late quiet move gets one ply less |
| 983 | <code>                    let reduce = depth &gt;= 3</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 984 | <code>                        &amp;&amp; legal&#95;count &gt; 3</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 985 | <code>                        &amp;&amp; calm</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 986 | <code>                        &amp;&amp; mv != self.killers&#91;ply&#93;&#91;0&#93;</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 987 | <code>                        &amp;&amp; mv != self.killers&#91;ply&#93;&#91;1&#93;;</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 988 | <code>                    // Principal variation search: prove the move is no better than alpha with</code> | Source explanation: Principal variation search: prove the move is no better than alpha with |
| 989 | <code>                    // a null window, and search again if it is</code> | Source explanation: a null window, and search again if it is |
| 990 | <code>                    let reduced&#95;depth = depth - 1 - i32::from(reduce);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 991 | <code>                    score = -self.alpha&#95;beta(</code> | Update the named state using the expression and assignment operator shown. |
| 992 | <code>                        &amp;child,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 993 | <code>                        reduced&#95;depth,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 994 | <code>                        ply + 1,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 995 | <code>                        -alpha - 1,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 996 | <code>                        -alpha,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 997 | <code>                        true,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 998 | <code>                    )?;</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 999 | <code>                    if score &gt; alpha &amp;&amp; (reduce &#124;&#124; score &lt; beta) {</code> | Test the condition and execute this branch only when it matches. |
| 1000 | <code>                        score = -self.research(</code> | Update the named state using the expression and assignment operator shown. |
| 1001 | <code>                            &amp;child,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1002 | <code>                            depth - 1,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1003 | <code>                            ply + 1,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1004 | <code>                            -beta,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1005 | <code>                            -alpha,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1006 | <code>                            if reduce { Research::Lmr } else { Research::Pvs },</code> | Test the condition and execute this branch only when it matches. |
| 1007 | <code>                        )?;</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1008 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 1009 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1010 | <code>                if score &gt; best&#95;score {</code> | Test the condition and execute this branch only when it matches. |
| 1011 | <code>                    best&#95;score = score;</code> | Update the named state using the expression and assignment operator shown. |
| 1012 | <code>                    best = mv;</code> | Update the named state using the expression and assignment operator shown. |
| 1013 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1014 | <code>                if score &gt; alpha {</code> | Test the condition and execute this branch only when it matches. |
| 1015 | <code>                    alpha = score;</code> | Update the named state using the expression and assignment operator shown. |
| 1016 | <code>                    if ply == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 1017 | <code>                        self.best&#95;move = mv;</code> | Update the named state using the expression and assignment operator shown. |
| 1018 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 1019 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1020 | <code>                if legal&#95;count == 1</code> | Test the condition and execute this branch only when it matches. |
| 1021 | <code>                    &amp;&amp; score &lt; beta</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1022 | <code>                    &amp;&amp; let Some(result) = self.split&#95;remaining(</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1023 | <code>                        position,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1024 | <code>                        &amp;mut entries&#91;index + 1..&#93;,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1025 | <code>                        depth,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1026 | <code>                        ply,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1027 | <code>                        alpha,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1028 | <code>                        beta,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1029 | <code>                        best&#95;score,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1030 | <code>                        best,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1031 | <code>                        check,</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1032 | <code>                    )</code> | Close the preceding block, expression, call, or collection. |
| 1033 | <code>                {</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1034 | <code>                    let (split&#95;score, split&#95;best, count) = result?;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1035 | <code>                    best&#95;score = split&#95;score;</code> | Update the named state using the expression and assignment operator shown. |
| 1036 | <code>                    best = split&#95;best;</code> | Update the named state using the expression and assignment operator shown. |
| 1037 | <code>                    legal&#95;count = count;</code> | Update the named state using the expression and assignment operator shown. |
| 1038 | <code>                    if best&#95;score &gt; alpha &amp;&amp; ply == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 1039 | <code>                        self.best&#95;move = best;</code> | Update the named state using the expression and assignment operator shown. |
| 1040 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 1041 | <code>                    if best&#95;score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 1042 | <code>                        self.record&#95;cutoff(position, best, depth, ply);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1043 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 1044 | <code>                    break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1045 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1046 | <code>                if score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 1047 | <code>                    if quiet {</code> | Test the condition and execute this branch only when it matches. |
| 1048 | <code>                        let entry = &amp;mut self.quiet&#95;history&#91;usize::from(!white)&#93;&#91;move&#95;start(mv)&#93;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1049 | <code>                            &#91;move&#95;target(mv)&#93;;</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1050 | <code>                        &#42;entry = HISTORY&#95;LIMIT.min(&#42;entry + depth &#42; depth);</code> | Update the named state using the expression and assignment operator shown. |
| 1051 | <code>                        if self.killers&#91;ply&#93;&#91;0&#93; != mv {</code> | Test the condition and execute this branch only when it matches. |
| 1052 | <code>                            self.killers&#91;ply&#93; = &#91;mv, self.killers&#91;ply&#93;&#91;0&#93;&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 1053 | <code>                        }</code> | Close the preceding block, expression, call, or collection. |
| 1054 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 1055 | <code>                    break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1056 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1057 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1058 | <code>            Ok((best&#95;score, best, legal&#95;count))</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1059 | <code>        })();</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1060 | <code>        self.history.pop();</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1061 | <code>        let (best&#95;score, best, legal&#95;count) = result?;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1062 | <code>        if legal&#95;count == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 1063 | <code>            // Checkmate, worse the sooner it comes, or stalemate</code> | Source explanation: Checkmate, worse the sooner it comes, or stalemate |
| 1064 | <code>            return Ok(if check { -(MATE&#95;SCORE - ply as i32) } else { 0 });</code> | Leave the current function with the displayed result. |
| 1065 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1066 | <code>        let mut bound = EXACT;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1067 | <code>        if best&#95;score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 1068 | <code>            bound = LOWER;</code> | Update the named state using the expression and assignment operator shown. |
| 1069 | <code>        } else if best&#95;score &lt;= original&#95;alpha {</code> | Test the condition and execute this branch only when it matches. |
| 1070 | <code>            bound = UPPER;</code> | Update the named state using the expression and assignment operator shown. |
| 1071 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1072 | <code>        self.metrics.stores += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 1073 | <code>        let stored = self</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1074 | <code>            .table</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1075 | <code>            .store(key, depth, best&#95;score, bound, best, ply as i32);</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1076 | <code>        self.metrics.replacements += u64::from(stored.replaced);</code> | Update the named state using the expression and assignment operator shown. |
| 1077 | <code>        self.metrics.protected += u64::from(stored.protected);</code> | Update the named state using the expression and assignment operator shown. |
| 1078 | <code>        Ok(best&#95;score)</code> | Evaluate or continue this expression in alpha&#95;beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1079 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 1080 | <code></code> | Blank line separating related declarations or steps. |
| 1081 | <code>    fn record&#95;cutoff(&amp;mut self, position: &amp;Position, mv: Move, depth: i32, ply: usize) {</code> | Declare record&#95;cutoff(); its parameters and return type define the interface. |
| 1082 | <code>        if captured&#95;kind(position, mv) == EMPTY &amp;&amp; move&#95;promotion(mv) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 1083 | <code>            let entry = &amp;mut self.quiet&#95;history&#91;usize::from(!position.white)&#93;&#91;move&#95;start(mv)&#93;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1084 | <code>                &#91;move&#95;target(mv)&#93;;</code> | Evaluate or continue this expression in record&#95;cutoff(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1085 | <code>            &#42;entry = HISTORY&#95;LIMIT.min(&#42;entry + depth &#42; depth);</code> | Update the named state using the expression and assignment operator shown. |
| 1086 | <code>            if self.killers&#91;ply&#93;&#91;0&#93; != mv {</code> | Test the condition and execute this branch only when it matches. |
| 1087 | <code>                self.killers&#91;ply&#93; = &#91;mv, self.killers&#91;ply&#93;&#91;0&#93;&#93;;</code> | Update the named state using the expression and assignment operator shown. |
| 1088 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1089 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1090 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 1091 | <code></code> | Blank line separating related declarations or steps. |
| 1092 | <code>    /// Return the line of best play that the table holds, as UCI text.</code> | Source explanation: Return the line of best play that the table holds, as UCI text. |
| 1093 | <code>    fn principal&#95;variation(&amp;self, position: &amp;Position, depth: i32) -&gt; String {</code> | Declare principal&#95;variation(); its parameters and return type define the interface. |
| 1094 | <code>        let mut line = String::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1095 | <code>        let mut current = &#42;position;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1096 | <code>        for &#95; in 0..depth {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 1097 | <code>            let Some(stored) = self.table.probe(current.key, 0) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1098 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1099 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 1100 | <code>            let mut moves = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1101 | <code>            generate&#95;moves(&amp;current, &amp;mut moves);</code> | Evaluate or continue this expression in principal&#95;variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1102 | <code>            if !moves.as&#95;mut&#95;slice().contains(&amp;stored.mv)</code> | Test the condition and execute this branch only when it matches. |
| 1103 | <code>                &#124;&#124; in&#95;check(&amp;make&#95;move(&amp;current, stored.mv), current.white)</code> | Evaluate or continue this expression in principal&#95;variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1104 | <code>            {</code> | Evaluate or continue this expression in principal&#95;variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1105 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1106 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1107 | <code>            line.push(&#x27; &#x27;);</code> | Evaluate or continue this expression in principal&#95;variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1108 | <code>            line.push&#95;str(&amp;move&#95;text(stored.mv));</code> | Evaluate or continue this expression in principal&#95;variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1109 | <code>            current = make&#95;move(&amp;current, stored.mv);</code> | Update the named state using the expression and assignment operator shown. |
| 1110 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1111 | <code>        line</code> | Evaluate or continue this expression in principal&#95;variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1112 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 1113 | <code></code> | Blank line separating related declarations or steps. |
| 1114 | <code>    /// Return a legal cached reply after the selected move.</code> | Source explanation: Return a legal cached reply after the selected move. |
| 1115 | <code>    fn ponder&#95;reply(&amp;self, position: &amp;Position, mv: Move) -&gt; Move {</code> | Declare ponder&#95;reply(); its parameters and return type define the interface. |
| 1116 | <code>        let child = make&#95;move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1117 | <code>        if let Some(stored) = self.table.probe(child.key, 0) {</code> | Test the condition and execute this branch only when it matches. |
| 1118 | <code>            let mut moves = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1119 | <code>            generate&#95;moves(&amp;child, &amp;mut moves);</code> | Evaluate or continue this expression in ponder&#95;reply(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1120 | <code>            for &amp;mut reply in moves.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 1121 | <code>                if reply == stored.mv &amp;&amp; !in&#95;check(&amp;make&#95;move(&amp;child, reply), child.white) {</code> | Test the condition and execute this branch only when it matches. |
| 1122 | <code>                    return reply;</code> | Leave the current function with the displayed result. |
| 1123 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1124 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1125 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1126 | <code>        NO&#95;MOVE</code> | Evaluate or continue this expression in ponder&#95;reply(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1127 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 1128 | <code></code> | Blank line separating related declarations or steps. |
| 1129 | <code>    /// Search one move deeper at a time and return the best move, or NO&#95;MOVE when there</code> | Source explanation: Search one move deeper at a time and return the best move, or NO&#95;MOVE when there |
| 1130 | <code>    /// is no legal move. The search plans to use seconds, takes longer when the best</code> | Source explanation: is no legal move. The search plans to use seconds, takes longer when the best |
| 1131 | <code>    /// move keeps changing, and never runs past longest. A depth limit replaces the</code> | Source explanation: move keeps changing, and never runs past longest. A depth limit replaces the |
| 1132 | <code>    /// clock, for the workshop designer&#x27;s checks.</code> | Source explanation: clock, for the workshop designer&#x27;s checks. |
| 1133 | <code>    fn think(</code> | Declare think(); its parameters and return type define the interface. |
| 1134 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1135 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 1136 | <code>        game&#95;history: &amp;&#91;u64&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 1137 | <code>        mut seconds: f64,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1138 | <code>        longest: f64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 1139 | <code>        depth&#95;limit: Option&lt;i32&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 1140 | <code>    ) -&gt; Move {</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1141 | <code>        self.begin&#95;search();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1142 | <code>        let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1143 | <code>        let mut completed&#95;move = NO&#95;MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1144 | <code>        let mut stdout = std::io::stdout();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1145 | <code>        &#x27;deepening: for depth in 1..=MAX&#95;DEPTH {</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1146 | <code>            let total = self.reported&#95;nodes();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1147 | <code>            if self.main&#95;control().poll(total) {</code> | Test the condition and execute this branch only when it matches. |
| 1148 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1149 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1150 | <code>            let previous = self.best&#95;move;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1151 | <code>            let (mut alpha, mut beta) = (-MATE&#95;SCORE - 1, MATE&#95;SCORE + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1152 | <code>            if depth &gt;= 4 {</code> | Test the condition and execute this branch only when it matches. |
| 1153 | <code>                alpha = score - ASPIRATION&#95;WINDOW;</code> | Update the named state using the expression and assignment operator shown. |
| 1154 | <code>                beta = score + ASPIRATION&#95;WINDOW;</code> | Update the named state using the expression and assignment operator shown. |
| 1155 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1156 | <code>            let mut retry = false;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1157 | <code>            loop {</code> | Repeat until an explicit break, return, or other control transfer ends the loop. |
| 1158 | <code>                self.history.clear();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1159 | <code>                self.history.extend&#95;from&#95;slice(game&#95;history);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1160 | <code>                let attempt&#95;nodes = self.nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1161 | <code>                let stopped&#95;nodes = self.work.stop&#95;discarded&#95;nodes;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1162 | <code>                let result = if retry {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1163 | <code>                    self.research(position, depth, 0, alpha, beta, Research::Aspiration)</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1164 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 1165 | <code>                    self.alpha&#95;beta(position, depth, 0, alpha, beta, true)</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1166 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 1167 | <code>                match result {</code> | Select an arm by matching the expression against the listed patterns. |
| 1168 | <code>                    Ok(found) =&gt; score = found,</code> | Handle this matching pattern with the expression or block on its right. |
| 1169 | <code>                    Err(SearchTimeout) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 1170 | <code>                        self.work.stop&#95;discarded&#95;nodes += (self.nodes - attempt&#95;nodes)</code> | Update the named state using the expression and assignment operator shown. |
| 1171 | <code>                            .saturating&#95;sub(self.work.stop&#95;discarded&#95;nodes - stopped&#95;nodes);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1172 | <code>                        break &#x27;deepening;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1173 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 1174 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1175 | <code>                // Outside the window: search again with that side open</code> | Source explanation: Outside the window: search again with that side open |
| 1176 | <code>                if score &lt;= alpha {</code> | Test the condition and execute this branch only when it matches. |
| 1177 | <code>                    alpha = -MATE&#95;SCORE - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 1178 | <code>                    retry = true;</code> | Update the named state using the expression and assignment operator shown. |
| 1179 | <code>                } else if score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 1180 | <code>                    beta = MATE&#95;SCORE + 1;</code> | Update the named state using the expression and assignment operator shown. |
| 1181 | <code>                    retry = true;</code> | Update the named state using the expression and assignment operator shown. |
| 1182 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 1183 | <code>                    break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1184 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 1185 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1186 | <code>            if self.best&#95;move == NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 1187 | <code>                break; // No legal move</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1188 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1189 | <code>            completed&#95;move = self.best&#95;move;</code> | Update the named state using the expression and assignment operator shown. |
| 1190 | <code>            let line = self.principal&#95;variation(position, depth);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1191 | <code>            let info = format!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1192 | <code>                &quot;info depth {depth} score cp {score} nodes {} pv{line}&quot;,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1193 | <code>                self.reported&#95;nodes()</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1194 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 1195 | <code>            let &#95; = writeln!(stdout, &quot;{info}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1196 | <code>            let &#95; = stdout.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1197 | <code>            // A best move that changes needs more time to settle</code> | Source explanation: A best move that changes needs more time to settle |
| 1198 | <code>            let changed = previous != NO&#95;MOVE &amp;&amp; self.best&#95;move != previous;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1199 | <code>            if changed {</code> | Test the condition and execute this branch only when it matches. |
| 1200 | <code>                seconds = (seconds &#42; 1.5).min(longest);</code> | Update the named state using the expression and assignment operator shown. |
| 1201 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1202 | <code>            self.main&#95;control()</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1203 | <code>                .completed(depth, seconds / 2.0, score, changed);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1204 | <code>            self.main&#95;control().log&#95;search(&amp;info);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1205 | <code>            if score.abs() &gt; MATE&#95;SCORE - 100 {</code> | Test the condition and execute this branch only when it matches. |
| 1206 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 1207 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1208 | <code>            match depth&#95;limit {</code> | Select an arm by matching the expression against the listed patterns. |
| 1209 | <code>                Some(limit) if depth &gt;= limit =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 1210 | <code>                None if self.main&#95;control().soft&#95;finished(seconds / 2.0) =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 1211 | <code>                &#95; =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 1212 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 1213 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1214 | <code>        let total = self.reported&#95;nodes();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1215 | <code>        let control = self.main&#95;control();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 1216 | <code>        control.nodes = total;</code> | Update the named state using the expression and assignment operator shown. |
| 1217 | <code>        if completed&#95;move != NO&#95;MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 1218 | <code>            self.best&#95;move = completed&#95;move;</code> | Update the named state using the expression and assignment operator shown. |
| 1219 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 1220 | <code>        self.best&#95;move</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 1221 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 1222 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### search_control.rs

Search cancellation, time budgets, ponder handoffs, and command coordination.

[Open source](src/search_control.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Read UCI commands during search; only the search thread owns chess state.</code> | Source explanation: Read UCI commands during search; only the search thread owns chess state. |
| 2 | <code></code> | Blank line separating related declarations or steps. |
| 3 | <code>use std::collections::VecDeque;</code> | Bring the named items into scope; no chess search runs here. |
| 4 | <code>use std::io::{BufRead, Write};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use std::sync::atomic::{AtomicBool, Ordering::Relaxed};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use std::sync::mpsc::{self, Receiver};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code>use std::sync::{Mutex, OnceLock};</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code>use std::time::{Duration, Instant, SystemTime, UNIX&#95;EPOCH};</code> | Bring the named items into scope; no chess search runs here. |
| 9 | <code></code> | Blank line separating related declarations or steps. |
| 10 | <code>#&#91;derive(Clone, Copy, PartialEq)&#93;</code> | Compiler attribute controlling the following declaration. |
| 11 | <code>pub enum Mode {</code> | Declare the enum Mode used by this module. |
| 12 | <code>    Normal,</code> | Continue the enclosing declaration or initialization expression. |
| 13 | <code>    Ponder,</code> | Continue the enclosing declaration or initialization expression. |
| 14 | <code>    Infinite,</code> | Continue the enclosing declaration or initialization expression. |
| 15 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 16 | <code></code> | Blank line separating related declarations or steps. |
| 17 | <code>const SHALLOW&#95;DEPTH: i32 = 8; // Below this completed depth, a hit deserves another look</code> | Define SHALLOW&#95;DEPTH, including its explicit type and initial value. |
| 18 | <code>const SCORE&#95;DROP: i32 = 35; // Centipawns lost between mature completed iterations</code> | Define SCORE&#95;DROP, including its explicit type and initial value. |
| 19 | <code>const MATURE&#95;DEPTH: i32 = 4; // Scores of shallower iterations swing too much to judge</code> | Define MATURE&#95;DEPTH, including its explicit type and initial value. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>/// What an explicitly stopped native ponder search hands to a matching go:</code> | Source explanation: What an explicitly stopped native ponder search hands to a matching go: |
| 22 | <code>/// its elapsed duration, completed depth, mature score, and signs of doubt.</code> | Source explanation: its elapsed duration, completed depth, mature score, and signs of doubt. |
| 23 | <code>#&#91;derive(Clone, Copy)&#93;</code> | Compiler attribute controlling the following declaration. |
| 24 | <code>pub struct PonderCredit {</code> | Declare the struct PonderCredit used by this module. |
| 25 | <code>    pub seconds: Duration,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 26 | <code>    pub depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 27 | <code>    pub score: Option&lt;i32&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 28 | <code>    pub deteriorating: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 29 | <code>    pub unstable: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 30 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 31 | <code></code> | Blank line separating related declarations or steps. |
| 32 | <code>pub static LOG&#95;ENABLED: AtomicBool = AtomicBool::new(false); // PonderDiagnostics controls output</code> | Define LOG&#95;ENABLED, including its explicit type and initial value. |
| 33 | <code>pub static SEARCH&#95;LOG&#95;ENABLED: AtomicBool = AtomicBool::new(false); // SearchDiagnostics controls output</code> | Define SEARCH&#95;LOG&#95;ENABLED, including its explicit type and initial value. |
| 34 | <code>static LOG&#95;START: OnceLock&lt;Instant&gt; = OnceLock::new();</code> | Define LOG&#95;START, including its explicit type and initial value. |
| 35 | <code>static LOG&#95;FILE: Mutex&lt;()&gt; = Mutex::new(()); // One thread writes a line at a time</code> | Define LOG&#95;FILE, including its explicit type and initial value. |
| 36 | <code></code> | Blank line separating related declarations or steps. |
| 37 | <code>/// Keep both diagnostic streams in one process session, independent of the GUI&#x27;s folder.</code> | Source explanation: Keep both diagnostic streams in one process session, independent of the GUI&#x27;s folder. |
| 38 | <code>fn log&#95;path(kind: &amp;str) -&gt; std::io::Result&lt;std::path::PathBuf&gt; {</code> | Declare log&#95;path(); its parameters and return type define the interface. |
| 39 | <code>    static SESSION: OnceLock&lt;u128&gt; = OnceLock::new();</code> | Define SESSION, including its explicit type and initial value. |
| 40 | <code>    let session = SESSION.get&#95;or&#95;init(&#124;&#124; {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 41 | <code>        SystemTime::now()</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 42 | <code>            .duration&#95;since(UNIX&#95;EPOCH)</code> | Evaluate or continue this expression in log&#95;path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 43 | <code>            .unwrap&#95;or&#95;default()</code> | Evaluate or continue this expression in log&#95;path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 44 | <code>            .as&#95;nanos()</code> | Evaluate or continue this expression in log&#95;path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 45 | <code>    });</code> | Evaluate or continue this expression in log&#95;path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code>    let folder = std::env::current&#95;exe()?.with&#95;file&#95;name(&quot;logs&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>    std::fs::create&#95;dir&#95;all(&amp;folder)?;</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 48 | <code>    Ok(folder.join(format!(&quot;{kind}&#95;{}&#95;{session}.txt&quot;, std::process::id())))</code> | Evaluate or continue this expression in log&#95;path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 49 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 50 | <code></code> | Blank line separating related declarations or steps. |
| 51 | <code>/// Append one line to this process&#x27;s ponder log while PonderDiagnostics</code> | Source explanation: Append one line to this process&#x27;s ponder log while PonderDiagnostics |
| 52 | <code>/// is on: the process id, milliseconds since the engine started, and the text.</code> | Source explanation: is on: the process id, milliseconds since the engine started, and the text. |
| 53 | <code>pub fn log(text: &amp;str) {</code> | Declare log(); its parameters and return type define the interface. |
| 54 | <code>    if !LOG&#95;ENABLED.load(Relaxed) {</code> | Test the condition and execute this branch only when it matches. |
| 55 | <code>        return;</code> | Leave the current function with the displayed result. |
| 56 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 57 | <code>    append&#95;log(&quot;ponder&quot;, text);</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 58 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 59 | <code></code> | Blank line separating related declarations or steps. |
| 60 | <code>/// Save search commands and reports independently of ponder logging.</code> | Source explanation: Save search commands and reports independently of ponder logging. |
| 61 | <code>pub fn search&#95;log(text: &amp;str) {</code> | Declare search&#95;log(); its parameters and return type define the interface. |
| 62 | <code>    if SEARCH&#95;LOG&#95;ENABLED.load(Relaxed) {</code> | Test the condition and execute this branch only when it matches. |
| 63 | <code>        let unix&#95;ms = SystemTime::now()</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 64 | <code>            .duration&#95;since(UNIX&#95;EPOCH)</code> | Evaluate or continue this expression in search&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>            .unwrap&#95;or&#95;default()</code> | Evaluate or continue this expression in search&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 66 | <code>            .as&#95;millis();</code> | Evaluate or continue this expression in search&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>        append&#95;log(&quot;search&quot;, &amp;format!(&quot;unix&#95;ms={unix&#95;ms} {text}&quot;));</code> | Evaluate or continue this expression in search&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 69 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 70 | <code></code> | Blank line separating related declarations or steps. |
| 71 | <code>/// Append beside the executable; unavailable diagnostic files never stop play.</code> | Source explanation: Append beside the executable; unavailable diagnostic files never stop play. |
| 72 | <code>fn append&#95;log(kind: &amp;str, text: &amp;str) {</code> | Declare append&#95;log(); its parameters and return type define the interface. |
| 73 | <code>    let start = &#42;LOG&#95;START.get&#95;or&#95;init(Instant::now);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 74 | <code>    let Ok(path) = log&#95;path(kind) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 75 | <code>        return;</code> | Leave the current function with the displayed result. |
| 76 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 77 | <code>    let &#95;lock = LOG&#95;FILE.lock();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 78 | <code>    if let Ok(mut file) = std::fs::OpenOptions::new()</code> | Test the condition and execute this branch only when it matches. |
| 79 | <code>        .create(true)</code> | Evaluate or continue this expression in append&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 80 | <code>        .append(true)</code> | Evaluate or continue this expression in append&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>        .open(path)</code> | Evaluate or continue this expression in append&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 82 | <code>    {</code> | Evaluate or continue this expression in append&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 83 | <code>        let ms = start.elapsed().as&#95;secs&#95;f64() &#42; 1000.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 84 | <code>        let line = format!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 85 | <code>            &quot;{} {ms:&gt;11.1} {text}</code> | Evaluate or continue this expression in append&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 86 | <code>&quot;,</code> | Evaluate or continue this expression in append&#95;log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 87 | <code>            std::process::id()</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 88 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 89 | <code>        let &#95; = file.write&#95;all(line.as&#95;bytes());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 90 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 91 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 92 | <code></code> | Blank line separating related declarations or steps. |
| 93 | <code>pub struct SearchControl {</code> | Declare the struct SearchControl used by this module. |
| 94 | <code>    commands: Receiver&lt;String&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 95 | <code>    pending: VecDeque&lt;String&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 96 | <code>    pub mode: Mode,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 97 | <code>    pub stopped: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 98 | <code>    stop&#95;received: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 99 | <code>    hit&#95;kind: &amp;&#x27;static str,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 100 | <code>    started: Instant,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 101 | <code>    longest: f64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 102 | <code>    search&#95;started: Instant,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 103 | <code>    pub was&#95;ponder: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 104 | <code>    hit&#95;time: Option&lt;Instant&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 105 | <code>    pub nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 106 | <code>    completed&#95;depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 107 | <code>    hit&#95;nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 108 | <code>    hit&#95;depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 109 | <code>    soft&#95;limit: f64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 110 | <code>    pub can&#95;reinvest: bool, // The clock holds a reserve for extra time after a hit</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 111 | <code>    pub source: &amp;&#x27;static str, // &quot;native&quot;, &quot;book&quot;, or &quot;tablebase&quot;</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 112 | <code>    score: Option&lt;i32&gt;,     // The score of the deepest mature iteration</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 113 | <code>    score&#95;depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 114 | <code>    deteriorating: bool, // The score fell by SCORE&#95;DROP between mature iterations</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 115 | <code>    unstable: bool,      // The best move changed in the latest mature iteration</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 116 | <code>    uncertain&#95;hit: bool, // The hit came on a shallow, falling, or changing result</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 117 | <code>    pub hit&#95;min&#95;percent: u32, // Minimum share of the plan spent after a ponder hit</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 118 | <code>    credit&#95;depth: i32,   // The soft target applies once this depth is complete</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 119 | <code>    search&#95;id: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 120 | <code>    diagnostics&#95;started: Instant, // This invocation, excluding credited earlier ponder work</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 121 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 122 | <code></code> | Blank line separating related declarations or steps. |
| 123 | <code>impl SearchControl {</code> | Begin the implementation of methods or a trait for the named type. |
| 124 | <code>    pub fn new() -&gt; Self {</code> | Declare new(); its parameters and return type define the interface. |
| 125 | <code>        LOG&#95;START.get&#95;or&#95;init(Instant::now);</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 126 | <code>        let (sender, commands) = mpsc::channel();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 127 | <code>        std::thread::spawn(move &#124;&#124; {</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 128 | <code>            for line in std::io::stdin().lock().lines() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 129 | <code>                let Ok(line) = line else { break };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 130 | <code>                log(&amp;format!(&quot;&lt; {line}&quot;));</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 131 | <code>                search&#95;log(&amp;format!(&quot;&lt; {line}&quot;));</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 132 | <code>                if sender.send(line).is&#95;err() {</code> | Test the condition and execute this branch only when it matches. |
| 133 | <code>                    return;</code> | Leave the current function with the displayed result. |
| 134 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 135 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 136 | <code>            let &#95; = sender.send(&quot;quit&quot;.to&#95;string());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 137 | <code>        });</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 138 | <code>        Self {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 139 | <code>            commands,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 140 | <code>            pending: VecDeque::new(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 141 | <code>            mode: Mode::Normal,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 142 | <code>            stopped: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 143 | <code>            stop&#95;received: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 144 | <code>            hit&#95;kind: &quot;none&quot;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 145 | <code>            started: Instant::now(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 146 | <code>            longest: f64::INFINITY,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 147 | <code>            search&#95;started: Instant::now(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 148 | <code>            was&#95;ponder: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 149 | <code>            hit&#95;time: None,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 150 | <code>            nodes: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 151 | <code>            completed&#95;depth: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 152 | <code>            hit&#95;nodes: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 153 | <code>            hit&#95;depth: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 154 | <code>            soft&#95;limit: f64::INFINITY,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 155 | <code>            can&#95;reinvest: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 156 | <code>            source: &quot;native&quot;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 157 | <code>            score: None,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 158 | <code>            score&#95;depth: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 159 | <code>            deteriorating: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 160 | <code>            unstable: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 161 | <code>            uncertain&#95;hit: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 162 | <code>            hit&#95;min&#95;percent: 25,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 163 | <code>            credit&#95;depth: 1,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 164 | <code>            search&#95;id: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 165 | <code>            diagnostics&#95;started: Instant::now(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 166 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 167 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 168 | <code></code> | Blank line separating related declarations or steps. |
| 169 | <code>    pub fn next&#95;command(&amp;mut self) -&gt; String {</code> | Declare next&#95;command(); its parameters and return type define the interface. |
| 170 | <code>        self.pending</code> | Evaluate or continue this expression in next&#95;command(); a final expression without a semicolon supplies its block&#x27;s value. |
| 171 | <code>            .pop&#95;front()</code> | Evaluate or continue this expression in next&#95;command(); a final expression without a semicolon supplies its block&#x27;s value. |
| 172 | <code>            .unwrap&#95;or&#95;else(&#124;&#124; self.commands.recv().unwrap&#95;or&#95;else(&#124;&#95;&#124; &quot;quit&quot;.to&#95;string()))</code> | Evaluate or continue this expression in next&#95;command(); a final expression without a semicolon supplies its block&#x27;s value. |
| 173 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 174 | <code></code> | Blank line separating related declarations or steps. |
| 175 | <code>    pub fn begin(&amp;mut self, mode: Mode, longest: f64) {</code> | Declare begin(); its parameters and return type define the interface. |
| 176 | <code>        self.mode = mode;</code> | Update the named state using the expression and assignment operator shown. |
| 177 | <code>        self.stopped = false;</code> | Update the named state using the expression and assignment operator shown. |
| 178 | <code>        self.stop&#95;received = false;</code> | Update the named state using the expression and assignment operator shown. |
| 179 | <code>        self.hit&#95;kind = &quot;none&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 180 | <code>        self.started = Instant::now();</code> | Update the named state using the expression and assignment operator shown. |
| 181 | <code>        self.search&#95;id += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 182 | <code>        self.diagnostics&#95;started = self.started;</code> | Update the named state using the expression and assignment operator shown. |
| 183 | <code>        self.longest = longest;</code> | Update the named state using the expression and assignment operator shown. |
| 184 | <code>        self.search&#95;started = self.started;</code> | Update the named state using the expression and assignment operator shown. |
| 185 | <code>        self.was&#95;ponder = mode == Mode::Ponder;</code> | Update the named state using the expression and assignment operator shown. |
| 186 | <code>        self.hit&#95;time = None;</code> | Update the named state using the expression and assignment operator shown. |
| 187 | <code>        self.nodes = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 188 | <code>        self.completed&#95;depth = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 189 | <code>        self.hit&#95;nodes = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 190 | <code>        self.hit&#95;depth = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 191 | <code>        self.soft&#95;limit = f64::INFINITY;</code> | Update the named state using the expression and assignment operator shown. |
| 192 | <code>        self.can&#95;reinvest = false;</code> | Update the named state using the expression and assignment operator shown. |
| 193 | <code>        self.source = &quot;native&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 194 | <code>        self.score = None;</code> | Update the named state using the expression and assignment operator shown. |
| 195 | <code>        self.score&#95;depth = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 196 | <code>        self.deteriorating = false;</code> | Update the named state using the expression and assignment operator shown. |
| 197 | <code>        self.unstable = false;</code> | Update the named state using the expression and assignment operator shown. |
| 198 | <code>        self.uncertain&#95;hit = false;</code> | Update the named state using the expression and assignment operator shown. |
| 199 | <code>        self.credit&#95;depth = 1;</code> | Update the named state using the expression and assignment operator shown. |
| 200 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 201 | <code></code> | Blank line separating related declarations or steps. |
| 202 | <code>    /// Tag reports with their search and separate wall time from own-clock time.</code> | Source explanation: Tag reports with their search and separate wall time from own-clock time. |
| 203 | <code>    pub fn log&#95;search(&amp;self, text: &amp;str) {</code> | Declare log&#95;search(); its parameters and return type define the interface. |
| 204 | <code>        if SEARCH&#95;LOG&#95;ENABLED.load(Relaxed) {</code> | Test the condition and execute this branch only when it matches. |
| 205 | <code>            search&#95;log(&amp;format!(</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 206 | <code>                &quot;search={} wall&#95;ms={:.1} own&#95;ms={:.1} completed&#95;depth={} hit&#95;kind={} {text}&quot;,</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 207 | <code>                self.search&#95;id,</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 208 | <code>                self.diagnostics&#95;started.elapsed().as&#95;secs&#95;f64() &#42; 1000.0,</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 209 | <code>                self.elapsed() &#42; 1000.0,</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 210 | <code>                self.completed&#95;depth,</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 211 | <code>                self.hit&#95;kind</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 212 | <code>            ));</code> | Evaluate or continue this expression in log&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 213 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 214 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 215 | <code></code> | Blank line separating related declarations or steps. |
| 216 | <code>    fn handle(&amp;mut self, line: String) {</code> | Declare handle(); its parameters and return type define the interface. |
| 217 | <code>        match line.split&#95;whitespace().next() {</code> | Select an arm by matching the expression against the listed patterns. |
| 218 | <code>            Some(&quot;isready&quot;) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 219 | <code>                let mut out = std::io::stdout();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 220 | <code>                let &#95; = writeln!(out, &quot;readyok&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 221 | <code>                let &#95; = out.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 222 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code>            Some(&quot;ponderhit&quot;) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 224 | <code>                if self.mode == Mode::Ponder {</code> | Test the condition and execute this branch only when it matches. |
| 225 | <code>                    self.mode = Mode::Normal;</code> | Update the named state using the expression and assignment operator shown. |
| 226 | <code>                    self.started = Instant::now();</code> | Update the named state using the expression and assignment operator shown. |
| 227 | <code>                    self.hit&#95;time = Some(self.started);</code> | Update the named state using the expression and assignment operator shown. |
| 228 | <code>                    self.hit&#95;kind = &quot;uci&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 229 | <code>                    self.hit&#95;nodes = self.nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 230 | <code>                    self.hit&#95;depth = self.completed&#95;depth;</code> | Update the named state using the expression and assignment operator shown. |
| 231 | <code>                    self.uncertain&#95;hit =</code> | Evaluate or continue this expression in handle(); a final expression without a semicolon supplies its block&#x27;s value. |
| 232 | <code>                        self.hit&#95;depth &lt; SHALLOW&#95;DEPTH &#124;&#124; self.deteriorating &#124;&#124; self.unstable;</code> | Evaluate or continue this expression in handle(); a final expression without a semicolon supplies its block&#x27;s value. |
| 233 | <code>                    self.log&#95;search(&amp;format!(&quot;ponderhit hit&#95;nodes={}&quot;, self.hit&#95;nodes));</code> | Evaluate or continue this expression in handle(); a final expression without a semicolon supplies its block&#x27;s value. |
| 234 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 235 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 236 | <code>            Some(&quot;stop&quot;) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 237 | <code>                self.stopped = true;</code> | Update the named state using the expression and assignment operator shown. |
| 238 | <code>                self.stop&#95;received = true;</code> | Update the named state using the expression and assignment operator shown. |
| 239 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 240 | <code>            Some(&quot;position&quot; &#124; &quot;go&quot; &#124; &quot;setoption&quot; &#124; &quot;ucinewgame&quot; &#124; &quot;quit&quot;) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 241 | <code>                self.pending.push&#95;back(line);</code> | Evaluate or continue this expression in handle(); a final expression without a semicolon supplies its block&#x27;s value. |
| 242 | <code>                self.stopped = true;</code> | Update the named state using the expression and assignment operator shown. |
| 243 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 244 | <code>            &#95; =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 245 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 246 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 247 | <code></code> | Blank line separating related declarations or steps. |
| 248 | <code>    pub fn poll(&amp;mut self, nodes: u64) -&gt; bool {</code> | Declare poll(); its parameters and return type define the interface. |
| 249 | <code>        self.nodes = nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 250 | <code>        while !self.stopped {</code> | Repeat this block while its condition remains true. |
| 251 | <code>            let Ok(line) = self.commands.try&#95;recv() else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 252 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 253 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 254 | <code>            self.handle(line);</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 255 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 256 | <code>        self.stopped</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 257 | <code>            &#124;&#124; (self.mode == Mode::Normal</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 258 | <code>                &amp;&amp; (self.elapsed() &gt;= self.longest</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 259 | <code>                    &#124;&#124; (self.was&#95;ponder</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 260 | <code>                        &amp;&amp; self.completed&#95;depth &gt;= self.credit&#95;depth</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 261 | <code>                        &amp;&amp; self.search&#95;started.elapsed().as&#95;secs&#95;f64() &gt;= self.soft&#95;limit</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 262 | <code>                        &amp;&amp; self.hit&#95;finished())))</code> | Evaluate or continue this expression in poll(); a final expression without a semicolon supplies its block&#x27;s value. |
| 263 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 264 | <code></code> | Blank line separating related declarations or steps. |
| 265 | <code>    /// The soft target is half the move plan. Reserve a fraction of that full plan</code> | Source explanation: The soft target is half the move plan. Reserve a fraction of that full plan |
| 266 | <code>    /// on our clock, while the independent hard deadline always takes precedence.</code> | Source explanation: on our clock, while the independent hard deadline always takes precedence. |
| 267 | <code>    fn hit&#95;minimum(&amp;self) -&gt; f64 {</code> | Declare hit&#95;minimum(); its parameters and return type define the interface. |
| 268 | <code>        let fraction = f64::from(self.hit&#95;min&#95;percent) / 100.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 269 | <code>        if fraction == 0.0 {</code> | Test the condition and execute this branch only when it matches. |
| 270 | <code>            return 0.0;</code> | Leave the current function with the displayed result. |
| 271 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 272 | <code>        (2.0 &#42; self.soft&#95;limit &#42; fraction).min(self.longest &#42; fraction)</code> | Evaluate or continue this expression in hit&#95;minimum(); a final expression without a semicolon supplies its block&#x27;s value. |
| 273 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 274 | <code></code> | Blank line separating related declarations or steps. |
| 275 | <code>    /// Return true when a hit earns extra time: the clock holds a reserve and the</code> | Source explanation: Return true when a hit earns extra time: the clock holds a reserve and the |
| 276 | <code>    /// result is shallow, falling, or changing.</code> | Source explanation: result is shallow, falling, or changing. |
| 277 | <code>    fn reinvest(&amp;self) -&gt; bool {</code> | Declare reinvest(); its parameters and return type define the interface. |
| 278 | <code>        self.can&#95;reinvest &amp;&amp; (self.uncertain&#95;hit &#124;&#124; self.deteriorating &#124;&#124; self.unstable)</code> | Evaluate or continue this expression in reinvest(); a final expression without a semicolon supplies its block&#x27;s value. |
| 279 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 280 | <code></code> | Blank line separating related declarations or steps. |
| 281 | <code>    /// Return true when the search has spent enough of its own clock after a hit. A</code> | Source explanation: Return true when the search has spent enough of its own clock after a hit. A |
| 282 | <code>    /// stable hit needs the ordinary reserve. An uncertain hit reserves half the plan</code> | Source explanation: stable hit needs the ordinary reserve. An uncertain hit reserves half the plan |
| 283 | <code>    /// and aims for one more completed depth, within one full plan and half the hard</code> | Source explanation: and aims for one more completed depth, within one full plan and half the hard |
| 284 | <code>    /// budget.</code> | Source explanation: budget. |
| 285 | <code>    fn hit&#95;finished(&amp;self) -&gt; bool {</code> | Declare hit&#95;finished(); its parameters and return type define the interface. |
| 286 | <code>        if !self.reinvest() {</code> | Test the condition and execute this branch only when it matches. |
| 287 | <code>            return self.elapsed() &gt;= self.hit&#95;minimum();</code> | Leave the current function with the displayed result. |
| 288 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 289 | <code>        let minimum = self</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 290 | <code>            .soft&#95;limit</code> | Evaluate or continue this expression in hit&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 291 | <code>            .min(self.longest / 2.0)</code> | Evaluate or continue this expression in hit&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 292 | <code>            .max(self.hit&#95;minimum());</code> | Evaluate or continue this expression in hit&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 293 | <code>        let limit = (2.0 &#42; self.soft&#95;limit).min(self.longest / 2.0);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 294 | <code>        self.elapsed() &gt;= minimum</code> | Evaluate or continue this expression in hit&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 295 | <code>            &amp;&amp; (self.completed&#95;depth &gt; self.hit&#95;depth &#124;&#124; self.elapsed() &gt;= limit)</code> | Evaluate or continue this expression in hit&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 296 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 297 | <code></code> | Blank line separating related declarations or steps. |
| 298 | <code>    /// Apply the soft limit without bypassing the thinking time a hit reserves.</code> | Source explanation: Apply the soft limit without bypassing the thinking time a hit reserves. |
| 299 | <code>    pub fn soft&#95;finished(&amp;self, target: f64) -&gt; bool {</code> | Declare soft&#95;finished(); its parameters and return type define the interface. |
| 300 | <code>        self.elapsed() &gt; target</code> | Evaluate or continue this expression in soft&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 301 | <code>            &amp;&amp; (!self.was&#95;ponder</code> | Evaluate or continue this expression in soft&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 302 | <code>                &#124;&#124; (self.completed&#95;depth &gt;= self.credit&#95;depth &amp;&amp; self.hit&#95;finished()))</code> | Evaluate or continue this expression in soft&#95;finished(); a final expression without a semicolon supplies its block&#x27;s value. |
| 303 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 304 | <code></code> | Blank line separating related declarations or steps. |
| 305 | <code>    pub fn wait(&amp;mut self) {</code> | Declare wait(); its parameters and return type define the interface. |
| 306 | <code>        while self.mode != Mode::Normal &amp;&amp; !self.stopped {</code> | Repeat this block while its condition remains true. |
| 307 | <code>            let line = self.commands.recv().unwrap&#95;or&#95;else(&#124;&#95;&#124; &quot;quit&quot;.to&#95;string());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 308 | <code>            self.handle(line);</code> | Evaluate or continue this expression in wait(); a final expression without a semicolon supplies its block&#x27;s value. |
| 309 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 310 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 311 | <code></code> | Blank line separating related declarations or steps. |
| 312 | <code>    pub fn elapsed(&amp;self) -&gt; f64 {</code> | Declare elapsed(); its parameters and return type define the interface. |
| 313 | <code>        if self.mode == Mode::Normal {</code> | Test the condition and execute this branch only when it matches. |
| 314 | <code>            self.started.elapsed().as&#95;secs&#95;f64()</code> | Evaluate or continue this expression in elapsed(); a final expression without a semicolon supplies its block&#x27;s value. |
| 315 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 316 | <code>            0.0</code> | Evaluate or continue this expression in elapsed(); a final expression without a semicolon supplies its block&#x27;s value. |
| 317 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 318 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 319 | <code></code> | Blank line separating related declarations or steps. |
| 320 | <code>    /// Remember native ponder work only after an explicit stop without a hit.</code> | Source explanation: Remember native ponder work only after an explicit stop without a hit. |
| 321 | <code>    /// Book waiting and cancellation by replacement commands earn no credit.</code> | Source explanation: Book waiting and cancellation by replacement commands earn no credit. |
| 322 | <code>    pub fn ponder&#95;credit(&amp;self) -&gt; Option&lt;PonderCredit&gt; {</code> | Declare ponder&#95;credit(); its parameters and return type define the interface. |
| 323 | <code>        if self.was&#95;ponder</code> | Test the condition and execute this branch only when it matches. |
| 324 | <code>            &amp;&amp; self.hit&#95;time.is&#95;none()</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 325 | <code>            &amp;&amp; self.stop&#95;received</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 326 | <code>            &amp;&amp; self.source == &quot;native&quot;</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 327 | <code>        {</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 328 | <code>            Some(PonderCredit {</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 329 | <code>                seconds: self.search&#95;started.elapsed(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 330 | <code>                depth: self.completed&#95;depth,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 331 | <code>                score: self.score,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 332 | <code>                deteriorating: self.deteriorating,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 333 | <code>                unstable: self.unstable,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 334 | <code>            })</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 335 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 336 | <code>            None</code> | Evaluate or continue this expression in ponder&#95;credit(); a final expression without a semicolon supplies its block&#x27;s value. |
| 337 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 338 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 339 | <code></code> | Blank line separating related declarations or steps. |
| 340 | <code>    /// Treat this search as a ponder hit: some GUIs, Lucas Chess among them, answer a</code> | Source explanation: Treat this search as a ponder hit: some GUIs, Lucas Chess among them, answer a |
| 341 | <code>    /// correct prediction with stop and a new go instead of ponderhit. The time spent</code> | Source explanation: correct prediction with stop and a new go instead of ponderhit. The time spent |
| 342 | <code>    /// pondering excludes the inter-command gap and counts toward the soft target once the new</code> | Source explanation: pondering excludes the inter-command gap and counts toward the soft target once the new |
| 343 | <code>    /// search, helped by the table, completes the depth the ponder search reached.</code> | Source explanation: search, helped by the table, completes the depth the ponder search reached. |
| 344 | <code>    /// The score and the signs of doubt of that search carry over.</code> | Source explanation: The score and the signs of doubt of that search carry over. |
| 345 | <code>    pub fn credit&#95;ponder(&amp;mut self, credit: PonderCredit) {</code> | Declare credit&#95;ponder(); its parameters and return type define the interface. |
| 346 | <code>        self.was&#95;ponder = true;</code> | Update the named state using the expression and assignment operator shown. |
| 347 | <code>        self.credit&#95;depth = credit.depth.max(1);</code> | Update the named state using the expression and assignment operator shown. |
| 348 | <code>        self.search&#95;started = self.started - credit.seconds;</code> | Update the named state using the expression and assignment operator shown. |
| 349 | <code>        self.hit&#95;time = Some(self.started);</code> | Update the named state using the expression and assignment operator shown. |
| 350 | <code>        self.hit&#95;depth = credit.depth;</code> | Update the named state using the expression and assignment operator shown. |
| 351 | <code>        self.hit&#95;kind = &quot;stop&#95;go&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 352 | <code>        self.score = credit.score;</code> | Update the named state using the expression and assignment operator shown. |
| 353 | <code>        self.score&#95;depth = credit.depth;</code> | Update the named state using the expression and assignment operator shown. |
| 354 | <code>        self.deteriorating = credit.deteriorating;</code> | Update the named state using the expression and assignment operator shown. |
| 355 | <code>        self.unstable = credit.unstable;</code> | Update the named state using the expression and assignment operator shown. |
| 356 | <code>        self.uncertain&#95;hit =</code> | Evaluate or continue this expression in credit&#95;ponder(); a final expression without a semicolon supplies its block&#x27;s value. |
| 357 | <code>            credit.depth &lt; SHALLOW&#95;DEPTH &#124;&#124; credit.deteriorating &#124;&#124; credit.unstable;</code> | Evaluate or continue this expression in credit&#95;ponder(); a final expression without a semicolon supplies its block&#x27;s value. |
| 358 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 359 | <code></code> | Blank line separating related declarations or steps. |
| 360 | <code>    /// Record a completed depth, the soft target (half the planned seconds), the</code> | Source explanation: Record a completed depth, the soft target (half the planned seconds), the |
| 361 | <code>    /// score, and whether the best move changed in this iteration.</code> | Source explanation: score, and whether the best move changed in this iteration. |
| 362 | <code>    pub fn completed(&amp;mut self, depth: i32, soft&#95;limit: f64, score: i32, changed: bool) {</code> | Declare completed(); its parameters and return type define the interface. |
| 363 | <code>        self.completed&#95;depth = depth;</code> | Update the named state using the expression and assignment operator shown. |
| 364 | <code>        self.soft&#95;limit = soft&#95;limit;</code> | Update the named state using the expression and assignment operator shown. |
| 365 | <code>        // The shallow iterations a Lucas restart replays keep the deeper score</code> | Source explanation: The shallow iterations a Lucas restart replays keep the deeper score |
| 366 | <code>        if depth &gt;= MATURE&#95;DEPTH.max(self.score&#95;depth) {</code> | Test the condition and execute this branch only when it matches. |
| 367 | <code>            if let Some(last) = self.score</code> | Test the condition and execute this branch only when it matches. |
| 368 | <code>                &amp;&amp; last - score &gt;= SCORE&#95;DROP</code> | Evaluate or continue this expression in completed(); a final expression without a semicolon supplies its block&#x27;s value. |
| 369 | <code>            {</code> | Evaluate or continue this expression in completed(); a final expression without a semicolon supplies its block&#x27;s value. |
| 370 | <code>                self.deteriorating = true;</code> | Update the named state using the expression and assignment operator shown. |
| 371 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 372 | <code>            self.unstable = changed;</code> | Update the named state using the expression and assignment operator shown. |
| 373 | <code>            self.score = Some(score);</code> | Update the named state using the expression and assignment operator shown. |
| 374 | <code>            self.score&#95;depth = depth;</code> | Update the named state using the expression and assignment operator shown. |
| 375 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 376 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 377 | <code></code> | Blank line separating related declarations or steps. |
| 378 | <code>    /// Report ponder work separately from time spent on our own clock.</code> | Source explanation: Report ponder work separately from time spent on our own clock. |
| 379 | <code>    pub fn report(&amp;self) {</code> | Declare report(); its parameters and return type define the interface. |
| 380 | <code>        if !self.was&#95;ponder {</code> | Test the condition and execute this branch only when it matches. |
| 381 | <code>            return;</code> | Leave the current function with the displayed result. |
| 382 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 383 | <code>        let end = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 384 | <code>        let ponder&#95;end = self.hit&#95;time.unwrap&#95;or(end);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 385 | <code>        let outcome = if self.hit&#95;time.is&#95;some() {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 386 | <code>            &quot;hit&quot;</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 387 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 388 | <code>            &quot;stopped&quot;</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 389 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 390 | <code>        let own&#95;seconds = self.hit&#95;time.map&#95;or(0.0, &#124;hit&#124; (end - hit).as&#95;secs&#95;f64());</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 391 | <code>        let ponder&#95;ms = (ponder&#95;end - self.search&#95;started).as&#95;secs&#95;f64() &#42; 1000.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 392 | <code>        let text = format!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 393 | <code>            &quot;ponder outcome={outcome} ponder&#95;ms={ponder&#95;ms:.1} hit&#95;depth={} hit&#95;nodes={} final&#95;depth={} final&#95;nodes={} own&#95;ms={:.1} hit&#95;min&#95;percent={}&quot;,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 394 | <code>            self.hit&#95;depth,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 395 | <code>            self.hit&#95;nodes,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 396 | <code>            self.completed&#95;depth,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 397 | <code>            self.nodes,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 398 | <code>            own&#95;seconds &#42; 1000.0,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 399 | <code>            self.hit&#95;min&#95;percent</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 400 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 401 | <code>        let hit&#95;cap&#95;ms = if self.soft&#95;limit.is&#95;finite() {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 402 | <code>            (2.0 &#42; self.soft&#95;limit).min(self.longest / 2.0) &#42; 1000.0</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 403 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 404 | <code>            0.0</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 405 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 406 | <code>        let text = format!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 407 | <code>            &quot;{text} hit&#95;kind={} reinvest={} shallow&#95;hit={} deteriorating={} unstable={} hit&#95;cap&#95;ms={hit&#95;cap&#95;ms:.1}&quot;,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 408 | <code>            self.hit&#95;kind,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 409 | <code>            u8::from(self.source == &quot;native&quot; &amp;&amp; self.reinvest()),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 410 | <code>            u8::from(self.hit&#95;depth &lt; SHALLOW&#95;DEPTH),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 411 | <code>            u8::from(self.deteriorating),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 412 | <code>            u8::from(self.unstable)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 413 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 414 | <code>        log(&amp;text);</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 415 | <code>        let mut out = std::io::stdout();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 416 | <code>        let &#95; = writeln!(out, &quot;info string {text}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 417 | <code>        let &#95; = out.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 418 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 419 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### search_metrics.rs

Collect worker and search accounting for optional diagnostics.

[Open source](src/search_metrics.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Per-worker search accounting. Node costs exclude nested double counting.</code> | Source explanation: Per-worker search accounting. Node costs exclude nested double counting. |
| 2 | <code></code> | Blank line separating related declarations or steps. |
| 3 | <code>#&#91;derive(Clone, Copy)&#93;</code> | Compiler attribute controlling the following declaration. |
| 4 | <code>pub enum Research {</code> | Declare the enum Research used by this module. |
| 5 | <code>    Pvs,</code> | Continue the enclosing declaration or initialization expression. |
| 6 | <code>    Lmr,</code> | Continue the enclosing declaration or initialization expression. |
| 7 | <code>    Aspiration,</code> | Continue the enclosing declaration or initialization expression. |
| 8 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 9 | <code></code> | Blank line separating related declarations or steps. |
| 10 | <code>#&#91;derive(Clone, Copy, Default)&#93;</code> | Compiler attribute controlling the following declaration. |
| 11 | <code>pub struct SearchMetrics {</code> | Declare the struct SearchMetrics used by this module. |
| 12 | <code>    pub nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 13 | <code>    pub active&#95;ns: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 14 | <code>    pub wait&#95;ns: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 15 | <code>    pub result&#95;wait&#95;ns: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 16 | <code>    pub cancel&#95;wait&#95;ns: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 17 | <code>    pub help&#95;tasks: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 18 | <code>    pub help&#95;nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 19 | <code>    pub help&#95;ns: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 20 | <code>    pub jobs: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 21 | <code>    pub pvs&#95;researches: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 22 | <code>    pub lmr&#95;researches: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 23 | <code>    pub aspiration&#95;researches: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 24 | <code>    pub research&#95;nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 25 | <code>    pub cutoff&#95;discarded&#95;nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 26 | <code>    pub stop&#95;discarded&#95;nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 27 | <code>    pub cutoff&#95;discarded&#95;tasks: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 28 | <code>    pub cutoff&#95;unclaimed&#95;tasks: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 29 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 30 | <code></code> | Blank line separating related declarations or steps. |
| 31 | <code>impl SearchMetrics {</code> | Begin the implementation of methods or a trait for the named type. |
| 32 | <code>    pub fn add(&amp;mut self, other: Self) {</code> | Declare add(); its parameters and return type define the interface. |
| 33 | <code>        self.nodes += other.nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 34 | <code>        self.active&#95;ns += other.active&#95;ns;</code> | Update the named state using the expression and assignment operator shown. |
| 35 | <code>        self.wait&#95;ns += other.wait&#95;ns;</code> | Update the named state using the expression and assignment operator shown. |
| 36 | <code>        self.result&#95;wait&#95;ns += other.result&#95;wait&#95;ns;</code> | Update the named state using the expression and assignment operator shown. |
| 37 | <code>        self.cancel&#95;wait&#95;ns += other.cancel&#95;wait&#95;ns;</code> | Update the named state using the expression and assignment operator shown. |
| 38 | <code>        self.help&#95;tasks += other.help&#95;tasks;</code> | Update the named state using the expression and assignment operator shown. |
| 39 | <code>        self.help&#95;nodes += other.help&#95;nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 40 | <code>        self.help&#95;ns += other.help&#95;ns;</code> | Update the named state using the expression and assignment operator shown. |
| 41 | <code>        self.jobs += other.jobs;</code> | Update the named state using the expression and assignment operator shown. |
| 42 | <code>        self.pvs&#95;researches += other.pvs&#95;researches;</code> | Update the named state using the expression and assignment operator shown. |
| 43 | <code>        self.lmr&#95;researches += other.lmr&#95;researches;</code> | Update the named state using the expression and assignment operator shown. |
| 44 | <code>        self.aspiration&#95;researches += other.aspiration&#95;researches;</code> | Update the named state using the expression and assignment operator shown. |
| 45 | <code>        self.research&#95;nodes += other.research&#95;nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 46 | <code>        self.cutoff&#95;discarded&#95;nodes += other.cutoff&#95;discarded&#95;nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 47 | <code>        self.stop&#95;discarded&#95;nodes += other.stop&#95;discarded&#95;nodes;</code> | Update the named state using the expression and assignment operator shown. |
| 48 | <code>        self.cutoff&#95;discarded&#95;tasks += other.cutoff&#95;discarded&#95;tasks;</code> | Update the named state using the expression and assignment operator shown. |
| 49 | <code>        self.cutoff&#95;unclaimed&#95;tasks += other.cutoff&#95;unclaimed&#95;tasks;</code> | Update the named state using the expression and assignment operator shown. |
| 50 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code></code> | Blank line separating related declarations or steps. |
| 53 | <code>impl std::fmt::Display for SearchMetrics {</code> | Begin the implementation of methods or a trait for the named type. |
| 54 | <code>    fn fmt(&amp;self, out: &amp;mut std::fmt::Formatter&lt;&#x27;&#95;&gt;) -&gt; std::fmt::Result {</code> | Declare fmt(); its parameters and return type define the interface. |
| 55 | <code>        write!(</code> | Format and send text to the output destination shown. |
| 56 | <code>            out,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 57 | <code>            &quot;worker&#95;nodes={} worker&#95;active&#95;ms={:.3} worker&#95;wait&#95;ms={:.3} worker&#95;jobs={} pvs&#95;researches={} lmr&#95;researches={} aspiration&#95;researches={} research&#95;nodes={} cutoff&#95;discarded&#95;nodes={} stop&#95;discarded&#95;nodes={} cutoff&#95;discarded&#95;tasks={} cutoff&#95;unclaimed&#95;tasks={} worker&#95;result&#95;wait&#95;ms={:.3} worker&#95;cancel&#95;wait&#95;ms={:.3} help&#95;tasks={} help&#95;nodes={} help&#95;active&#95;ms={:.3}&quot;,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 58 | <code>            self.nodes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 59 | <code>            self.active&#95;ns as f64 / 1&#95;000&#95;000.0,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 60 | <code>            self.wait&#95;ns as f64 / 1&#95;000&#95;000.0,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>            self.jobs,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>            self.pvs&#95;researches,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 63 | <code>            self.lmr&#95;researches,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>            self.aspiration&#95;researches,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>            self.research&#95;nodes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 66 | <code>            self.cutoff&#95;discarded&#95;nodes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>            self.stop&#95;discarded&#95;nodes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code>            self.cutoff&#95;discarded&#95;tasks,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 69 | <code>            self.cutoff&#95;unclaimed&#95;tasks,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 70 | <code>            self.result&#95;wait&#95;ns as f64 / 1&#95;000&#95;000.0,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 71 | <code>            self.cancel&#95;wait&#95;ns as f64 / 1&#95;000&#95;000.0,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 72 | <code>            self.help&#95;tasks,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 73 | <code>            self.help&#95;nodes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 74 | <code>            self.help&#95;ns as f64 / 1&#95;000&#95;000.0</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>        )</code> | Close the preceding block, expression, call, or collection. |
| 76 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 77 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### table_metrics.rs

Collect transposition-table usage and replacement measurements.

[Open source](src/table_metrics.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Worker-local table counters; no shared writes in the recursive search.</code> | Source explanation: Worker-local table counters; no shared writes in the recursive search. |
| 2 | <code></code> | Blank line separating related declarations or steps. |
| 3 | <code>#&#91;derive(Clone, Copy, Default)&#93;</code> | Compiler attribute controlling the following declaration. |
| 4 | <code>pub struct TableMetrics {</code> | Declare the struct TableMetrics used by this module. |
| 5 | <code>    pub probes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 6 | <code>    pub hits: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 7 | <code>    pub cutoffs: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 8 | <code>    pub stores: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 9 | <code>    pub replacements: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 10 | <code>    pub protected: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 11 | <code>    pub refreshes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 12 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 13 | <code></code> | Blank line separating related declarations or steps. |
| 14 | <code>impl TableMetrics {</code> | Begin the implementation of methods or a trait for the named type. |
| 15 | <code>    pub fn add(&amp;mut self, other: Self) {</code> | Declare add(); its parameters and return type define the interface. |
| 16 | <code>        self.probes += other.probes;</code> | Update the named state using the expression and assignment operator shown. |
| 17 | <code>        self.hits += other.hits;</code> | Update the named state using the expression and assignment operator shown. |
| 18 | <code>        self.cutoffs += other.cutoffs;</code> | Update the named state using the expression and assignment operator shown. |
| 19 | <code>        self.stores += other.stores;</code> | Update the named state using the expression and assignment operator shown. |
| 20 | <code>        self.replacements += other.replacements;</code> | Update the named state using the expression and assignment operator shown. |
| 21 | <code>        self.protected += other.protected;</code> | Update the named state using the expression and assignment operator shown. |
| 22 | <code>        self.refreshes += other.refreshes;</code> | Update the named state using the expression and assignment operator shown. |
| 23 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 24 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 25 | <code></code> | Blank line separating related declarations or steps. |
| 26 | <code>impl std::fmt::Display for TableMetrics {</code> | Begin the implementation of methods or a trait for the named type. |
| 27 | <code>    fn fmt(&amp;self, out: &amp;mut std::fmt::Formatter&lt;&#x27;&#95;&gt;) -&gt; std::fmt::Result {</code> | Declare fmt(); its parameters and return type define the interface. |
| 28 | <code>        write!(</code> | Format and send text to the output destination shown. |
| 29 | <code>            out,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>            &quot;tt&#95;probes={} tt&#95;hits={} tt&#95;cutoffs={} tt&#95;stores={} tt&#95;replacements={} tt&#95;hit&#95;pct={:.2} tt&#95;cutoff&#95;pct={:.2} tt&#95;replacement&#95;pct={:.2} tt&#95;protected={} tt&#95;refreshes={}&quot;,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>            self.probes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 32 | <code>            self.hits,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 33 | <code>            self.cutoffs,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 34 | <code>            self.stores,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 35 | <code>            self.replacements,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>            100.0 &#42; self.hits as f64 / self.probes.max(1) as f64,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 37 | <code>            100.0 &#42; self.cutoffs as f64 / self.probes.max(1) as f64,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>            100.0 &#42; self.replacements as f64 / self.stores.max(1) as f64,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 39 | <code>            self.protected,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 40 | <code>            self.refreshes</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>        )</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 43 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### tablebase.rs

Bounded root-only HTTPS advice, response validation, caching, and fallback.

[Open source](src/tablebase.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Query Lichess only at the root, keeping FEN, HTTPS, and JSON at the boundary.</code> | Source explanation: Query Lichess only at the root, keeping FEN, HTTPS, and JSON at the boundary. |
| 2 | <code></code> | Blank line separating related declarations or steps. |
| 3 | <code>use std::collections::{HashMap, HashSet};</code> | Bring the named items into scope; no chess search runs here. |
| 4 | <code>use std::io::Read;</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use std::sync::mpsc::{self, Receiver};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use std::time::{Duration, Instant, SystemTime};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code></code> | Blank line separating related declarations or steps. |
| 8 | <code>use serde&#95;json::Value;</code> | Bring the named items into scope; no chess search runs here. |
| 9 | <code></code> | Blank line separating related declarations or steps. |
| 10 | <code>use crate::attacks::in&#95;check;</code> | Bring the named items into scope; no chess search runs here. |
| 11 | <code>use crate::board::{Move, NO&#95;MOVE, Position, WHITE, move&#95;text};</code> | Bring the named items into scope; no chess search runs here. |
| 12 | <code>use crate::make&#95;move::make&#95;move;</code> | Bring the named items into scope; no chess search runs here. |
| 13 | <code>use crate::move&#95;generation::{MoveList, generate&#95;moves};</code> | Bring the named items into scope; no chess search runs here. |
| 14 | <code></code> | Blank line separating related declarations or steps. |
| 15 | <code>pub const ENDPOINT: &amp;str = &quot;https://tablebase.lichess.org/standard&quot;;</code> | Define ENDPOINT, including its explicit type and initial value. |
| 16 | <code>const MAX&#95;BYTES: u64 = 65&#95;536; // Bound the JSON response held in memory</code> | Define MAX&#95;BYTES, including its explicit type and initial value. |
| 17 | <code>const CACHE&#95;SIZE: usize = 256; // Positions retained in this engine process</code> | Define CACHE&#95;SIZE, including its explicit type and initial value. |
| 18 | <code>const MAX&#95;WAIT: f64 = 1.0; // At most one second and one quarter of the move budget</code> | Define MAX&#95;WAIT, including its explicit type and initial value. |
| 19 | <code>const MIN&#95;WAIT: f64 = 0.02; // Very short moves use native search immediately</code> | Define MIN&#95;WAIT, including its explicit type and initial value. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>type Response = (Option&lt;Value&gt;, Instant);</code> | Declare the type Response used by this module. |
| 22 | <code></code> | Blank line separating related declarations or steps. |
| 23 | <code>/// Serialize the native board and actual halfmove counter for one API request.</code> | Source explanation: Serialize the native board and actual halfmove counter for one API request. |
| 24 | <code>pub fn position&#95;fen(position: &amp;Position) -&gt; String {</code> | Declare position&#95;fen(); its parameters and return type define the interface. |
| 25 | <code>    let mut text = String::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 26 | <code>    // Row 0 is rank 8 and column 0 is file a, exactly the order FEN uses.</code> | Source explanation: Row 0 is rank 8 and column 0 is file a, exactly the order FEN uses. |
| 27 | <code>    for row in 0..8 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 28 | <code>        if row &gt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 29 | <code>            text.push(&#x27;/&#x27;);</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 31 | <code>        let mut gap = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 32 | <code>        for col in 0..8 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 33 | <code>            let piece = position.board&#91;row &#42; 8 + col&#93;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 34 | <code>            if piece == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 35 | <code>                gap += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 36 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 37 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 38 | <code>            if gap &gt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 39 | <code>                text.push&#95;str(&amp;gap.to&#95;string());</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 40 | <code>                gap = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 41 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code>            let letter = b&quot;.pnbrqk&quot;&#91;(piece &amp; 7) as usize&#93; as char;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 43 | <code>            text.push(if piece &amp; WHITE != 0 {</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 44 | <code>                letter.to&#95;ascii&#95;uppercase()</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 45 | <code>            } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 46 | <code>                letter</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 47 | <code>            });</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 48 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 49 | <code>        if gap &gt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 50 | <code>            text.push&#95;str(&amp;gap.to&#95;string());</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 51 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 53 | <code>    let mut rights = String::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 54 | <code>    for (letter, bit) in &#91;(&#x27;K&#x27;, 1), (&#x27;Q&#x27;, 2), (&#x27;k&#x27;, 4), (&#x27;q&#x27;, 8)&#93; {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 55 | <code>        if position.castling &amp; bit != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 56 | <code>            rights.push(letter);</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 57 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 58 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 59 | <code>    if rights.is&#95;empty() {</code> | Test the condition and execute this branch only when it matches. |
| 60 | <code>        rights.push(&#x27;-&#x27;);</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 62 | <code>    let target = if position.en&#95;passant &lt; 0 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 63 | <code>        &quot;-&quot;.to&#95;string()</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 65 | <code>        let square = position.en&#95;passant as u8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 66 | <code>        format!(</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>            &quot;{}{}&quot;,</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code>            (b&#x27;a&#x27; + square % 8) as char,</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 69 | <code>            (b&#x27;8&#x27; - square / 8) as char</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 70 | <code>        )</code> | Close the preceding block, expression, call, or collection. |
| 71 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 72 | <code>    // Fullmove number does not affect a probe. The engine does not store it.</code> | Source explanation: Fullmove number does not affect a probe. The engine does not store it. |
| 73 | <code>    format!(</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 74 | <code>        &quot;{text} {} {rights} {target} {} 1&quot;,</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>        if position.white { &quot;w&quot; } else { &quot;b&quot; },</code> | Test the condition and execute this branch only when it matches. |
| 76 | <code>        position.halfmove</code> | Evaluate or continue this expression in position&#95;fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 77 | <code>    )</code> | Close the preceding block, expression, call, or collection. |
| 78 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 79 | <code></code> | Blank line separating related declarations or steps. |
| 80 | <code>/// Probe only small positions without castling rights or repeated history.</code> | Source explanation: Probe only small positions without castling rights or repeated history. |
| 81 | <code>pub fn eligible(position: &amp;Position, history: &amp;&#91;u64&#93;) -&gt; bool {</code> | Declare eligible(); its parameters and return type define the interface. |
| 82 | <code>    if position.castling != 0 &#124;&#124; position.halfmove &gt;= 100 &#124;&#124; position.occupied().count&#95;ones() &gt; 7 {</code> | Test the condition and execute this branch only when it matches. |
| 83 | <code>        return false;</code> | Leave the current function with the displayed result. |
| 84 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 85 | <code>    let mut seen = HashSet::from(&#91;position.key&#93;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 86 | <code>    let start = history</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 87 | <code>        .len()</code> | Evaluate or continue this expression in eligible(); a final expression without a semicolon supplies its block&#x27;s value. |
| 88 | <code>        .saturating&#95;sub(position.halfmove.max(0) as usize);</code> | Evaluate or continue this expression in eligible(); a final expression without a semicolon supplies its block&#x27;s value. |
| 89 | <code>    for key in &amp;history&#91;start..&#93; {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 90 | <code>        if !seen.insert(&#42;key) {</code> | Test the condition and execute this branch only when it matches. |
| 91 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 92 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 93 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code>    true</code> | Evaluate or continue this expression in eligible(); a final expression without a semicolon supplies its block&#x27;s value. |
| 95 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 96 | <code></code> | Blank line separating related declarations or steps. |
| 97 | <code>/// Map only definite API categories to the outcome under the fifty-move rule.</code> | Source explanation: Map only definite API categories to the outcome under the fifty-move rule. |
| 98 | <code>fn outcome(category: &amp;Value) -&gt; Option&lt;i32&gt; {</code> | Declare outcome(); its parameters and return type define the interface. |
| 99 | <code>    match category.as&#95;str()? {</code> | Select an arm by matching the expression against the listed patterns. |
| 100 | <code>        &quot;win&quot; =&gt; Some(1),</code> | Handle this matching pattern with the expression or block on its right. |
| 101 | <code>        &quot;loss&quot; =&gt; Some(-1),</code> | Handle this matching pattern with the expression or block on its right. |
| 102 | <code>        &quot;draw&quot; &#124; &quot;cursed-win&quot; &#124; &quot;blessed-loss&quot; =&gt; Some(0),</code> | Handle this matching pattern with the expression or block on its right. |
| 103 | <code>        &#95; =&gt; None,</code> | Handle this matching pattern with the expression or block on its right. |
| 104 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 105 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 106 | <code></code> | Blank line separating related declarations or steps. |
| 107 | <code>/// Accept the first ranked move only when its result and native legality agree.</code> | Source explanation: Accept the first ranked move only when its result and native legality agree. |
| 108 | <code>pub fn response&#95;move(position: &amp;Position, history: &amp;&#91;u64&#93;, data: &amp;Value) -&gt; Move {</code> | Declare response&#95;move(); its parameters and return type define the interface. |
| 109 | <code>    let Some(root&#95;outcome) = outcome(&amp;data&#91;&quot;category&quot;&#93;) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 110 | <code>        return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 111 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 112 | <code>    let Some(first) = data&#91;&quot;moves&quot;&#93;.as&#95;array().and&#95;then(&#124;moves&#124; moves.first()) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 113 | <code>        return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 114 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 115 | <code>    if outcome(&amp;first&#91;&quot;category&quot;&#93;) != Some(-root&#95;outcome) {</code> | Test the condition and execute this branch only when it matches. |
| 116 | <code>        return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 117 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 118 | <code>    let Some(text) = first&#91;&quot;uci&quot;&#93;.as&#95;str() else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 119 | <code>        return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 120 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 121 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 122 | <code>    generate&#95;moves(position, &amp;mut list);</code> | Evaluate or continue this expression in response&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 123 | <code>    for &amp;mut mv in list.as&#95;mut&#95;slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 124 | <code>        if move&#95;text(mv) != text {</code> | Test the condition and execute this branch only when it matches. |
| 125 | <code>            continue;</code> | Skip the rest of this iteration and advance the loop. |
| 126 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 127 | <code>        let child = make&#95;move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 128 | <code>        if in&#95;check(&amp;child, position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 129 | <code>            return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 130 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 131 | <code>        let start = history</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 132 | <code>            .len()</code> | Evaluate or continue this expression in response&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 133 | <code>            .saturating&#95;sub(position.halfmove.max(0) as usize);</code> | Evaluate or continue this expression in response&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 134 | <code>        if child.halfmove &gt; 0 &amp;&amp; history&#91;start..&#93;.contains(&amp;child.key) {</code> | Test the condition and execute this branch only when it matches. |
| 135 | <code>            return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 136 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 137 | <code>        return mv;</code> | Leave the current function with the displayed result. |
| 138 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 139 | <code>    NO&#95;MOVE</code> | Evaluate or continue this expression in response&#95;move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 140 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 141 | <code></code> | Blank line separating related declarations or steps. |
| 142 | <code>/// Respect Retry-After seconds or an HTTP date, with at least a minute off.</code> | Source explanation: Respect Retry-After seconds or an HTTP date, with at least a minute off. |
| 143 | <code>pub fn retry&#95;delay(value: Option&lt;&amp;str&gt;) -&gt; Duration {</code> | Declare retry&#95;delay(); its parameters and return type define the interface. |
| 144 | <code>    let delay = value</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 145 | <code>        .and&#95;then(&#124;text&#124; {</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 146 | <code>            text.parse::&lt;u64&gt;()</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 147 | <code>                .ok()</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 148 | <code>                .map(Duration::from&#95;secs)</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 149 | <code>                .or&#95;else(&#124;&#124; {</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 150 | <code>                    httpdate::parse&#95;http&#95;date(text)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 151 | <code>                        .ok()?</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>                        .duration&#95;since(SystemTime::now())</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 153 | <code>                        .ok()</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 154 | <code>                })</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 155 | <code>        })</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 156 | <code>        .unwrap&#95;or(Duration::from&#95;secs(60));</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 157 | <code>    delay.max(Duration::from&#95;secs(60))</code> | Evaluate or continue this expression in retry&#95;delay(); a final expression without a semicolon supplies its block&#x27;s value. |
| 158 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 159 | <code></code> | Blank line separating related declarations or steps. |
| 160 | <code>/// Fetch one bounded response. TLS certificate validation stays enabled.</code> | Source explanation: Fetch one bounded response. TLS certificate validation stays enabled. |
| 161 | <code>fn fetch(fen: &amp;str, seconds: f64, endpoint: &amp;str) -&gt; Response {</code> | Declare fetch(); its parameters and return type define the interface. |
| 162 | <code>    let mut data = None;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 163 | <code>    let mut delay = Duration::from&#95;secs(5);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 164 | <code>    let result = (&#124;&#124; -&gt; Result&lt;(), Box&lt;dyn std::error::Error&gt;&gt; {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 165 | <code>        let client = reqwest::blocking::Client::builder()</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 166 | <code>            .timeout(Duration::from&#95;secs&#95;f64(seconds))</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 167 | <code>            .redirect(reqwest::redirect::Policy::none())</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 168 | <code>            .user&#95;agent(&quot;Remus7/1.0&quot;)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 169 | <code>            .build()?;</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 170 | <code>        let response = client</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 171 | <code>            .get(endpoint)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 172 | <code>            .query(&amp;&#91;(&quot;fen&quot;, fen)&#93;)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 173 | <code>            .header(&quot;Accept&quot;, &quot;application/json&quot;)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 174 | <code>            .send()?;</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 175 | <code>        if response.status().as&#95;u16() == 429 {</code> | Test the condition and execute this branch only when it matches. |
| 176 | <code>            delay = retry&#95;delay(</code> | Update the named state using the expression and assignment operator shown. |
| 177 | <code>                response</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 178 | <code>                    .headers()</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>                    .get(&quot;Retry-After&quot;)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 180 | <code>                    .and&#95;then(&#124;v&#124; v.to&#95;str().ok()),</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 181 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 182 | <code>            return Ok(());</code> | Leave the current function with the displayed result. |
| 183 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 184 | <code>        let mut body = Vec::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 185 | <code>        response</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 186 | <code>            .error&#95;for&#95;status()?</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 187 | <code>            .take(MAX&#95;BYTES + 1)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 188 | <code>            .read&#95;to&#95;end(&amp;mut body)?;</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 189 | <code>        if body.len() as u64 &lt;= MAX&#95;BYTES {</code> | Test the condition and execute this branch only when it matches. |
| 190 | <code>            let parsed: Value = serde&#95;json::from&#95;slice(&amp;body)?;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 191 | <code>            if parsed.is&#95;object() {</code> | Test the condition and execute this branch only when it matches. |
| 192 | <code>                data = Some(parsed);</code> | Update the named state using the expression and assignment operator shown. |
| 193 | <code>                delay = Duration::from&#95;secs(1);</code> | Update the named state using the expression and assignment operator shown. |
| 194 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 195 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 196 | <code>        Ok(())</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 197 | <code>    })();</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 198 | <code>    let &#95; = result; // A transport or JSON error selects native search.</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 199 | <code>    let now = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 200 | <code>    (</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 201 | <code>        data,</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 202 | <code>        now.checked&#95;add(delay)</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 203 | <code>            .unwrap&#95;or&#95;else(&#124;&#124; now + Duration::from&#95;secs(31&#95;536&#95;000)),</code> | Evaluate or continue this expression in fetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 204 | <code>    )</code> | Close the preceding block, expression, call, or collection. |
| 205 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 206 | <code></code> | Blank line separating related declarations or steps. |
| 207 | <code>/// One request at a time, a small cache, and a process-wide request cooldown.</code> | Source explanation: One request at a time, a small cache, and a process-wide request cooldown. |
| 208 | <code>pub struct Tablebase {</code> | Declare the struct Tablebase used by this module. |
| 209 | <code>    endpoint: String,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 210 | <code>    cache: HashMap&lt;String, Value&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 211 | <code>    pending: Option&lt;(String, Receiver&lt;Response&gt;)&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 212 | <code>    next&#95;request: Instant,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 213 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 214 | <code></code> | Blank line separating related declarations or steps. |
| 215 | <code>impl Tablebase {</code> | Begin the implementation of methods or a trait for the named type. |
| 216 | <code>    pub fn new(endpoint: &amp;str) -&gt; Self {</code> | Declare new(); its parameters and return type define the interface. |
| 217 | <code>        Self {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 218 | <code>            endpoint: endpoint.to&#95;string(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 219 | <code>            cache: HashMap::new(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 220 | <code>            pending: None,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 221 | <code>            next&#95;request: Instant::now(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 222 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 224 | <code></code> | Blank line separating related declarations or steps. |
| 225 | <code>    /// Collect a late result by its own FEN, never by the current root position.</code> | Source explanation: Collect a late result by its own FEN, never by the current root position. |
| 226 | <code>    fn collect(&amp;mut self, seconds: f64) {</code> | Declare collect(); its parameters and return type define the interface. |
| 227 | <code>        let Some((fen, receiver)) = &amp;self.pending else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 228 | <code>            return;</code> | Leave the current function with the displayed result. |
| 229 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 230 | <code>        match receiver.recv&#95;timeout(Duration::from&#95;secs&#95;f64(seconds.max(0.0))) {</code> | Select an arm by matching the expression against the listed patterns. |
| 231 | <code>            Ok((data, next&#95;request)) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 232 | <code>                self.next&#95;request = next&#95;request;</code> | Update the named state using the expression and assignment operator shown. |
| 233 | <code>                if let Some(data) = data {</code> | Test the condition and execute this branch only when it matches. |
| 234 | <code>                    if self.cache.len() &gt;= CACHE&#95;SIZE {</code> | Test the condition and execute this branch only when it matches. |
| 235 | <code>                        self.cache.clear();</code> | Evaluate or continue this expression in collect(); a final expression without a semicolon supplies its block&#x27;s value. |
| 236 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 237 | <code>                    self.cache.insert(fen.clone(), data);</code> | Evaluate or continue this expression in collect(); a final expression without a semicolon supplies its block&#x27;s value. |
| 238 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 239 | <code>                self.pending = None;</code> | Update the named state using the expression and assignment operator shown. |
| 240 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 241 | <code>            Err(mpsc::RecvTimeoutError::Disconnected) =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 242 | <code>                self.pending = None;</code> | Update the named state using the expression and assignment operator shown. |
| 243 | <code>                self.next&#95;request = Instant::now() + Duration::from&#95;secs(5);</code> | Update the named state using the expression and assignment operator shown. |
| 244 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 245 | <code>            Err(mpsc::RecvTimeoutError::Timeout) =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 246 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 247 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 248 | <code></code> | Blank line separating related declarations or steps. |
| 249 | <code>    /// Wait at most a quarter of the move budget, then let native search continue.</code> | Source explanation: Wait at most a quarter of the move budget, then let native search continue. |
| 250 | <code>    pub fn lookup(&amp;mut self, position: &amp;Position, history: &amp;&#91;u64&#93;, longest: f64) -&gt; Move {</code> | Declare lookup(); its parameters and return type define the interface. |
| 251 | <code>        self.collect(0.0);</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 252 | <code>        if !eligible(position, history) {</code> | Test the condition and execute this branch only when it matches. |
| 253 | <code>            return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 254 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 255 | <code>        let fen = position&#95;fen(position);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 256 | <code>        if let Some(data) = self.cache.get(&amp;fen) {</code> | Test the condition and execute this branch only when it matches. |
| 257 | <code>            return response&#95;move(position, history, data);</code> | Leave the current function with the displayed result. |
| 258 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 259 | <code>        let seconds = MAX&#95;WAIT.min(longest.max(0.0) / 4.0);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 260 | <code>        if self.pending.is&#95;some() &#124;&#124; Instant::now() &lt; self.next&#95;request &#124;&#124; seconds &lt; MIN&#95;WAIT {</code> | Test the condition and execute this branch only when it matches. |
| 261 | <code>            return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 262 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 263 | <code>        let (sender, receiver) = mpsc::channel();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 264 | <code>        let request&#95;fen = fen.clone();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 265 | <code>        let endpoint = self.endpoint.clone();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 266 | <code>        if std::thread::Builder::new()</code> | Test the condition and execute this branch only when it matches. |
| 267 | <code>            .spawn(move &#124;&#124; {</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 268 | <code>                let &#95; = sender.send(fetch(&amp;request&#95;fen, seconds, &amp;endpoint));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 269 | <code>            })</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 270 | <code>            .is&#95;err()</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 271 | <code>        {</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 272 | <code>            return NO&#95;MOVE;</code> | Leave the current function with the displayed result. |
| 273 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 274 | <code>        self.pending = Some((fen.clone(), receiver));</code> | Update the named state using the expression and assignment operator shown. |
| 275 | <code>        self.collect(seconds);</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 276 | <code>        self.cache</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 277 | <code>            .get(&amp;fen)</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 278 | <code>            .map&#95;or(NO&#95;MOVE, &#124;data&#124; response&#95;move(position, history, data))</code> | Evaluate or continue this expression in lookup(); a final expression without a semicolon supplies its block&#x27;s value. |
| 279 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 280 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### transposition_table.rs

Store and retrieve bounded search results using position keys.

[Open source](src/transposition_table.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Remember searched positions in four-entry, 64-byte buckets.</code> | Source explanation: Remember searched positions in four-entry, 64-byte buckets. |
| 2 | <code>//!</code> | Source explanation: |
| 3 | <code>//! The table holds a configurable number of buckets with multiply-shift hashing. A full bucket replaces its</code> | Source explanation: The table holds a configurable number of buckets with multiply-shift hashing. A full bucket replaces its |
| 4 | <code>//! least useful entry according to search age and depth. All search threads share it.</code> | Source explanation: least useful entry according to search age and depth. All search threads share it. |
| 5 | <code></code> | Blank line separating related declarations or steps. |
| 6 | <code>use std::arch::x86&#95;64::{&#95;MM&#95;HINT&#95;T0, &#95;mm&#95;prefetch};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code>use std::ffi::c&#95;void;</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code>use std::ptr;</code> | Bring the named items into scope; no chess search runs here. |
| 9 | <code>use std::sync::atomic::{AtomicU8, AtomicU64, Ordering::Relaxed};</code> | Bring the named items into scope; no chess search runs here. |
| 10 | <code></code> | Blank line separating related declarations or steps. |
| 11 | <code>use crate::board::{MATE&#95;SCORE, Move};</code> | Bring the named items into scope; no chess search runs here. |
| 12 | <code></code> | Blank line separating related declarations or steps. |
| 13 | <code>pub const EXACT: u8 = 0;</code> | Define EXACT, including its explicit type and initial value. |
| 14 | <code>pub const LOWER: u8 = 1;</code> | Define LOWER, including its explicit type and initial value. |
| 15 | <code>pub const UPPER: u8 = 2;</code> | Define UPPER, including its explicit type and initial value. |
| 16 | <code></code> | Blank line separating related declarations or steps. |
| 17 | <code>const MIB: usize = 1024 &#42; 1024;</code> | Define MIB, including its explicit type and initial value. |
| 18 | <code>const MEM&#95;COMMIT: u32 = 0x1000; // Back the region with Windows commit capacity</code> | Define MEM&#95;COMMIT, including its explicit type and initial value. |
| 19 | <code>const MEM&#95;RESERVE: u32 = 0x2000; // Reserve addresses for the table</code> | Define MEM&#95;RESERVE, including its explicit type and initial value. |
| 20 | <code>const MEM&#95;RELEASE: u32 = 0x8000; // Release the whole allocation on drop</code> | Define MEM&#95;RELEASE, including its explicit type and initial value. |
| 21 | <code>const PAGE&#95;READWRITE: u32 = 0x04; // Ordinary writable, nonexecutable pages</code> | Define PAGE&#95;READWRITE, including its explicit type and initial value. |
| 22 | <code></code> | Blank line separating related declarations or steps. |
| 23 | <code>#&#91;link(name = &quot;kernel32&quot;)&#93;</code> | Compiler attribute controlling the following declaration. |
| 24 | <code>unsafe extern &quot;system&quot; {</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 25 | <code>    fn VirtualAlloc(</code> | Declare VirtualAlloc(); its parameters and return type define the interface. |
| 26 | <code>        address: &#42;mut c&#95;void,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 27 | <code>        bytes: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 28 | <code>        allocation: u32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 29 | <code>        protect: u32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 30 | <code>    ) -&gt; &#42;mut c&#95;void;</code> | Evaluate or continue this expression in VirtualAlloc(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>    fn VirtualFree(address: &#42;mut c&#95;void, bytes: usize, free&#95;type: u32) -&gt; i32;</code> | Declare VirtualFree(); its parameters and return type define the interface. |
| 32 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 33 | <code>const ENTRIES&#95;PER&#95;BUCKET: usize = 4;</code> | Define ENTRIES&#95;PER&#95;BUCKET, including its explicit type and initial value. |
| 34 | <code>const AGE&#95;LIMIT: u8 = 64; // Ages wrap around at this count of searches</code> | Define AGE&#95;LIMIT, including its explicit type and initial value. |
| 35 | <code>const PAYLOAD&#95;MASK: u64 = (1&#95;u64 &lt;&lt; 58) - 1; // Age is independent of key validation</code> | Define PAYLOAD&#95;MASK, including its explicit type and initial value. |
| 36 | <code>const AGE&#95;WEIGHT: i32 = 8; // One search of age counts as much as this many plies</code> | Define AGE&#95;WEIGHT, including its explicit type and initial value. |
| 37 | <code></code> | Blank line separating related declarations or steps. |
| 38 | <code>/// Each entry holds the key XOR the score/move/depth/bound payload, then data.</code> | Source explanation: Each entry holds the key XOR the score/move/depth/bound payload, then data. |
| 39 | <code>/// Age occupies the upper six data bits and is excluded from key validation, so</code> | Source explanation: Age occupies the upper six data bits and is excluded from key validation, so |
| 40 | <code>/// refreshing age with a single compare-exchange cannot mix score and key words.</code> | Source explanation: refreshing age with a single compare-exchange cannot mix score and key words. |
| 41 | <code>/// Concurrent payload writers can produce a cache miss; readers validate the key.</code> | Source explanation: Concurrent payload writers can produce a cache miss; readers validate the key. |
| 42 | <code>#&#91;repr(align(64))&#93;</code> | Compiler attribute controlling the following declaration. |
| 43 | <code>struct Bucket {</code> | Declare the struct Bucket used by this module. |
| 44 | <code>    slots: &#91;&#91;AtomicU64; 2&#93;; ENTRIES&#95;PER&#95;BUCKET&#93;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 45 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 46 | <code></code> | Blank line separating related declarations or steps. |
| 47 | <code>const &#95;: () = assert!(size&#95;of::&lt;Bucket&gt;() == 64);</code> | Define &#95;, including its explicit type and initial value. |
| 48 | <code></code> | Blank line separating related declarations or steps. |
| 49 | <code>/// Pack score, move, depth, bound, and age into one word.</code> | Source explanation: Pack score, move, depth, bound, and age into one word. |
| 50 | <code>#&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 51 | <code>fn pack(score: i32, mv: Move, depth: u8, bound&#95;and&#95;age: u8) -&gt; u64 {</code> | Declare pack(); its parameters and return type define the interface. |
| 52 | <code>    u64::from(score as u32)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 53 | <code>        &#124; (u64::from(mv) &lt;&lt; 32)</code> | Evaluate or continue this expression in pack(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>        &#124; (u64::from(depth) &lt;&lt; 48)</code> | Evaluate or continue this expression in pack(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>        &#124; (u64::from(bound&#95;and&#95;age) &lt;&lt; 56)</code> | Evaluate or continue this expression in pack(); a final expression without a semicolon supplies its block&#x27;s value. |
| 56 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 57 | <code></code> | Blank line separating related declarations or steps. |
| 58 | <code>/// Map the whole 64-bit key range onto the fixed table.</code> | Source explanation: Map the whole 64-bit key range onto the fixed table. |
| 59 | <code>#&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 60 | <code>fn bucket&#95;index(key: u64, bucket&#95;count: usize) -&gt; usize {</code> | Declare bucket&#95;index(); its parameters and return type define the interface. |
| 61 | <code>    ((u128::from(key) &#42; bucket&#95;count as u128) &gt;&gt; 64) as usize</code> | Evaluate or continue this expression in bucket&#95;index(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 63 | <code></code> | Blank line separating related declarations or steps. |
| 64 | <code>/// Obtain aligned zero-filled pages without touching the whole table at startup.</code> | Source explanation: Obtain aligned zero-filled pages without touching the whole table at startup. |
| 65 | <code>fn allocate&#95;buckets(bytes: usize) -&gt; &#42;mut Bucket {</code> | Declare allocate&#95;buckets(); its parameters and return type define the interface. |
| 66 | <code>    // Windows aligns the base to at least a page boundary, exceeding Bucket&#x27;s 64</code> | Source explanation: Windows aligns the base to at least a page boundary, exceeding Bucket&#x27;s 64 |
| 67 | <code>    // bytes. Zero bits are valid atomics. Pages are supplied on access.</code> | Source explanation: bytes. Zero bits are valid atomics. Pages are supplied on access. |
| 68 | <code>    unsafe {</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 69 | <code>        VirtualAlloc(</code> | Evaluate or continue this expression in allocate&#95;buckets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 70 | <code>            ptr::null&#95;mut(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 71 | <code>            bytes,</code> | Evaluate or continue this expression in allocate&#95;buckets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 72 | <code>            MEM&#95;RESERVE &#124; MEM&#95;COMMIT,</code> | Evaluate or continue this expression in allocate&#95;buckets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 73 | <code>            PAGE&#95;READWRITE,</code> | Evaluate or continue this expression in allocate&#95;buckets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 74 | <code>        )</code> | Close the preceding block, expression, call, or collection. |
| 75 | <code>        .cast()</code> | Evaluate or continue this expression in allocate&#95;buckets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 76 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 77 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 78 | <code></code> | Blank line separating related declarations or steps. |
| 79 | <code>/// What probe() found: the depth, score, bound, and best move of a position.</code> | Source explanation: What probe() found: the depth, score, bound, and best move of a position. |
| 80 | <code>#&#91;derive(Clone, Copy)&#93;</code> | Compiler attribute controlling the following declaration. |
| 81 | <code>pub struct Stored {</code> | Declare the struct Stored used by this module. |
| 82 | <code>    data: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 83 | <code>    pub depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 84 | <code>    pub score: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 85 | <code>    pub bound: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 86 | <code>    pub mv: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 87 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 88 | <code></code> | Blank line separating related declarations or steps. |
| 89 | <code>#&#91;derive(Default)&#93;</code> | Compiler attribute controlling the following declaration. |
| 90 | <code>pub struct StoreResult {</code> | Declare the struct StoreResult used by this module. |
| 91 | <code>    pub replaced: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 92 | <code>    pub protected: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 93 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code></code> | Blank line separating related declarations or steps. |
| 95 | <code>pub struct Table {</code> | Declare the struct Table used by this module. |
| 96 | <code>    buckets: &#42;mut Bucket,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 97 | <code>    bucket&#95;count: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 98 | <code>    age: AtomicU8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 99 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 100 | <code></code> | Blank line separating related declarations or steps. |
| 101 | <code>// Every access to the buckets goes through atomic words, so threads may share them.</code> | Source explanation: Every access to the buckets goes through atomic words, so threads may share them. |
| 102 | <code>unsafe impl Send for Table {}</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 103 | <code>unsafe impl Sync for Table {}</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 104 | <code></code> | Blank line separating related declarations or steps. |
| 105 | <code>impl Table {</code> | Begin the implementation of methods or a trait for the named type. |
| 106 | <code>    pub fn new() -&gt; Table {</code> | Declare new(); its parameters and return type define the interface. |
| 107 | <code>        Self::with&#95;size(512)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 108 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 109 | <code></code> | Blank line separating related declarations or steps. |
| 110 | <code>    pub fn disabled() -&gt; Table {</code> | Declare disabled(); its parameters and return type define the interface. |
| 111 | <code>        Table {</code> | Evaluate or continue this expression in disabled(); a final expression without a semicolon supplies its block&#x27;s value. |
| 112 | <code>            buckets: ptr::null&#95;mut(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 113 | <code>            bucket&#95;count: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 114 | <code>            age: AtomicU8::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 115 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 116 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 117 | <code></code> | Blank line separating related declarations or steps. |
| 118 | <code>    pub fn with&#95;size(mib: usize) -&gt; Table {</code> | Declare with&#95;size(); its parameters and return type define the interface. |
| 119 | <code>        assert!(matches!(mib, 512 &#124; 1024 &#124; 2048));</code> | Evaluate or continue this expression in with&#95;size(); a final expression without a semicolon supplies its block&#x27;s value. |
| 120 | <code>        Table {</code> | Evaluate or continue this expression in with&#95;size(); a final expression without a semicolon supplies its block&#x27;s value. |
| 121 | <code>            buckets: allocate&#95;buckets(mib &#42; MIB),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 122 | <code>            bucket&#95;count: mib &#42; MIB / size&#95;of::&lt;Bucket&gt;(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 123 | <code>            age: AtomicU8::new(0),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 124 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 125 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 126 | <code></code> | Blank line separating related declarations or steps. |
| 127 | <code>    pub fn size&#95;mib(&amp;self) -&gt; usize {</code> | Declare size&#95;mib(); its parameters and return type define the interface. |
| 128 | <code>        if self.buckets.is&#95;null() {</code> | Test the condition and execute this branch only when it matches. |
| 129 | <code>            0</code> | Evaluate or continue this expression in size&#95;mib(); a final expression without a semicolon supplies its block&#x27;s value. |
| 130 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 131 | <code>            self.bucket&#95;count &#42; size&#95;of::&lt;Bucket&gt;() / MIB</code> | Evaluate or continue this expression in size&#95;mib(); a final expression without a semicolon supplies its block&#x27;s value. |
| 132 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 133 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 134 | <code></code> | Blank line separating related declarations or steps. |
| 135 | <code>    /// Estimate occupied slots, including older searches, from 4,096 spread buckets.</code> | Source explanation: Estimate occupied slots, including older searches, from 4,096 spread buckets. |
| 136 | <code>    /// Call between searches, after all workers have finished writing the table.</code> | Source explanation: Call between searches, after all workers have finished writing the table. |
| 137 | <code>    pub fn fill&#95;percentage(&amp;self) -&gt; f64 {</code> | Declare fill&#95;percentage(); its parameters and return type define the interface. |
| 138 | <code>        if self.buckets.is&#95;null() {</code> | Test the condition and execute this branch only when it matches. |
| 139 | <code>            return 0.0;</code> | Leave the current function with the displayed result. |
| 140 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 141 | <code>        let sampled&#95;buckets = self.bucket&#95;count.min(4096);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 142 | <code>        let stride = self.bucket&#95;count / sampled&#95;buckets;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 143 | <code>        let mut occupied = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 144 | <code>        for sample in 0..sampled&#95;buckets {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 145 | <code>            let index = sample &#42; stride + stride / 2;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 146 | <code>            // Each midpoint is inside the live allocation, away from adjacent samples.</code> | Source explanation: Each midpoint is inside the live allocation, away from adjacent samples. |
| 147 | <code>            let slots = unsafe { &amp;(&#42;self.buckets.add(index)).slots };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 148 | <code>            for slot in slots {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 149 | <code>                let data = slot&#91;1&#93;.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 150 | <code>                if slot&#91;0&#93;.load(Relaxed) ^ (data &amp; PAYLOAD&#95;MASK) != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 151 | <code>                    occupied += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 152 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 153 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 154 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>        100.0 &#42; occupied as f64 / (sampled&#95;buckets &#42; ENTRIES&#95;PER&#95;BUCKET) as f64</code> | Evaluate or continue this expression in fill&#95;percentage(); a final expression without a semicolon supplies its block&#x27;s value. |
| 156 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 157 | <code></code> | Blank line separating related declarations or steps. |
| 158 | <code>    /// Mark the start of a new search, so older entries give way first.</code> | Source explanation: Mark the start of a new search, so older entries give way first. |
| 159 | <code>    pub fn new&#95;search(&amp;self) {</code> | Declare new&#95;search(); its parameters and return type define the interface. |
| 160 | <code>        let age = self.age.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 161 | <code>        self.age.store((age + 1) % AGE&#95;LIMIT, Relaxed);</code> | Evaluate or continue this expression in new&#95;search(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 163 | <code></code> | Blank line separating related declarations or steps. |
| 164 | <code>    /// Save one search result. A mate score is stored as mate from here.</code> | Source explanation: Save one search result. A mate score is stored as mate from here. |
| 165 | <code>    #&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 166 | <code>    pub fn store(</code> | Declare store(); its parameters and return type define the interface. |
| 167 | <code>        &amp;self,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 168 | <code>        key: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 169 | <code>        depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 170 | <code>        mut score: i32,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 171 | <code>        bound: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 172 | <code>        mv: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 173 | <code>        ply: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 174 | <code>    ) -&gt; StoreResult {</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 175 | <code>        if score &gt; MATE&#95;SCORE - 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 176 | <code>            score += ply;</code> | Update the named state using the expression and assignment operator shown. |
| 177 | <code>        } else if score &lt; -MATE&#95;SCORE + 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 178 | <code>            score -= ply;</code> | Update the named state using the expression and assignment operator shown. |
| 179 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 180 | <code>        if self.buckets.is&#95;null() &#124;&#124; key == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 181 | <code>            return StoreResult::default(); // Zero marks an empty entry, so this key is not cached.</code> | Leave the current function with the displayed result. |
| 182 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 183 | <code>        let age = self.age.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 184 | <code>        let index = bucket&#95;index(key, self.bucket&#95;count);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 185 | <code>        // The index is inside the owned allocation, and every word is atomic.</code> | Source explanation: The index is inside the owned allocation, and every word is atomic. |
| 186 | <code>        let slots = unsafe { &amp;(&#42;self.buckets.add(index)).slots };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 187 | <code>        let mut chosen = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 188 | <code>        let mut replacement = true;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 189 | <code>        let mut lowest&#95;value = i32::MAX;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 190 | <code>        for (i, slot) in slots.iter().enumerate() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 191 | <code>            let data = slot&#91;1&#93;.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 192 | <code>            let stored&#95;key = slot&#91;0&#93;.load(Relaxed) ^ (data &amp; PAYLOAD&#95;MASK);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 193 | <code>            if stored&#95;key == key {</code> | Test the condition and execute this branch only when it matches. |
| 194 | <code>                let stored&#95;depth = i32::from((data &gt;&gt; 48) as u8);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 195 | <code>                let stored&#95;bound = (data &gt;&gt; 56) as u8 &amp; 3;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 196 | <code>                if stored&#95;depth &gt; depth</code> | Test the condition and execute this branch only when it matches. |
| 197 | <code>                    &#124;&#124; (stored&#95;depth == depth &amp;&amp; stored&#95;bound == EXACT &amp;&amp; bound != EXACT)</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 198 | <code>                {</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 199 | <code>                    return StoreResult {</code> | Leave the current function with the displayed result. |
| 200 | <code>                        replaced: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 201 | <code>                        protected: true,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 202 | <code>                    };</code> | Close the preceding block, expression, call, or collection. |
| 203 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 204 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 205 | <code>            if stored&#95;key == key &#124;&#124; stored&#95;key == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 206 | <code>                replacement = false;</code> | Update the named state using the expression and assignment operator shown. |
| 207 | <code>                chosen = i;</code> | Update the named state using the expression and assignment operator shown. |
| 208 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 209 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 210 | <code>            let stored&#95;age = (data &gt;&gt; 58) as u8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 211 | <code>            let searches&#95;ago = i32::from(age.wrapping&#95;sub(stored&#95;age) % AGE&#95;LIMIT);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 212 | <code>            let value = i32::from((data &gt;&gt; 48) as u8) - AGE&#95;WEIGHT &#42; searches&#95;ago;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 213 | <code>            if value &lt; lowest&#95;value {</code> | Test the condition and execute this branch only when it matches. |
| 214 | <code>                lowest&#95;value = value;</code> | Update the named state using the expression and assignment operator shown. |
| 215 | <code>                chosen = i;</code> | Update the named state using the expression and assignment operator shown. |
| 216 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 217 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 218 | <code>        let data = pack(score, mv, depth as u8, bound &#124; (age &lt;&lt; 2));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 219 | <code>        slots&#91;chosen&#93;&#91;0&#93;.store(key ^ (data &amp; PAYLOAD&#95;MASK), Relaxed);</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 220 | <code>        slots&#91;chosen&#93;&#91;1&#93;.store(data, Relaxed);</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 221 | <code>        StoreResult {</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 222 | <code>            replaced: replacement,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 223 | <code>            protected: false,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 224 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 225 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 226 | <code></code> | Blank line separating related declarations or steps. |
| 227 | <code>    /// Refresh a useful cutoff once per search generation. A changed payload or key</code> | Source explanation: Refresh a useful cutoff once per search generation. A changed payload or key |
| 228 | <code>    /// makes this best-effort update a no-op; score, depth, bound, and move stay intact.</code> | Source explanation: makes this best-effort update a no-op; score, depth, bound, and move stay intact. |
| 229 | <code>    #&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 230 | <code>    pub fn refresh(&amp;self, key: u64, stored: &amp;Stored) -&gt; bool {</code> | Declare refresh(); its parameters and return type define the interface. |
| 231 | <code>        let age = u64::from(self.age.load(Relaxed));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 232 | <code>        if self.buckets.is&#95;null() &#124;&#124; key == 0 &#124;&#124; stored.data &gt;&gt; 58 == age {</code> | Test the condition and execute this branch only when it matches. |
| 233 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 234 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 235 | <code>        let index = bucket&#95;index(key, self.bucket&#95;count);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 236 | <code>        // The bucket belongs to this live table; only atomic words are accessed.</code> | Source explanation: The bucket belongs to this live table; only atomic words are accessed. |
| 237 | <code>        let slots = unsafe { &amp;(&#42;self.buckets.add(index)).slots };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 238 | <code>        for slot in slots {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 239 | <code>            if slot&#91;0&#93;.load(Relaxed) ^ (stored.data &amp; PAYLOAD&#95;MASK) == key {</code> | Test the condition and execute this branch only when it matches. |
| 240 | <code>                return slot&#91;1&#93;</code> | Leave the current function with the displayed result. |
| 241 | <code>                    .compare&#95;exchange(</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 242 | <code>                        stored.data,</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 243 | <code>                        (stored.data &amp; PAYLOAD&#95;MASK) &#124; (age &lt;&lt; 58),</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 244 | <code>                        Relaxed,</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 245 | <code>                        Relaxed,</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 246 | <code>                    )</code> | Close the preceding block, expression, call, or collection. |
| 247 | <code>                    .is&#95;ok();</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 248 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 249 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 250 | <code>        false</code> | Evaluate or continue this expression in refresh(); a final expression without a semicolon supplies its block&#x27;s value. |
| 251 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 252 | <code></code> | Blank line separating related declarations or steps. |
| 253 | <code>    /// Ask the processor to start loading the bucket of a position. The table is far</code> | Source explanation: Ask the processor to start loading the bucket of a position. The table is far |
| 254 | <code>    /// larger than the processor&#x27;s cache, so a bucket takes about as long to arrive</code> | Source explanation: larger than the processor&#x27;s cache, so a bucket takes about as long to arrive |
| 255 | <code>    /// from memory as a whole position takes to search; asking as soon as the key is</code> | Source explanation: from memory as a whole position takes to search; asking as soon as the key is |
| 256 | <code>    /// known lets the search work during that wait.</code> | Source explanation: known lets the search work during that wait. |
| 257 | <code>    #&#91;inline(always)&#93;</code> | Compiler attribute controlling the following declaration. |
| 258 | <code>    pub fn prefetch(&amp;self, key: u64) {</code> | Declare prefetch(); its parameters and return type define the interface. |
| 259 | <code>        if !self.buckets.is&#95;null() {</code> | Test the condition and execute this branch only when it matches. |
| 260 | <code>            let bucket = self</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 261 | <code>                .buckets</code> | Evaluate or continue this expression in prefetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 262 | <code>                .wrapping&#95;add(bucket&#95;index(key, self.bucket&#95;count));</code> | Evaluate or continue this expression in prefetch(); a final expression without a semicolon supplies its block&#x27;s value. |
| 263 | <code>            // A prefetch reads nothing and cannot fault, whatever the address.</code> | Source explanation: A prefetch reads nothing and cannot fault, whatever the address. |
| 264 | <code>            unsafe {</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 265 | <code>                &#95;mm&#95;prefetch::&lt;&#95;MM&#95;HINT&#95;T0&gt;(bucket.cast::&lt;i8&gt;());</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 266 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 267 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 268 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 269 | <code></code> | Blank line separating related declarations or steps. |
| 270 | <code>    /// Return what the table holds for the position, or None.</code> | Source explanation: Return what the table holds for the position, or None. |
| 271 | <code>    #&#91;inline&#93;</code> | Compiler attribute controlling the following declaration. |
| 272 | <code>    pub fn probe(&amp;self, key: u64, ply: i32) -&gt; Option&lt;Stored&gt; {</code> | Declare probe(); its parameters and return type define the interface. |
| 273 | <code>        if self.buckets.is&#95;null() &#124;&#124; key == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 274 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 275 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 276 | <code>        let index = bucket&#95;index(key, self.bucket&#95;count);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 277 | <code>        // The allocation stays live while self is borrowed, and index is in bounds.</code> | Source explanation: The allocation stays live while self is borrowed, and index is in bounds. |
| 278 | <code>        let slots = unsafe { &amp;(&#42;self.buckets.add(index)).slots };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 279 | <code>        for slot in slots {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 280 | <code>            let data = slot&#91;1&#93;.load(Relaxed);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 281 | <code>            if slot&#91;0&#93;.load(Relaxed) ^ (data &amp; PAYLOAD&#95;MASK) == key {</code> | Test the condition and execute this branch only when it matches. |
| 282 | <code>                let mut score = data as u32 as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 283 | <code>                if score &gt; MATE&#95;SCORE - 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 284 | <code>                    score -= ply;</code> | Update the named state using the expression and assignment operator shown. |
| 285 | <code>                } else if score &lt; -MATE&#95;SCORE + 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 286 | <code>                    score += ply;</code> | Update the named state using the expression and assignment operator shown. |
| 287 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 288 | <code>                return Some(Stored {</code> | Leave the current function with the displayed result. |
| 289 | <code>                    data,</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 290 | <code>                    depth: i32::from((data &gt;&gt; 48) as u8),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 291 | <code>                    score,</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 292 | <code>                    bound: (data &gt;&gt; 56) as u8 &amp; 3,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 293 | <code>                    mv: (data &gt;&gt; 32) as Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 294 | <code>                });</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 295 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 296 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 297 | <code>        None</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 298 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 299 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 300 | <code></code> | Blank line separating related declarations or steps. |
| 301 | <code>impl Drop for Table {</code> | Begin the implementation of methods or a trait for the named type. |
| 302 | <code>    fn drop(&amp;mut self) {</code> | Declare drop(); its parameters and return type define the interface. |
| 303 | <code>        if !self.buckets.is&#95;null() {</code> | Test the condition and execute this branch only when it matches. |
| 304 | <code>            // This base belongs to VirtualAlloc, not the Rust heap allocator.</code> | Source explanation: This base belongs to VirtualAlloc, not the Rust heap allocator. |
| 305 | <code>            unsafe {</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 306 | <code>                VirtualFree(self.buckets.cast(), 0, MEM&#95;RELEASE);</code> | Evaluate or continue this expression in drop(); a final expression without a semicolon supplies its block&#x27;s value. |
| 307 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 308 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 309 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 310 | <code>}</code> | Close the preceding block, expression, call, or collection. |

## Source coverage and identity

The reference covers all 15 runtime Rust modules and all 4,380 physical source lines. These are source-reference line numbers, not student typing totals. The overview's Lines of code count excludes blank and comment lines. SHA-256 identifies exact source bytes, including line endings.

| Source | SHA-256 |
| --- | --- |
| [src/attacks.rs](src/attacks.rs) | `a603a0e200d148fe4f24c53173ae0059ec52fc4080f05789fc6714355d287965` |
| [src/board.rs](src/board.rs) | `f270d32c8b7f79bc95d1487a35d6996936ee6f461f375036a5cbdcee0751f537` |
| [src/evaluation.rs](src/evaluation.rs) | `a6573df53533f20a4cb59968c196a49d759e5ca61a6e61d3f814bd9ec98311e8` |
| [src/exchange.rs](src/exchange.rs) | `c1b1fe90c897dccafcf28744abb1b2d127397f0c073ea0051bf75cf76b50e892` |
| [src/main.rs](src/main.rs) | `b128d212c51eeb2fccbd381ee000991a42ec26c5732436fe76f03b9c0a9b860a` |
| [src/make_move.rs](src/make_move.rs) | `0fb0a99aeae26a9e125532a2f888267077766f06906d0a4a501ca11f54d34460` |
| [src/move_generation.rs](src/move_generation.rs) | `0aed5b9d2d00495e33d7ff2b892a2c84b302e72d923809504cc70ae0e665fb39` |
| [src/opening_book.rs](src/opening_book.rs) | `37dbe9750c830b6a4132d199555ffe06fd63d834303ceb874866dd821e69f865` |
| [src/parallel_search.rs](src/parallel_search.rs) | `214b7e3c54283c96ed8eff42426a1abb03ab2279c1399a81e442a9acb7ef8ba5` |
| [src/search.rs](src/search.rs) | `5de2ed80cb0d81825ee0d42529c0813148e78489d372cd00de58b374efb81427` |
| [src/search_control.rs](src/search_control.rs) | `eda35cbcabd670c935e837af68c65dacf8492f50b9287e63d6864ad74b8475dc` |
| [src/search_metrics.rs](src/search_metrics.rs) | `2f18a00dfec15ef59890102ca8e916252321ea12eb6c5ca66379a2278bdec31b` |
| [src/table_metrics.rs](src/table_metrics.rs) | `17a22b54527c25bd5d24e3a3e615b36d7f96732ce3a0dbf93af30b73ddd8f9e9` |
| [src/tablebase.rs](src/tablebase.rs) | `084f32ecf0d76c8de4a529b433976ae337548b98eb47921c998da81ae7d31c33` |
| [src/transposition_table.rs](src/transposition_table.rs) | `ccd78022c360b4eb97cdb14333242c328978d2b2ff4216e4baa3ee0bec402698` |
