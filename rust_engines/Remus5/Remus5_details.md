# Remus5 source reference

Read [the engine overview](remus5.md) for the algorithms, options, and measurement limits. This reference shows every source line, including comments and spacing, with structural annotations. Original source comments carry the algorithm-specific reasoning. Source identity is not a behavior test.

## Line-by-line source reference

### attacks.rs

Bitboard rays, piece attacks, and king-safety queries.

[Open source](src/attacks.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Find out what each piece reaches and whether a square is attacked.</code> | Source explanation: Find out what each piece reaches and whether a square is attacked. |
| 2 | <code>//!</code> | Source explanation:  |
| 3 | <code>//! The tables hold, for each square, a bitboard of what a knight, king, or pawn on that</code> | Source explanation: The tables hold, for each square, a bitboard of what a knight, king, or pawn on that |
| 4 | <code>//! square reaches, and the squares along each of the eight rays. The compiler fills them</code> | Source explanation: square reaches, and the squares along each of the eight rays. The compiler fills them |
| 5 | <code>//! in while it builds the program, so they cost nothing when the program runs.</code> | Source explanation: in while it builds the program, so they cost nothing when the program runs. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>use crate::board::{BISHOP, KING, KNIGHT, PAWN, Position, QUEEN, ROOK};</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code></code> | Blank line separating related declarations or steps. |
| 9 | <code>const KNIGHT_JUMPS: [(i32, i32); 8] = [</code> | Define KNIGHT_JUMPS, including its explicit type and initial value. |
| 10 | <code>    (-2, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 11 | <code>    (-2, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 12 | <code>    (-1, -2),</code> | Continue the enclosing declaration or initialization expression. |
| 13 | <code>    (-1, 2),</code> | Continue the enclosing declaration or initialization expression. |
| 14 | <code>    (1, -2),</code> | Continue the enclosing declaration or initialization expression. |
| 15 | <code>    (1, 2),</code> | Continue the enclosing declaration or initialization expression. |
| 16 | <code>    (2, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 17 | <code>    (2, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 18 | <code>];</code> | Close the preceding block, expression, call, or collection. |
| 19 | <code></code> | Blank line separating related declarations or steps. |
| 20 | <code>// The rays: the first four raise the square number and the last four lower it.</code> | Source explanation: The rays: the first four raise the square number and the last four lower it. |
| 21 | <code>// Rooks use rays 0, 1, 4, and 5, and bishops use rays 2, 3, 6, and 7.</code> | Source explanation: Rooks use rays 0, 1, 4, and 5, and bishops use rays 2, 3, 6, and 7. |
| 22 | <code>const DIRECTIONS: [(i32, i32); 8] = [</code> | Define DIRECTIONS, including its explicit type and initial value. |
| 23 | <code>    (1, 0),</code> | Continue the enclosing declaration or initialization expression. |
| 24 | <code>    (0, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 25 | <code>    (1, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 26 | <code>    (1, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 27 | <code>    (-1, 0),</code> | Continue the enclosing declaration or initialization expression. |
| 28 | <code>    (0, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 29 | <code>    (-1, -1),</code> | Continue the enclosing declaration or initialization expression. |
| 30 | <code>    (-1, 1),</code> | Continue the enclosing declaration or initialization expression. |
| 31 | <code>];</code> | Close the preceding block, expression, call, or collection. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>const fn bit_if_on_board(row: i32, col: i32) -&gt; u64 {</code> | Declare bit_if_on_board(); its parameters and return type define the interface. |
| 34 | <code>    if row &gt;= 0 &amp;&amp; row &lt; 8 &amp;&amp; col &gt;= 0 &amp;&amp; col &lt; 8 {</code> | Test the condition and execute this branch only when it matches. |
| 35 | <code>        1_u64 &lt;&lt; (row * 8 + col)</code> | Evaluate or continue this expression in bit_if_on_board(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 37 | <code>        0</code> | Evaluate or continue this expression in bit_if_on_board(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 39 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 40 | <code></code> | Blank line separating related declarations or steps. |
| 41 | <code>const fn build_steps(steps: &amp;[(i32, i32); 8]) -&gt; [u64; 64] {</code> | Declare build_steps(); its parameters and return type define the interface. |
| 42 | <code>    let mut table = [0; 64];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 43 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 44 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 45 | <code>        let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>        let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>        let mut i = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 48 | <code>        while i &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 49 | <code>            table[square] &#124;= bit_if_on_board(row + steps[i].0, col + steps[i].1);</code> | Update the named state using the expression and assignment operator shown. |
| 50 | <code>            i += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 51 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 54 | <code>    table</code> | Evaluate or continue this expression in build_steps(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code></code> | Blank line separating related declarations or steps. |
| 57 | <code>const fn build_rays() -&gt; [[u64; 64]; 8] {</code> | Declare build_rays(); its parameters and return type define the interface. |
| 58 | <code>    let mut table = [[0; 64]; 8];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 59 | <code>    let mut direction = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 60 | <code>    while direction &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 61 | <code>        let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 62 | <code>        while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 63 | <code>            let mut row = (square / 8) as i32 + DIRECTIONS[direction].0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 64 | <code>            let mut col = (square % 8) as i32 + DIRECTIONS[direction].1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 65 | <code>            while bit_if_on_board(row, col) != 0 {</code> | Repeat this block while its condition remains true. |
| 66 | <code>                table[direction][square] &#124;= bit_if_on_board(row, col);</code> | Update the named state using the expression and assignment operator shown. |
| 67 | <code>                row += DIRECTIONS[direction].0;</code> | Update the named state using the expression and assignment operator shown. |
| 68 | <code>                col += DIRECTIONS[direction].1;</code> | Update the named state using the expression and assignment operator shown. |
| 69 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 70 | <code>            square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 71 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 72 | <code>        direction += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 73 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code>    table</code> | Evaluate or continue this expression in build_rays(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 76 | <code></code> | Blank line separating related declarations or steps. |
| 77 | <code>const fn build_pawn_attacks() -&gt; [[u64; 64]; 2] {</code> | Declare build_pawn_attacks(); its parameters and return type define the interface. |
| 78 | <code>    let mut table = [[0; 64]; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 79 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 80 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 81 | <code>        let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 82 | <code>        let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 83 | <code>        // Row 0 is rank 8, so a White pawn attacks toward row 0</code> | Source explanation: Row 0 is rank 8, so a White pawn attacks toward row 0 |
| 84 | <code>        table[0][square] = bit_if_on_board(row - 1, col - 1) &#124; bit_if_on_board(row - 1, col + 1);</code> | Update the named state using the expression and assignment operator shown. |
| 85 | <code>        table[1][square] = bit_if_on_board(row + 1, col - 1) &#124; bit_if_on_board(row + 1, col + 1);</code> | Update the named state using the expression and assignment operator shown. |
| 86 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 87 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 88 | <code>    table</code> | Evaluate or continue this expression in build_pawn_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 89 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 90 | <code></code> | Blank line separating related declarations or steps. |
| 91 | <code>pub static KNIGHT_ATTACKS: [u64; 64] = build_steps(&amp;KNIGHT_JUMPS);</code> | Define KNIGHT_ATTACKS, including its explicit type and initial value. |
| 92 | <code>pub static KING_ATTACKS: [u64; 64] = build_steps(&amp;DIRECTIONS);</code> | Define KING_ATTACKS, including its explicit type and initial value. |
| 93 | <code>pub static PAWN_ATTACKS: [[u64; 64]; 2] = build_pawn_attacks();</code> | Define PAWN_ATTACKS, including its explicit type and initial value. |
| 94 | <code>static RAYS: [[u64; 64]; 8] = build_rays();</code> | Define RAYS, including its explicit type and initial value. |
| 95 | <code></code> | Blank line separating related declarations or steps. |
| 96 | <code>/// The squares along a ray that raises the square number, up to the first piece.</code> | Source explanation: The squares along a ray that raises the square number, up to the first piece. |
| 97 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 98 | <code>fn rising_ray(direction: usize, square: usize, occupied: u64) -&gt; u64 {</code> | Declare rising_ray(); its parameters and return type define the interface. |
| 99 | <code>    let ray = RAYS[direction][square];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 100 | <code>    let blockers = ray &amp; occupied;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 101 | <code>    if blockers == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 102 | <code>        ray</code> | Evaluate or continue this expression in rising_ray(); a final expression without a semicolon supplies its block&#x27;s value. |
| 103 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 104 | <code>        ray ^ RAYS[direction][blockers.trailing_zeros() as usize]</code> | Evaluate or continue this expression in rising_ray(); a final expression without a semicolon supplies its block&#x27;s value. |
| 105 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 106 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 107 | <code></code> | Blank line separating related declarations or steps. |
| 108 | <code>/// The squares along a ray that lowers the square number, up to the first piece.</code> | Source explanation: The squares along a ray that lowers the square number, up to the first piece. |
| 109 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 110 | <code>fn falling_ray(direction: usize, square: usize, occupied: u64) -&gt; u64 {</code> | Declare falling_ray(); its parameters and return type define the interface. |
| 111 | <code>    let ray = RAYS[direction][square];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 112 | <code>    let blockers = ray &amp; occupied;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 113 | <code>    if blockers == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 114 | <code>        ray</code> | Evaluate or continue this expression in falling_ray(); a final expression without a semicolon supplies its block&#x27;s value. |
| 115 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 116 | <code>        ray ^ RAYS[direction][63 - blockers.leading_zeros() as usize]</code> | Evaluate or continue this expression in falling_ray(); a final expression without a semicolon supplies its block&#x27;s value. |
| 117 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 118 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 119 | <code></code> | Blank line separating related declarations or steps. |
| 120 | <code>/// The squares a rook on the square reaches, including the first piece on each line.</code> | Source explanation: The squares a rook on the square reaches, including the first piece on each line. |
| 121 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 122 | <code>pub fn rook_attacks(square: usize, occupied: u64) -&gt; u64 {</code> | Declare rook_attacks(); its parameters and return type define the interface. |
| 123 | <code>    rising_ray(0, square, occupied)</code> | Evaluate or continue this expression in rook_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 124 | <code>        &#124; rising_ray(1, square, occupied)</code> | Evaluate or continue this expression in rook_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 125 | <code>        &#124; falling_ray(4, square, occupied)</code> | Evaluate or continue this expression in rook_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 126 | <code>        &#124; falling_ray(5, square, occupied)</code> | Evaluate or continue this expression in rook_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 127 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 128 | <code></code> | Blank line separating related declarations or steps. |
| 129 | <code>/// The squares a bishop on the square reaches, including the first piece on each line.</code> | Source explanation: The squares a bishop on the square reaches, including the first piece on each line. |
| 130 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 131 | <code>pub fn bishop_attacks(square: usize, occupied: u64) -&gt; u64 {</code> | Declare bishop_attacks(); its parameters and return type define the interface. |
| 132 | <code>    rising_ray(2, square, occupied)</code> | Evaluate or continue this expression in bishop_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 133 | <code>        &#124; rising_ray(3, square, occupied)</code> | Evaluate or continue this expression in bishop_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 134 | <code>        &#124; falling_ray(6, square, occupied)</code> | Evaluate or continue this expression in bishop_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 135 | <code>        &#124; falling_ray(7, square, occupied)</code> | Evaluate or continue this expression in bishop_attacks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 136 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 137 | <code></code> | Blank line separating related declarations or steps. |
| 138 | <code>/// Return true if a piece of the given color attacks the square.</code> | Source explanation: Return true if a piece of the given color attacks the square. |
| 139 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 140 | <code>pub fn is_attacked(position: &amp;Position, square: usize, by_white: bool) -&gt; bool {</code> | Declare is_attacked(); its parameters and return type define the interface. |
| 141 | <code>    let side = usize::from(!by_white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 142 | <code>    let attackers = position.by_side[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 143 | <code>    let kinds = &amp;position.by_kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 144 | <code>    // Look out from the square: a pawn of the other color on the square would attack</code> | Source explanation: Look out from the square: a pawn of the other color on the square would attack |
| 145 | <code>    // exactly the squares where a pawn of this color attacks it</code> | Source explanation: exactly the squares where a pawn of this color attacks it |
| 146 | <code>    if PAWN_ATTACKS[1 - side][square] &amp; attackers &amp; kinds[PAWN as usize] != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 147 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 148 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 149 | <code>    if KNIGHT_ATTACKS[square] &amp; attackers &amp; kinds[KNIGHT as usize] != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 150 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 151 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 152 | <code>    if KING_ATTACKS[square] &amp; attackers &amp; kinds[KING as usize] != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 153 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 154 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>    let queens = kinds[QUEEN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 156 | <code>    let occupied = position.occupied();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 157 | <code>    let straight = attackers &amp; (kinds[ROOK as usize] &#124; queens);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>    if straight != 0 &amp;&amp; rook_attacks(square, occupied) &amp; straight != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 159 | <code>        return true;</code> | Leave the current function with the displayed result. |
| 160 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 161 | <code>    let diagonal = attackers &amp; (kinds[BISHOP as usize] &#124; queens);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 162 | <code>    diagonal != 0 &amp;&amp; bishop_attacks(square, occupied) &amp; diagonal != 0</code> | Evaluate or continue this expression in is_attacked(); a final expression without a semicolon supplies its block&#x27;s value. |
| 163 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 164 | <code></code> | Blank line separating related declarations or steps. |
| 165 | <code>/// Return true if the king of the given color is attacked.</code> | Source explanation: Return true if the king of the given color is attacked. |
| 166 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 167 | <code>pub fn in_check(position: &amp;Position, white: bool) -&gt; bool {</code> | Declare in_check(); its parameters and return type define the interface. |
| 168 | <code>    let kings = position.by_side[usize::from(!white)] &amp; position.by_kind[KING as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 169 | <code>    is_attacked(position, kings.trailing_zeros() as usize, !white)</code> | Evaluate or continue this expression in in_check(); a final expression without a semicolon supplies its block&#x27;s value. |
| 170 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### board.rs

Position and move representation, square coordinates, and position keys.

[Open source](src/board.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! The position, the starting position, and the names of squares and moves.</code> | Source explanation: The position, the starting position, and the names of squares and moves. |
| 2 | <code>//!</code> | Source explanation:  |
| 3 | <code>//! Square numbers match Remus4: row 0 is rank 8 and column 0 is file a, so a8 is 0,</code> | Source explanation: Square numbers match Remus4: row 0 is rank 8 and column 0 is file a, so a8 is 0, |
| 4 | <code>//! h8 is 7, a1 is 56, and h1 is 63. A piece is its kind, from 1 for a pawn to 6 for a</code> | Source explanation: h8 is 7, a1 is 56, and h1 is 63. A piece is its kind, from 1 for a pawn to 6 for a |
| 5 | <code>//! king, plus 8 if it is White.</code> | Source explanation: king, plus 8 if it is White. |
| 6 | <code>//!</code> | Source explanation:  |
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
| 19 | <code>pub const MATE_SCORE: i32 = 100_000; // Larger than any material count</code> | Define MATE_SCORE, including its explicit type and initial value. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>/// A move packed as from * 512 + to * 8 + promotion. Comparing two packed moves gives</code> | Source explanation: A move packed as from * 512 + to * 8 + promotion. Comparing two packed moves gives |
| 22 | <code>/// the same order as comparing the tuples (from, to, promotion) in Remus4. The number 0</code> | Source explanation: the same order as comparing the tuples (from, to, promotion) in Remus4. The number 0 |
| 23 | <code>/// means &quot;no move&quot;: a8 to a8 is never played.</code> | Source explanation: means &quot;no move&quot;: a8 to a8 is never played. |
| 24 | <code>pub type Move = u16;</code> | Declare the type Move used by this module. |
| 25 | <code></code> | Blank line separating related declarations or steps. |
| 26 | <code>pub const NO_MOVE: Move = 0;</code> | Define NO_MOVE, including its explicit type and initial value. |
| 27 | <code></code> | Blank line separating related declarations or steps. |
| 28 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 29 | <code>pub const fn new_move(start: usize, target: usize, promotion: u8) -&gt; Move {</code> | Declare new_move(); its parameters and return type define the interface. |
| 30 | <code>    ((start &lt;&lt; 9) &#124; (target &lt;&lt; 3) &#124; promotion as usize) as Move</code> | Evaluate or continue this expression in new_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 34 | <code>pub const fn move_start(mv: Move) -&gt; usize {</code> | Declare move_start(); its parameters and return type define the interface. |
| 35 | <code>    (mv &gt;&gt; 9) as usize</code> | Evaluate or continue this expression in move_start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 37 | <code></code> | Blank line separating related declarations or steps. |
| 38 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 39 | <code>pub const fn move_target(mv: Move) -&gt; usize {</code> | Declare move_target(); its parameters and return type define the interface. |
| 40 | <code>    ((mv &gt;&gt; 3) &amp; 63) as usize</code> | Evaluate or continue this expression in move_target(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code></code> | Blank line separating related declarations or steps. |
| 43 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 44 | <code>pub const fn move_promotion(mv: Move) -&gt; u8 {</code> | Declare move_promotion(); its parameters and return type define the interface. |
| 45 | <code>    (mv &amp; 7) as u8</code> | Evaluate or continue this expression in move_promotion(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 47 | <code></code> | Blank line separating related declarations or steps. |
| 48 | <code>/// One step of the SplitMix64 random number generator, usable at compile time.</code> | Source explanation: One step of the SplitMix64 random number generator, usable at compile time. |
| 49 | <code>const fn next_random(state: u64) -&gt; (u64, u64) {</code> | Declare next_random(); its parameters and return type define the interface. |
| 50 | <code>    let state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 51 | <code>    let mut z = state;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 52 | <code>    z = (z ^ (z &gt;&gt; 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>    z = (z ^ (z &gt;&gt; 27)).wrapping_mul(0x94D0_49BB_1331_11EB);</code> | Update the named state using the expression and assignment operator shown. |
| 54 | <code>    (state, z ^ (z &gt;&gt; 31))</code> | Evaluate or continue this expression in next_random(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code></code> | Blank line separating related declarations or steps. |
| 57 | <code>/// A random number for every piece on every square, for each en passant square, for</code> | Source explanation: A random number for every piece on every square, for each en passant square, for |
| 58 | <code>/// each set of castling rights, and for Black to move. The key of a position is the</code> | Source explanation: each set of castling rights, and for Black to move. The key of a position is the |
| 59 | <code>/// exclusive-or of the numbers that describe it, which make_move() updates in a few</code> | Source explanation: exclusive-or of the numbers that describe it, which make_move() updates in a few |
| 60 | <code>/// steps instead of reading all 64 squares.</code> | Source explanation: steps instead of reading all 64 squares. |
| 61 | <code>pub struct KeyTable {</code> | Declare the struct KeyTable used by this module. |
| 62 | <code>    pub piece: [[u64; 64]; 15],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 63 | <code>    pub en_passant: [u64; 64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 64 | <code>    pub castling: [u64; 16],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 65 | <code>    pub black: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 66 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code></code> | Blank line separating related declarations or steps. |
| 68 | <code>const fn build_keys() -&gt; KeyTable {</code> | Declare build_keys(); its parameters and return type define the interface. |
| 69 | <code>    let mut table = KeyTable {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 70 | <code>        piece: [[0; 64]; 15],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 71 | <code>        en_passant: [0; 64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 72 | <code>        castling: [0; 16],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 73 | <code>        black: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 74 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 75 | <code>    let mut state = 2016_u64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 76 | <code>    let mut piece = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 77 | <code>    while piece &lt; 15 {</code> | Repeat this block while its condition remains true. |
| 78 | <code>        let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 79 | <code>        while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 80 | <code>            let (next, value) = next_random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 81 | <code>            state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 82 | <code>            table.piece[piece][square] = value;</code> | Update the named state using the expression and assignment operator shown. |
| 83 | <code>            square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 84 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 85 | <code>        piece += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 86 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 87 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 88 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 89 | <code>        let (next, value) = next_random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 90 | <code>        state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 91 | <code>        table.en_passant[square] = value;</code> | Update the named state using the expression and assignment operator shown. |
| 92 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 93 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code>    let mut rights = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 95 | <code>    while rights &lt; 16 {</code> | Repeat this block while its condition remains true. |
| 96 | <code>        let (next, value) = next_random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 97 | <code>        state = next;</code> | Update the named state using the expression and assignment operator shown. |
| 98 | <code>        table.castling[rights] = value;</code> | Update the named state using the expression and assignment operator shown. |
| 99 | <code>        rights += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 100 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 101 | <code>    let (_, value) = next_random(state);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>    table.black = value;</code> | Update the named state using the expression and assignment operator shown. |
| 103 | <code>    table</code> | Evaluate or continue this expression in build_keys(); a final expression without a semicolon supplies its block&#x27;s value. |
| 104 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 105 | <code></code> | Blank line separating related declarations or steps. |
| 106 | <code>pub static KEYS: KeyTable = build_keys();</code> | Define KEYS, including its explicit type and initial value. |
| 107 | <code></code> | Blank line separating related declarations or steps. |
| 108 | <code>/// The position. Copying this small struct is the copy of copy-make.</code> | Source explanation: The position. Copying this small struct is the copy of copy-make. |
| 109 | <code>#[derive(Clone, Copy)]</code> | Compiler attribute controlling the following declaration. |
| 110 | <code>pub struct Position {</code> | Declare the struct Position used by this module. |
| 111 | <code>    pub board: [u8; 64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 112 | <code>    pub by_kind: [u64; 7], // The squares of each kind of piece, of either side</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 113 | <code>    pub by_side: [u64; 2], // The squares of White&#x27;s pieces, then Black&#x27;s</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 114 | <code>    pub key: u64,          // The number that names the position</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 115 | <code>    pub white: bool,       // True when White is to move</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 116 | <code>    pub castling: u8,      // 1, 2 for White&#x27;s rights, 4, 8 for Black&#x27;s</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 117 | <code>    pub en_passant: i8,    // The square a pawn just skipped over, or -1</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 118 | <code>    pub halfmove: i32,     // The moves since the last capture or pawn move</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 119 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 120 | <code></code> | Blank line separating related declarations or steps. |
| 121 | <code>impl Position {</code> | Begin the implementation of methods or a trait for the named type. |
| 122 | <code>    /// Build a position from its plain parts.</code> | Source explanation: Build a position from its plain parts. |
| 123 | <code>    pub fn new(</code> | Declare new(); its parameters and return type define the interface. |
| 124 | <code>        board: [u8; 64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 125 | <code>        white: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 126 | <code>        castling: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 127 | <code>        en_passant: i8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 128 | <code>        halfmove: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 129 | <code>    ) -&gt; Position {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 130 | <code>        let mut position = Position {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 131 | <code>            board: [EMPTY; 64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 132 | <code>            by_kind: [0; 7],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 133 | <code>            by_side: [0; 2],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 134 | <code>            key: KEYS.castling[castling as usize],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 135 | <code>            white,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 136 | <code>            castling,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 137 | <code>            en_passant,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 138 | <code>            halfmove,</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 139 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 140 | <code>        for (square, &amp;piece) in board.iter().enumerate() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 141 | <code>            if piece != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 142 | <code>                position.put_piece(square, piece);</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 143 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 144 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 145 | <code>        if !white {</code> | Test the condition and execute this branch only when it matches. |
| 146 | <code>            position.key ^= KEYS.black;</code> | Update the named state using the expression and assignment operator shown. |
| 147 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 148 | <code>        if en_passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 149 | <code>            position.key ^= KEYS.en_passant[en_passant as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 150 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 151 | <code>        position</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 153 | <code></code> | Blank line separating related declarations or steps. |
| 154 | <code>    /// The starting position.</code> | Source explanation: The starting position. |
| 155 | <code>    pub fn start() -&gt; Position {</code> | Declare start(); its parameters and return type define the interface. |
| 156 | <code>        let back_rank = [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 157 | <code>        let mut board = [EMPTY; 64];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>        for col in 0..8 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 159 | <code>            board[col] = back_rank[col];</code> | Update the named state using the expression and assignment operator shown. |
| 160 | <code>            board[8 + col] = PAWN;</code> | Evaluate or continue this expression in start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 161 | <code>            board[48 + col] = WHITE + PAWN;</code> | Evaluate or continue this expression in start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>            board[56 + col] = WHITE + back_rank[col];</code> | Evaluate or continue this expression in start(); a final expression without a semicolon supplies its block&#x27;s value. |
| 163 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 164 | <code>        Position::new(board, true, 15, -1, 0)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 165 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 166 | <code></code> | Blank line separating related declarations or steps. |
| 167 | <code>    /// Build a position from FEN text, for the workshop designer&#x27;s checks.</code> | Source explanation: Build a position from FEN text, for the workshop designer&#x27;s checks. |
| 168 | <code>    pub fn from_fen(words: &amp;[&amp;str]) -&gt; Position {</code> | Declare from_fen(); its parameters and return type define the interface. |
| 169 | <code>        let mut board = [EMPTY; 64];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 170 | <code>        let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 171 | <code>        for letter in words[0].chars() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 172 | <code>            if let Some(gap) = letter.to_digit(10) {</code> | Test the condition and execute this branch only when it matches. |
| 173 | <code>                square += gap as usize;</code> | Update the named state using the expression and assignment operator shown. |
| 174 | <code>            } else if letter != &#x27;/&#x27; {</code> | Test the condition and execute this branch only when it matches. |
| 175 | <code>                let kind = &quot;.pnbrqk&quot;.find(letter.to_ascii_lowercase()).unwrap_or(0) as u8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 176 | <code>                board[square] = kind</code> | Update the named state using the expression and assignment operator shown. |
| 177 | <code>                    + if letter.is_ascii_uppercase() {</code> | Evaluate or continue this expression in from_fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 178 | <code>                        WHITE</code> | Evaluate or continue this expression in from_fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>                    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 180 | <code>                        0</code> | Evaluate or continue this expression in from_fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 181 | <code>                    };</code> | Close the preceding block, expression, call, or collection. |
| 182 | <code>                square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 183 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 184 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 185 | <code>        let mut castling = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 186 | <code>        for (letter, bit) in [(&#x27;K&#x27;, 1), (&#x27;Q&#x27;, 2), (&#x27;k&#x27;, 4), (&#x27;q&#x27;, 8)] {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 187 | <code>            if words[2].contains(letter) {</code> | Test the condition and execute this branch only when it matches. |
| 188 | <code>                castling += bit;</code> | Update the named state using the expression and assignment operator shown. |
| 189 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 190 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 191 | <code>        let en_passant = if words[3] == &quot;-&quot; {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 192 | <code>            -1</code> | Evaluate or continue this expression in from_fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 193 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 194 | <code>            parse_square(words[3]) as i8</code> | Evaluate or continue this expression in from_fen(); a final expression without a semicolon supplies its block&#x27;s value. |
| 195 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 196 | <code>        let halfmove = words.get(4).and_then(&#124;text&#124; text.parse().ok()).unwrap_or(0);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 197 | <code>        Position::new(board, words[1] == &quot;w&quot;, castling, en_passant, halfmove)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 198 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 199 | <code></code> | Blank line separating related declarations or steps. |
| 200 | <code>    /// Place a piece on an empty square.</code> | Source explanation: Place a piece on an empty square. |
| 201 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 202 | <code>    pub fn put_piece(&amp;mut self, square: usize, piece: u8) {</code> | Declare put_piece(); its parameters and return type define the interface. |
| 203 | <code>        let bit = 1_u64 &lt;&lt; square;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 204 | <code>        self.board[square] = piece;</code> | Update the named state using the expression and assignment operator shown. |
| 205 | <code>        self.by_kind[(piece &amp; 7) as usize] &#124;= bit;</code> | Evaluate or continue this expression in put_piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 206 | <code>        self.by_side[usize::from(piece &lt; WHITE)] &#124;= bit;</code> | Evaluate or continue this expression in put_piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 207 | <code>        self.key ^= KEYS.piece[piece as usize][square];</code> | Update the named state using the expression and assignment operator shown. |
| 208 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 209 | <code></code> | Blank line separating related declarations or steps. |
| 210 | <code>    /// Lift the piece off a square.</code> | Source explanation: Lift the piece off a square. |
| 211 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 212 | <code>    pub fn remove_piece(&amp;mut self, square: usize) {</code> | Declare remove_piece(); its parameters and return type define the interface. |
| 213 | <code>        let piece = self.board[square];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 214 | <code>        let bit = 1_u64 &lt;&lt; square;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 215 | <code>        self.board[square] = EMPTY;</code> | Update the named state using the expression and assignment operator shown. |
| 216 | <code>        self.by_kind[(piece &amp; 7) as usize] &amp;= !bit;</code> | Evaluate or continue this expression in remove_piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 217 | <code>        self.by_side[usize::from(piece &lt; WHITE)] &amp;= !bit;</code> | Evaluate or continue this expression in remove_piece(); a final expression without a semicolon supplies its block&#x27;s value. |
| 218 | <code>        self.key ^= KEYS.piece[piece as usize][square];</code> | Update the named state using the expression and assignment operator shown. |
| 219 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 220 | <code></code> | Blank line separating related declarations or steps. |
| 221 | <code>    /// The squares of every piece on the board.</code> | Source explanation: The squares of every piece on the board. |
| 222 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 223 | <code>    pub fn occupied(&amp;self) -&gt; u64 {</code> | Declare occupied(); its parameters and return type define the interface. |
| 224 | <code>        self.by_side[0] &#124; self.by_side[1]</code> | Evaluate or continue this expression in occupied(); a final expression without a semicolon supplies its block&#x27;s value. |
| 225 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 226 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 227 | <code></code> | Blank line separating related declarations or steps. |
| 228 | <code>/// Return the square that text such as &quot;e2&quot; names.</code> | Source explanation: Return the square that text such as &quot;e2&quot; names. |
| 229 | <code>pub fn parse_square(text: &amp;str) -&gt; usize {</code> | Declare parse_square(); its parameters and return type define the interface. |
| 230 | <code>    let bytes = text.as_bytes();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 231 | <code>    let row = b&#x27;8&#x27;.wrapping_sub(bytes[1]) as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 232 | <code>    let col = bytes[0].wrapping_sub(b&#x27;a&#x27;) as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 233 | <code>    (row * 8 + col) &amp; 63</code> | Evaluate or continue this expression in parse_square(); a final expression without a semicolon supplies its block&#x27;s value. |
| 234 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 235 | <code></code> | Blank line separating related declarations or steps. |
| 236 | <code>/// Add the name of a square, such as &quot;e2&quot;, to the text.</code> | Source explanation: Add the name of a square, such as &quot;e2&quot;, to the text. |
| 237 | <code>fn push_square(text: &amp;mut String, square: usize) {</code> | Declare push_square(); its parameters and return type define the interface. |
| 238 | <code>    text.push((b&#x27;a&#x27; + (square % 8) as u8) as char);</code> | Evaluate or continue this expression in push_square(); a final expression without a semicolon supplies its block&#x27;s value. |
| 239 | <code>    text.push((b&#x27;8&#x27; - (square / 8) as u8) as char);</code> | Evaluate or continue this expression in push_square(); a final expression without a semicolon supplies its block&#x27;s value. |
| 240 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 241 | <code></code> | Blank line separating related declarations or steps. |
| 242 | <code>/// Return the UCI text of a move, such as &quot;e7e8q&quot;.</code> | Source explanation: Return the UCI text of a move, such as &quot;e7e8q&quot;. |
| 243 | <code>pub fn move_text(mv: Move) -&gt; String {</code> | Declare move_text(); its parameters and return type define the interface. |
| 244 | <code>    let mut text = String::with_capacity(5);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 245 | <code>    push_square(&amp;mut text, move_start(mv));</code> | Evaluate or continue this expression in move_text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 246 | <code>    push_square(&amp;mut text, move_target(mv));</code> | Evaluate or continue this expression in move_text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 247 | <code>    let promotion = move_promotion(mv) as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 248 | <code>    if promotion != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 249 | <code>        text.push(b&quot;.pnbrqk&quot;[promotion] as char);</code> | Evaluate or continue this expression in move_text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 250 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 251 | <code>    text</code> | Evaluate or continue this expression in move_text(); a final expression without a semicolon supplies its block&#x27;s value. |
| 252 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### diagnostics.rs

Write optional per-process diagnostic records without changing normal UCI output.

[Open source](src/diagnostics.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Save search measurements at command and iteration boundaries, outside the node loop.</code> | Source explanation: Save search measurements at command and iteration boundaries, outside the node loop. |
| 2 | <code></code> | Blank line separating related declarations or steps. |
| 3 | <code>use std::io::Write;</code> | Bring the named items into scope; no chess search runs here. |
| 4 | <code>use std::sync::OnceLock;</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use std::sync::atomic::{AtomicBool, Ordering::Relaxed};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use std::time::{Instant, SystemTime, UNIX_EPOCH};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code></code> | Blank line separating related declarations or steps. |
| 8 | <code>pub static ENABLED: AtomicBool = AtomicBool::new(false);</code> | Define ENABLED, including its explicit type and initial value. |
| 9 | <code>static STARTED: OnceLock&lt;Instant&gt; = OnceLock::new();</code> | Define STARTED, including its explicit type and initial value. |
| 10 | <code></code> | Blank line separating related declarations or steps. |
| 11 | <code>/// Keep both diagnostic streams in one process session, independent of the GUI&#x27;s folder.</code> | Source explanation: Keep both diagnostic streams in one process session, independent of the GUI&#x27;s folder. |
| 12 | <code>fn log_path(kind: &amp;str) -&gt; std::io::Result&lt;std::path::PathBuf&gt; {</code> | Declare log_path(); its parameters and return type define the interface. |
| 13 | <code>    static SESSION: OnceLock&lt;u128&gt; = OnceLock::new();</code> | Define SESSION, including its explicit type and initial value. |
| 14 | <code>    let session = SESSION.get_or_init(&#124;&#124; {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 15 | <code>        SystemTime::now()</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 16 | <code>            .duration_since(UNIX_EPOCH)</code> | Evaluate or continue this expression in log_path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 17 | <code>            .unwrap_or_default()</code> | Evaluate or continue this expression in log_path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 18 | <code>            .as_nanos()</code> | Evaluate or continue this expression in log_path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 19 | <code>    });</code> | Evaluate or continue this expression in log_path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 20 | <code>    let folder = std::env::current_exe()?.with_file_name(&quot;logs&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 21 | <code>    std::fs::create_dir_all(&amp;folder)?;</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 22 | <code>    Ok(folder.join(format!(&quot;{kind}_{}_{session}.txt&quot;, std::process::id())))</code> | Evaluate or continue this expression in log_path(); a final expression without a semicolon supplies its block&#x27;s value. |
| 23 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 24 | <code></code> | Blank line separating related declarations or steps. |
| 25 | <code>/// Append beside the executable. Missing permissions or storage never stop play.</code> | Source explanation: Append beside the executable. Missing permissions or storage never stop play. |
| 26 | <code>pub fn log(text: &amp;str) {</code> | Declare log(); its parameters and return type define the interface. |
| 27 | <code>    if !ENABLED.load(Relaxed) {</code> | Test the condition and execute this branch only when it matches. |
| 28 | <code>        return;</code> | Leave the current function with the displayed result. |
| 29 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 30 | <code>    let started = *STARTED.get_or_init(Instant::now);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 31 | <code>    let Ok(path) = log_path(&quot;search&quot;) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 32 | <code>        return;</code> | Leave the current function with the displayed result. |
| 33 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 34 | <code>    if let Ok(mut file) = std::fs::OpenOptions::new()</code> | Test the condition and execute this branch only when it matches. |
| 35 | <code>        .create(true)</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>        .append(true)</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 37 | <code>        .open(path)</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>    {</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 39 | <code>        let unix_ms = SystemTime::now()</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 40 | <code>            .duration_since(UNIX_EPOCH)</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>            .unwrap_or_default()</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 42 | <code>            .as_millis();</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 43 | <code>        let _ = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 44 | <code>            file,</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 45 | <code>            &quot;{} {:&gt;11.1} unix_ms={unix_ms} {text}&quot;,</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code>            std::process::id(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 47 | <code>            started.elapsed().as_secs_f64() * 1000.0</code> | Evaluate or continue this expression in log(); a final expression without a semicolon supplies its block&#x27;s value. |
| 48 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 49 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 50 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code></code> | Blank line separating related declarations or steps. |
| 52 | <code>pub struct Diagnostics {</code> | Declare the struct Diagnostics used by this module. |
| 53 | <code>    search_id: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 54 | <code>    started: Instant,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 55 | <code>    pub completed_depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 56 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 57 | <code></code> | Blank line separating related declarations or steps. |
| 58 | <code>impl Diagnostics {</code> | Begin the implementation of methods or a trait for the named type. |
| 59 | <code>    pub fn new() -&gt; Self {</code> | Declare new(); its parameters and return type define the interface. |
| 60 | <code>        STARTED.get_or_init(Instant::now);</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>        Self {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>            search_id: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 63 | <code>            started: Instant::now(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 64 | <code>            completed_depth: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 65 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 66 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code></code> | Blank line separating related declarations or steps. |
| 68 | <code>    pub fn begin(&amp;mut self) {</code> | Declare begin(); its parameters and return type define the interface. |
| 69 | <code>        self.search_id += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 70 | <code>        self.started = Instant::now();</code> | Update the named state using the expression and assignment operator shown. |
| 71 | <code>        self.completed_depth = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 72 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code></code> | Blank line separating related declarations or steps. |
| 74 | <code>    pub fn report(&amp;self, text: &amp;str) {</code> | Declare report(); its parameters and return type define the interface. |
| 75 | <code>        if ENABLED.load(Relaxed) {</code> | Test the condition and execute this branch only when it matches. |
| 76 | <code>            let ms = self.started.elapsed().as_secs_f64() * 1000.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 77 | <code>            log(&amp;format!(</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 78 | <code>                &quot;search={} wall_ms={ms:.1} own_ms={ms:.1} completed_depth={} {text}&quot;,</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 79 | <code>                self.search_id, self.completed_depth</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 80 | <code>            ));</code> | Evaluate or continue this expression in report(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 82 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 83 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### evaluation.rs

Score material, piece placement, pawn structure, and king/endgame terms.

[Open source](src/evaluation.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Score a position in centipawns for the side to move. Every table and term matches</code> | Source explanation: Score a position in centipawns for the side to move. Every table and term matches |
| 2 | <code>//! evaluation.py in Remus4, and every division rounds down as Python&#x27;s // does.</code> | Source explanation: evaluation.py in Remus4, and every division rounds down as Python&#x27;s // does. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use crate::attacks::{KNIGHT_ATTACKS, bishop_attacks, rook_attacks};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use crate::board::{BISHOP, KING, KNIGHT, PAWN, Position, QUEEN, ROOK};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>/// Return a 64-square table built from files a to d of every rank.</code> | Source explanation: Return a 64-square table built from files a to d of every rank. |
| 8 | <code>const fn mirrored_table(left_half: [i32; 32]) -&gt; [i32; 64] {</code> | Declare mirrored_table(); its parameters and return type define the interface. |
| 9 | <code>    let columns = [0, 1, 2, 3, 3, 2, 1, 0];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 10 | <code>    let mut table = [0; 64];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 11 | <code>    let mut row = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 12 | <code>    while row &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 13 | <code>        let mut col = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 14 | <code>        while col &lt; 8 {</code> | Repeat this block while its condition remains true. |
| 15 | <code>            table[row * 8 + col] = left_half[row * 4 + columns[col]];</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 16 | <code>            col += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 17 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 18 | <code>        row += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 19 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 20 | <code>    table</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 21 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 22 | <code></code> | Blank line separating related declarations or steps. |
| 23 | <code>#[rustfmt::skip]</code> | Compiler attribute controlling the following declaration. |
| 24 | <code>static PAWN_TABLE: [i32; 64] = mirrored_table([</code> | Define PAWN_TABLE, including its explicit type and initial value. |
| 25 | <code>      0,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 26 | <code>     50,  50,  50,  50,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 27 | <code>     10,  10,  20,  30,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 28 | <code>      5,   5,  10,  25,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 29 | <code>      0,   0,   0,  20,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>      5,  -5, -10,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>      5,  10,  10, -20,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 32 | <code>      0,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 33 | <code>]);</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 34 | <code></code> | Blank line separating related declarations or steps. |
| 35 | <code>#[rustfmt::skip]</code> | Compiler attribute controlling the following declaration. |
| 36 | <code>static KNIGHT_TABLE: [i32; 64] = mirrored_table([</code> | Define KNIGHT_TABLE, including its explicit type and initial value. |
| 37 | <code>    -50, -40, -30, -30,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>    -40, -20,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 39 | <code>    -30,   0,  10,  15,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 40 | <code>    -30,   5,  15,  20,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>    -30,   0,  15,  20,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 42 | <code>    -30,   5,  10,  15,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 43 | <code>    -40, -20,   0,   5,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 44 | <code>    -50, -40, -30, -30,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 45 | <code>]);</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code></code> | Blank line separating related declarations or steps. |
| 47 | <code>#[rustfmt::skip]</code> | Compiler attribute controlling the following declaration. |
| 48 | <code>static BISHOP_TABLE: [i32; 64] = mirrored_table([</code> | Define BISHOP_TABLE, including its explicit type and initial value. |
| 49 | <code>    -20, -10, -10, -10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 50 | <code>    -10,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 51 | <code>    -10,   0,   5,  10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 52 | <code>    -10,   5,   5,  10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 53 | <code>    -10,   0,  10,  10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>    -10,  10,  10,  10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>    -10,   5,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 56 | <code>    -20, -10, -10, -10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 57 | <code>]);</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 58 | <code></code> | Blank line separating related declarations or steps. |
| 59 | <code>#[rustfmt::skip]</code> | Compiler attribute controlling the following declaration. |
| 60 | <code>static KING_MIDDLE_TABLE: [i32; 64] = mirrored_table([</code> | Define KING_MIDDLE_TABLE, including its explicit type and initial value. |
| 61 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 63 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>    -30, -40, -40, -50,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>    -20, -30, -30, -40,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 66 | <code>    -10, -20, -20, -20,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>     20,  20,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code>     20,  30,  10,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 69 | <code>]);</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 70 | <code></code> | Blank line separating related declarations or steps. |
| 71 | <code>#[rustfmt::skip]</code> | Compiler attribute controlling the following declaration. |
| 72 | <code>static ROOK_TABLE: [i32; 64] = mirrored_table([</code> | Define ROOK_TABLE, including its explicit type and initial value. |
| 73 | <code>      0,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 74 | <code>      5,  10,  10,  10,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 76 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 77 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 78 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 79 | <code>     -5,   0,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 80 | <code>      0,   0,   0,   5,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>]);</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 82 | <code></code> | Blank line separating related declarations or steps. |
| 83 | <code>#[rustfmt::skip]</code> | Compiler attribute controlling the following declaration. |
| 84 | <code>static KING_END_TABLE: [i32; 64] = mirrored_table([</code> | Define KING_END_TABLE, including its explicit type and initial value. |
| 85 | <code>    -50, -40, -30, -20,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 86 | <code>    -30, -20, -10,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 87 | <code>    -30, -10,  20,  30,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 88 | <code>    -30, -10,  30,  40,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 89 | <code>    -30, -10,  30,  40,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 90 | <code>    -30, -10,  20,  30,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 91 | <code>    -30, -30,   0,   0,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 92 | <code>    -50, -30, -30, -30,</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 93 | <code>]);</code> | Evaluate or continue this expression in mirrored_table(); a final expression without a semicolon supplies its block&#x27;s value. |
| 94 | <code></code> | Blank line separating related declarations or steps. |
| 95 | <code>// What each kind of piece is worth, from empty (0) to king (6)</code> | Source explanation: What each kind of piece is worth, from empty (0) to king (6) |
| 96 | <code>pub const PIECE_VALUES: [i32; 7] = [0, 100, 300, 300, 500, 900, 0];</code> | Define PIECE_VALUES, including its explicit type and initial value. |
| 97 | <code></code> | Blank line separating related declarations or steps. |
| 98 | <code>// The bonus of a passed pawn by the moves it needs to promote</code> | Source explanation: The bonus of a passed pawn by the moves it needs to promote |
| 99 | <code>const PASSED_BONUS: [i32; 7] = [0, 120, 80, 50, 30, 15, 10];</code> | Define PASSED_BONUS, including its explicit type and initial value. |
| 100 | <code>const MOBILITY_BONUS: i32 = 2; // Per square a knight, bishop, rook, or queen reaches</code> | Define MOBILITY_BONUS, including its explicit type and initial value. |
| 101 | <code>const BISHOP_PAIR: i32 = 30;</code> | Define BISHOP_PAIR, including its explicit type and initial value. |
| 102 | <code>const ROOK_OPEN_FILE: i32 = 20; // A file with no pawns</code> | Define ROOK_OPEN_FILE, including its explicit type and initial value. |
| 103 | <code>const ROOK_HALF_OPEN_FILE: i32 = 10; // A file with only enemy pawns</code> | Define ROOK_HALF_OPEN_FILE, including its explicit type and initial value. |
| 104 | <code>const DOUBLED_PAWN: i32 = 15; // For each extra pawn on a file</code> | Define DOUBLED_PAWN, including its explicit type and initial value. |
| 105 | <code>const ISOLATED_PAWN: i32 = 15; // No friendly pawn on a file beside it</code> | Define ISOLATED_PAWN, including its explicit type and initial value. |
| 106 | <code>const UNSTOPPABLE_PAWN: i32 = 500; // A passed pawn the enemy king cannot catch</code> | Define UNSTOPPABLE_PAWN, including its explicit type and initial value. |
| 107 | <code>const SHIELD_PAWN: i32 = 10; // For each pawn in front of a king on its back rank</code> | Define SHIELD_PAWN, including its explicit type and initial value. |
| 108 | <code>const OPEN_KING_FILE: i32 = 15; // For each file at the king with no friendly pawn</code> | Define OPEN_KING_FILE, including its explicit type and initial value. |
| 109 | <code></code> | Blank line separating related declarations or steps. |
| 110 | <code>const fn file_mask(col: i32) -&gt; u64 {</code> | Declare file_mask(); its parameters and return type define the interface. |
| 111 | <code>    if col &lt; 0 &#124;&#124; col &gt; 7 {</code> | Test the condition and execute this branch only when it matches. |
| 112 | <code>        0</code> | Evaluate or continue this expression in file_mask(); a final expression without a semicolon supplies its block&#x27;s value. |
| 113 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 114 | <code>        0x0101_0101_0101_0101_u64 &lt;&lt; col</code> | Evaluate or continue this expression in file_mask(); a final expression without a semicolon supplies its block&#x27;s value. |
| 115 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 116 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 117 | <code></code> | Blank line separating related declarations or steps. |
| 118 | <code>/// For each side and square, the squares where an enemy pawn can stop a pawn on that</code> | Source explanation: For each side and square, the squares where an enemy pawn can stop a pawn on that |
| 119 | <code>/// square: ahead of it, on its own file or a file beside it.</code> | Source explanation: square: ahead of it, on its own file or a file beside it. |
| 120 | <code>const fn build_passed_masks() -&gt; [[u64; 64]; 2] {</code> | Declare build_passed_masks(); its parameters and return type define the interface. |
| 121 | <code>    let mut table = [[0; 64]; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 122 | <code>    let mut square = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 123 | <code>    while square &lt; 64 {</code> | Repeat this block while its condition remains true. |
| 124 | <code>        let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 125 | <code>        let col = (square % 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 126 | <code>        let files = file_mask(col - 1) &#124; file_mask(col) &#124; file_mask(col + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 127 | <code>        // Row 0 is rank 8, so the rows ahead of a White pawn have smaller numbers</code> | Source explanation: Row 0 is rank 8, so the rows ahead of a White pawn have smaller numbers |
| 128 | <code>        let rows_above = if row == 0 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 129 | <code>            0</code> | Evaluate or continue this expression in build_passed_masks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 130 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 131 | <code>            u64::MAX &gt;&gt; (64 - 8 * row)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 132 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 133 | <code>        let rows_below = if row == 7 {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 134 | <code>            0</code> | Evaluate or continue this expression in build_passed_masks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 135 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 136 | <code>            u64::MAX &lt;&lt; (8 * (row + 1))</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 137 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 138 | <code>        table[0][square] = files &amp; rows_above;</code> | Update the named state using the expression and assignment operator shown. |
| 139 | <code>        table[1][square] = files &amp; rows_below;</code> | Update the named state using the expression and assignment operator shown. |
| 140 | <code>        square += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 141 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 142 | <code>    table</code> | Evaluate or continue this expression in build_passed_masks(); a final expression without a semicolon supplies its block&#x27;s value. |
| 143 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 144 | <code></code> | Blank line separating related declarations or steps. |
| 145 | <code>static PASSED_MASKS: [[u64; 64]; 2] = build_passed_masks();</code> | Define PASSED_MASKS, including its explicit type and initial value. |
| 146 | <code></code> | Blank line separating related declarations or steps. |
| 147 | <code>const SIGNS: [i32; 2] = [1, -1];</code> | Define SIGNS, including its explicit type and initial value. |
| 148 | <code></code> | Blank line separating related declarations or steps. |
| 149 | <code>/// Score the pawn shield of one king, larger with more pieces left.</code> | Source explanation: Score the pawn shield of one king, larger with more pieces left. |
| 150 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 151 | <code>fn king_safety(own_pawns: u64, counts: &amp;[i32; 10], king: i32, side: usize, phase: i32) -&gt; i32 {</code> | Declare king_safety(); its parameters and return type define the interface. |
| 152 | <code>    if king / 8 != (if side == 0 { 7 } else { 0 }) {</code> | Test the condition and execute this branch only when it matches. |
| 153 | <code>        return 0; // Only a king on its back rank has a shield</code> | Leave the current function with the displayed result. |
| 154 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>    let col = king % 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 156 | <code>    let forward = if side == 0 { -8 } else { 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 157 | <code>    let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>    for c in (col - 1).max(0)..(col + 2).min(8) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 159 | <code>        if own_pawns &amp; (1_u64 &lt;&lt; (king + forward + c - col)) != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 160 | <code>            score += SHIELD_PAWN;</code> | Update the named state using the expression and assignment operator shown. |
| 161 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 162 | <code>        if counts[(c + 1) as usize] == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 163 | <code>            score -= OPEN_KING_FILE;</code> | Update the named state using the expression and assignment operator shown. |
| 164 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 165 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 166 | <code>    (score * phase).div_euclid(24)</code> | Evaluate or continue this expression in king_safety(); a final expression without a semicolon supplies its block&#x27;s value. |
| 167 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 168 | <code></code> | Blank line separating related declarations or steps. |
| 169 | <code>/// Score the pawns, the rooks that use their files, and the pawn shield of each king,</code> | Source explanation: Score the pawns, the rooks that use their files, and the pawn shield of each king, |
| 170 | <code>/// from White&#x27;s side.</code> | Source explanation: from White&#x27;s side. |
| 171 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 172 | <code>fn pawn_terms(</code> | Declare pawn_terms(); its parameters and return type define the interface. |
| 173 | <code>    pawns: &amp;[u64; 2],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 174 | <code>    rooks: &amp;[u64; 2],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 175 | <code>    kings: &amp;[i32; 2],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 176 | <code>    phase: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 177 | <code>    white: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 178 | <code>) -&gt; i32 {</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 179 | <code>    let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 180 | <code>    // Entry col + 1 counts file col, so the a and h files have an empty file on each side</code> | Source explanation: Entry col + 1 counts file col, so the a and h files have an empty file on each side |
| 181 | <code>    let mut counts = [[0_i32; 10]; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 182 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 183 | <code>        let mut left = pawns[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 184 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 185 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 186 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 187 | <code>            counts[side][square % 8 + 1] += 1;</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 188 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 189 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 190 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 191 | <code>        let sign = SIGNS[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 192 | <code>        let enemy = 1 - side;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 193 | <code>        for count in counts[side] {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 194 | <code>            score -= sign * DOUBLED_PAWN * (count - 1).max(0);</code> | Update the named state using the expression and assignment operator shown. |
| 195 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 196 | <code>        let mut left = pawns[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 197 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 198 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 199 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 200 | <code>            let row = (square / 8) as i32;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 201 | <code>            let col = square % 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 202 | <code>            if counts[side][col] == 0 &amp;&amp; counts[side][col + 2] == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 203 | <code>                score -= sign * ISOLATED_PAWN;</code> | Update the named state using the expression and assignment operator shown. |
| 204 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 205 | <code>            if PASSED_MASKS[side][square] &amp; pawns[enemy] != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 206 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 207 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 208 | <code>            let steps = if side == 0 { row } else { 7 - row }; // Moves to promotion</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 209 | <code>            score += sign * PASSED_BONUS[steps as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 210 | <code>            // The pawn race: with no pieces left, a pawn whose promotion square the enemy</code> | Source explanation: The pawn race: with no pieces left, a pawn whose promotion square the enemy |
| 211 | <code>            // king cannot reach (the rule of the square) becomes a queen</code> | Source explanation: king cannot reach (the rule of the square) becomes a queen |
| 212 | <code>            let king = kings[enemy];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 213 | <code>            let mut king_steps = (king / 8 - 7 * side as i32)</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 214 | <code>                .abs()</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 215 | <code>                .max((king % 8 - col as i32).abs());</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 216 | <code>            if white != (side == 0) {</code> | Test the condition and execute this branch only when it matches. |
| 217 | <code>                king_steps -= 1; // The enemy king moves first</code> | Update the named state using the expression and assignment operator shown. |
| 218 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 219 | <code>            if phase == 0 &amp;&amp; king_steps &gt; steps.min(5) {</code> | Test the condition and execute this branch only when it matches. |
| 220 | <code>                score += sign * UNSTOPPABLE_PAWN;</code> | Update the named state using the expression and assignment operator shown. |
| 221 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 222 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code>        let mut left = rooks[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 224 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 225 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 226 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 227 | <code>            let entry = square % 8 + 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 228 | <code>            if counts[side][entry] == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 229 | <code>                let open_file = counts[enemy][entry] == 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 230 | <code>                score += sign</code> | Update the named state using the expression and assignment operator shown. |
| 231 | <code>                    * if open_file {</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 232 | <code>                        ROOK_OPEN_FILE</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 233 | <code>                    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 234 | <code>                        ROOK_HALF_OPEN_FILE</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 235 | <code>                    };</code> | Close the preceding block, expression, call, or collection. |
| 236 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 237 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 238 | <code>        score += sign * king_safety(pawns[side], &amp;counts[side], kings[side], side, phase);</code> | Update the named state using the expression and assignment operator shown. |
| 239 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 240 | <code>    score</code> | Evaluate or continue this expression in pawn_terms(); a final expression without a semicolon supplies its block&#x27;s value. |
| 241 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 242 | <code></code> | Blank line separating related declarations or steps. |
| 243 | <code>/// In an endgame where one side is ahead, reward pushing the losing king to the edge and</code> | Source explanation: In an endgame where one side is ahead, reward pushing the losing king to the edge and |
| 244 | <code>/// bringing the winning king close, from White&#x27;s side.</code> | Source explanation: bringing the winning king close, from White&#x27;s side. |
| 245 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 246 | <code>fn mating_bonus(material: i32, kings: &amp;[i32; 2], phase: i32) -&gt; i32 {</code> | Declare mating_bonus(); its parameters and return type define the interface. |
| 247 | <code>    if phase &gt; 8 &#124;&#124; material.abs() &lt; 400 {</code> | Test the condition and execute this branch only when it matches. |
| 248 | <code>        return 0;</code> | Leave the current function with the displayed result. |
| 249 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 250 | <code>    let winner = if material &gt; 0 { 0 } else { 1 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 251 | <code>    let loser_row = kings[1 - winner] / 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 252 | <code>    let loser_col = kings[1 - winner] % 8;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 253 | <code>    // 0 in the center, 6 in a corner</code> | Source explanation: 0 in the center, 6 in a corner |
| 254 | <code>    let edge = (3 - loser_row).max(loser_row - 4) + (3 - loser_col).max(loser_col - 4);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 255 | <code>    let distance = (kings[0] / 8 - kings[1] / 8)</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 256 | <code>        .abs()</code> | Evaluate or continue this expression in mating_bonus(); a final expression without a semicolon supplies its block&#x27;s value. |
| 257 | <code>        .max((kings[0] % 8 - kings[1] % 8).abs());</code> | Evaluate or continue this expression in mating_bonus(); a final expression without a semicolon supplies its block&#x27;s value. |
| 258 | <code>    let bonus = 10 * edge + 5 * (7 - distance);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 259 | <code>    if winner == 0 { bonus } else { -bonus }</code> | Test the condition and execute this branch only when it matches. |
| 260 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 261 | <code></code> | Blank line separating related declarations or steps. |
| 262 | <code>/// Return the score for the side to move: positive when it is ahead.</code> | Source explanation: Return the score for the side to move: positive when it is ahead. |
| 263 | <code>pub fn evaluate(position: &amp;Position) -&gt; i32 {</code> | Declare evaluate(); its parameters and return type define the interface. |
| 264 | <code>    let kinds = &amp;position.by_kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 265 | <code>    let occupied = position.occupied();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 266 | <code>    let mut score = 0; // From White&#x27;s side</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 267 | <code>    let mut material = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 268 | <code>    let mut phase = 0; // 24 with every piece on the board, 0 with none left</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 269 | <code>    let mut kings = [0_i32; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 270 | <code>    let mut pawns = [0_u64; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 271 | <code>    let mut rooks = [0_u64; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 272 | <code>    let mut bishop_count = [0_u32; 2];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 273 | <code></code> | Blank line separating related declarations or steps. |
| 274 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 275 | <code>        let sign = SIGNS[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 276 | <code>        let own = position.by_side[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 277 | <code>        // Flip the row for Black: Black&#x27;s a8 is White&#x27;s a1</code> | Source explanation: Flip the row for Black: Black&#x27;s a8 is White&#x27;s a1 |
| 278 | <code>        let flip = if side == 0 { 0 } else { 56 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 279 | <code>        let mut side_score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 280 | <code>        let mut reach_count = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 281 | <code></code> | Blank line separating related declarations or steps. |
| 282 | <code>        pawns[side] = own &amp; kinds[PAWN as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 283 | <code>        rooks[side] = own &amp; kinds[ROOK as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 284 | <code>        kings[side] = (own &amp; kinds[KING as usize]).trailing_zeros() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 285 | <code></code> | Blank line separating related declarations or steps. |
| 286 | <code>        let mut left = pawns[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 287 | <code>        material += sign * 100 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 288 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 289 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 290 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 291 | <code>            side_score += PAWN_TABLE[square ^ flip];</code> | Update the named state using the expression and assignment operator shown. |
| 292 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 293 | <code>        let mut left = own &amp; kinds[KNIGHT as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 294 | <code>        phase += left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 295 | <code>        material += sign * 300 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 296 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 297 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 298 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 299 | <code>            side_score += KNIGHT_TABLE[square ^ flip];</code> | Update the named state using the expression and assignment operator shown. |
| 300 | <code>            reach_count += (KNIGHT_ATTACKS[square] &amp; !own).count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 301 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 302 | <code>        let mut left = own &amp; kinds[BISHOP as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 303 | <code>        bishop_count[side] = left.count_ones();</code> | Update the named state using the expression and assignment operator shown. |
| 304 | <code>        phase += left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 305 | <code>        material += sign * 300 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 306 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 307 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 308 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 309 | <code>            side_score += BISHOP_TABLE[square ^ flip];</code> | Update the named state using the expression and assignment operator shown. |
| 310 | <code>            reach_count += (bishop_attacks(square, occupied) &amp; !own).count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 311 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 312 | <code>        let mut left = rooks[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 313 | <code>        phase += 2 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 314 | <code>        material += sign * 500 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 315 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 316 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 317 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 318 | <code>            side_score += ROOK_TABLE[square ^ flip];</code> | Update the named state using the expression and assignment operator shown. |
| 319 | <code>            reach_count += (rook_attacks(square, occupied) &amp; !own).count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 320 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 321 | <code>        let mut left = own &amp; kinds[QUEEN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 322 | <code>        phase += 4 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 323 | <code>        material += sign * 900 * left.count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 324 | <code>        while left != 0 {</code> | Repeat this block while its condition remains true. |
| 325 | <code>            let square = left.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 326 | <code>            left &amp;= left - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 327 | <code>            let reach = rook_attacks(square, occupied) &#124; bishop_attacks(square, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 328 | <code>            reach_count += (reach &amp; !own).count_ones() as i32;</code> | Update the named state using the expression and assignment operator shown. |
| 329 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 330 | <code>        score += sign * (side_score + MOBILITY_BONUS * reach_count);</code> | Update the named state using the expression and assignment operator shown. |
| 331 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 332 | <code></code> | Blank line separating related declarations or steps. |
| 333 | <code>    // Insufficient material: a lone knight or bishop cannot mate</code> | Source explanation: Insufficient material: a lone knight or bishop cannot mate |
| 334 | <code>    if phase &lt;= 1 &amp;&amp; pawns[0] == 0 &amp;&amp; pawns[1] == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 335 | <code>        return 0;</code> | Leave the current function with the displayed result. |
| 336 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 337 | <code>    let phase = phase.min(24);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 338 | <code>    for side in 0..2 {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 339 | <code>        let flip = if side == 0 { 0 } else { 56 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 340 | <code>        let view = kings[side] as usize ^ flip;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 341 | <code>        let taper = KING_MIDDLE_TABLE[view] * phase + KING_END_TABLE[view] * (24 - phase);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 342 | <code>        score += (SIGNS[side] * taper).div_euclid(24);</code> | Update the named state using the expression and assignment operator shown. |
| 343 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 344 | <code>    if bishop_count[0] &gt;= 2 {</code> | Test the condition and execute this branch only when it matches. |
| 345 | <code>        score += BISHOP_PAIR;</code> | Update the named state using the expression and assignment operator shown. |
| 346 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 347 | <code>    if bishop_count[1] &gt;= 2 {</code> | Test the condition and execute this branch only when it matches. |
| 348 | <code>        score -= BISHOP_PAIR;</code> | Update the named state using the expression and assignment operator shown. |
| 349 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 350 | <code>    score += material + pawn_terms(&amp;pawns, &amp;rooks, &amp;kings, phase, position.white);</code> | Update the named state using the expression and assignment operator shown. |
| 351 | <code>    score += mating_bonus(material, &amp;kings, phase);</code> | Update the named state using the expression and assignment operator shown. |
| 352 | <code>    if position.white { score } else { -score }</code> | Test the condition and execute this branch only when it matches. |
| 353 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### exchange.rs

Estimate a sequence of captures on one square for move ordering and pruning.

[Open source](src/exchange.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Static exchange evaluation (SEE): play out every capture on one square, cheapest</code> | Source explanation: Static exchange evaluation (SEE): play out every capture on one square, cheapest |
| 2 | <code>//! attacker first, and return what the first capture wins or loses.</code> | Source explanation: attacker first, and return what the first capture wins or loses. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use crate::attacks::{KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, bishop_attacks, rook_attacks};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use crate::board::{BISHOP, EMPTY, KING, KNIGHT, Move, PAWN, QUEEN, ROOK, WHITE};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code>use crate::board::{move_start, move_target};</code> | Bring the named items into scope; no chess search runs here. |
| 7 | <code>use crate::evaluation::PIECE_VALUES;</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code></code> | Blank line separating related declarations or steps. |
| 9 | <code>/// The board during the exchange, with only what the exchange needs.</code> | Source explanation: The board during the exchange, with only what the exchange needs. |
| 10 | <code>struct ExchangeBoard {</code> | Declare the struct ExchangeBoard used by this module. |
| 11 | <code>    board: [u8; 64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 12 | <code>    by_kind: [u64; 7],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 13 | <code>    by_side: [u64; 2],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 14 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 15 | <code></code> | Blank line separating related declarations or steps. |
| 16 | <code>impl ExchangeBoard {</code> | Begin the implementation of methods or a trait for the named type. |
| 17 | <code>    /// Lift the piece off the square.</code> | Source explanation: Lift the piece off the square. |
| 18 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 19 | <code>    fn remove(&amp;mut self, square: usize) {</code> | Declare remove(); its parameters and return type define the interface. |
| 20 | <code>        let piece = self.board[square];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 21 | <code>        let bit = 1_u64 &lt;&lt; square;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 22 | <code>        self.board[square] = EMPTY;</code> | Update the named state using the expression and assignment operator shown. |
| 23 | <code>        self.by_kind[(piece &amp; 7) as usize] &amp;= !bit;</code> | Evaluate or continue this expression in remove(); a final expression without a semicolon supplies its block&#x27;s value. |
| 24 | <code>        self.by_side[usize::from(piece &lt; WHITE)] &amp;= !bit;</code> | Evaluate or continue this expression in remove(); a final expression without a semicolon supplies its block&#x27;s value. |
| 25 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 26 | <code></code> | Blank line separating related declarations or steps. |
| 27 | <code>    /// Move the piece on start to the target square, capturing what stands there.</code> | Source explanation: Move the piece on start to the target square, capturing what stands there. |
| 28 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 29 | <code>    fn capture(&amp;mut self, start: usize, target: usize) {</code> | Declare capture(); its parameters and return type define the interface. |
| 30 | <code>        let piece = self.board[start];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 31 | <code>        let bit = 1_u64 &lt;&lt; target;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 32 | <code>        self.remove(target);</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 33 | <code>        self.remove(start);</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 34 | <code>        self.board[target] = piece;</code> | Update the named state using the expression and assignment operator shown. |
| 35 | <code>        self.by_kind[(piece &amp; 7) as usize] &#124;= bit;</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>        self.by_side[usize::from(piece &lt; WHITE)] &#124;= bit;</code> | Evaluate or continue this expression in capture(); a final expression without a semicolon supplies its block&#x27;s value. |
| 37 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 38 | <code></code> | Blank line separating related declarations or steps. |
| 39 | <code>    /// Return the square of the cheapest piece that attacks the square, or None. A</code> | Source explanation: Return the square of the cheapest piece that attacks the square, or None. A |
| 40 | <code>    /// knight and a bishop are worth the same, and the lower square number goes first.</code> | Source explanation: knight and a bishop are worth the same, and the lower square number goes first. |
| 41 | <code>    #[inline]</code> | Compiler attribute controlling the following declaration. |
| 42 | <code>    fn cheapest_attacker(&amp;self, square: usize, by_white: bool) -&gt; Option&lt;usize&gt; {</code> | Declare cheapest_attacker(); its parameters and return type define the interface. |
| 43 | <code>        let side = usize::from(!by_white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 44 | <code>        let own = self.by_side[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 45 | <code>        let kinds = &amp;self.by_kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>        let occupied = self.by_side[0] &#124; self.by_side[1];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>        let found = PAWN_ATTACKS[1 - side][square] &amp; own &amp; kinds[PAWN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 48 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 49 | <code>            return Some(found.trailing_zeros() as usize);</code> | Leave the current function with the displayed result. |
| 50 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code>        let bishop_lines = bishop_attacks(square, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 52 | <code>        let found = own</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 53 | <code>            &amp; ((KNIGHT_ATTACKS[square] &amp; kinds[KNIGHT as usize])</code> | Evaluate or continue this expression in cheapest_attacker(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>                &#124; (bishop_lines &amp; kinds[BISHOP as usize]));</code> | Evaluate or continue this expression in cheapest_attacker(); a final expression without a semicolon supplies its block&#x27;s value. |
| 55 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 56 | <code>            return Some(found.trailing_zeros() as usize);</code> | Leave the current function with the displayed result. |
| 57 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 58 | <code>        let rook_lines = rook_attacks(square, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 59 | <code>        let found = own &amp; rook_lines &amp; kinds[ROOK as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 60 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>            return Some(found.trailing_zeros() as usize);</code> | Leave the current function with the displayed result. |
| 62 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 63 | <code>        let found = own &amp; (rook_lines &#124; bishop_lines) &amp; kinds[QUEEN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 64 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 65 | <code>            return Some(found.trailing_zeros() as usize);</code> | Leave the current function with the displayed result. |
| 66 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code>        let found = own &amp; KING_ATTACKS[square] &amp; kinds[KING as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 68 | <code>        if found != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 69 | <code>            return Some(found.trailing_zeros() as usize);</code> | Leave the current function with the displayed result. |
| 70 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 71 | <code>        None</code> | Evaluate or continue this expression in cheapest_attacker(); a final expression without a semicolon supplies its block&#x27;s value. |
| 72 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code></code> | Blank line separating related declarations or steps. |
| 75 | <code>const KING_VALUE: i32 = 2000; // The king captures last: losing it ends the game</code> | Define KING_VALUE, including its explicit type and initial value. |
| 76 | <code></code> | Blank line separating related declarations or steps. |
| 77 | <code>/// Return the material the capture wins when both sides keep recapturing.</code> | Source explanation: Return the material the capture wins when both sides keep recapturing. |
| 78 | <code>pub fn exchange_score(board: &amp;[u8; 64], by_kind: &amp;[u64; 7], by_side: &amp;[u64; 2], mv: Move) -&gt; i32 {</code> | Declare exchange_score(); its parameters and return type define the interface. |
| 79 | <code>    let start = move_start(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 80 | <code>    let target = move_target(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 81 | <code>    let mut exchange = ExchangeBoard {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 82 | <code>        board: *board,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 83 | <code>        by_kind: *by_kind,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 84 | <code>        by_side: *by_side,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 85 | <code>    };</code> | Close the preceding block, expression, call, or collection. |
| 86 | <code>    // gains[i] is what the side making capture i has won after it</code> | Source explanation: gains[i] is what the side making capture i has won after it |
| 87 | <code>    let mut gains = [0_i32; 40];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 88 | <code>    let mut count = 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 89 | <code>    gains[0] = PIECE_VALUES[(exchange.board[target] &amp; 7) as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 90 | <code>    let mut white = exchange.board[start] &gt; WHITE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 91 | <code>    exchange.capture(start, target);</code> | Evaluate or continue this expression in exchange_score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 92 | <code>    let mut attacker = exchange.cheapest_attacker(target, !white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 93 | <code>    while let Some(from) = attacker {</code> | Repeat this block while its condition remains true. |
| 94 | <code>        // The king may not capture onto a defended square</code> | Source explanation: The king may not capture onto a defended square |
| 95 | <code>        if exchange.board[target] &amp; 7 == KING {</code> | Test the condition and execute this branch only when it matches. |
| 96 | <code>            if count == 1 {</code> | Test the condition and execute this branch only when it matches. |
| 97 | <code>                return -KING_VALUE;</code> | Leave the current function with the displayed result. |
| 98 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 99 | <code>            count -= 1;</code> | Update the named state using the expression and assignment operator shown. |
| 100 | <code>            break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 101 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 102 | <code>        gains[count] = PIECE_VALUES[(exchange.board[target] &amp; 7) as usize] - gains[count - 1];</code> | Update the named state using the expression and assignment operator shown. |
| 103 | <code>        count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 104 | <code>        exchange.capture(from, target);</code> | Evaluate or continue this expression in exchange_score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 105 | <code>        white = !white;</code> | Update the named state using the expression and assignment operator shown. |
| 106 | <code>        attacker = exchange.cheapest_attacker(target, !white);</code> | Update the named state using the expression and assignment operator shown. |
| 107 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 108 | <code>    // Work back from the last capture: each side may stop capturing</code> | Source explanation: Work back from the last capture: each side may stop capturing |
| 109 | <code>    for i in (1..count).rev() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 110 | <code>        gains[i - 1] = -(-gains[i - 1]).max(gains[i]);</code> | Evaluate or continue this expression in exchange_score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 111 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 112 | <code>    gains[0]</code> | Evaluate or continue this expression in exchange_score(); a final expression without a semicolon supplies its block&#x27;s value. |
| 113 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### main.rs

Read UCI commands, maintain game state, choose legal root moves, and report results.

[Open source](src/main.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Talk to a chess GUI such as Lucas Chess through the Universal Chess Interface (UCI).</code> | Source explanation: Talk to a chess GUI such as Lucas Chess through the Universal Chess Interface (UCI). |
| 2 | <code>//! The GUI sends one command per line, and the engine answers each one. Besides what</code> | Source explanation: The GUI sends one command per line, and the engine answers each one. Besides what |
| 3 | <code>//! Lucas Chess sends, &quot;position fen&quot;, &quot;go depth&quot;, &quot;go perft&quot;, and &quot;eval&quot; serve the</code> | Source explanation: Lucas Chess sends, &quot;position fen&quot;, &quot;go depth&quot;, &quot;go perft&quot;, and &quot;eval&quot; serve the |
| 4 | <code>//! workshop designer&#x27;s check that this engine matches Remus4.</code> | Source explanation: workshop designer&#x27;s check that this engine matches Remus4. |
| 5 | <code></code> | Blank line separating related declarations or steps. |
| 6 | <code>mod attacks;</code> | Include a sibling Rust module in this program. |
| 7 | <code>mod board;</code> | Include a sibling Rust module in this program. |
| 8 | <code>mod diagnostics;</code> | Include a sibling Rust module in this program. |
| 9 | <code>mod evaluation;</code> | Include a sibling Rust module in this program. |
| 10 | <code>mod exchange;</code> | Include a sibling Rust module in this program. |
| 11 | <code>mod make_move;</code> | Include a sibling Rust module in this program. |
| 12 | <code>mod move_generation;</code> | Include a sibling Rust module in this program. |
| 13 | <code>mod opening_book;</code> | Include a sibling Rust module in this program. |
| 14 | <code>mod search;</code> | Include a sibling Rust module in this program. |
| 15 | <code>mod table_metrics;</code> | Include a sibling Rust module in this program. |
| 16 | <code>mod transposition_table;</code> | Include a sibling Rust module in this program. |
| 17 | <code></code> | Blank line separating related declarations or steps. |
| 18 | <code>use std::io::{BufRead, Write};</code> | Bring the named items into scope; no chess search runs here. |
| 19 | <code></code> | Blank line separating related declarations or steps. |
| 20 | <code>use attacks::in_check;</code> | Bring the named items into scope; no chess search runs here. |
| 21 | <code>use board::{Move, NO_MOVE, Position, move_text, new_move, parse_square};</code> | Bring the named items into scope; no chess search runs here. |
| 22 | <code>use evaluation::evaluate;</code> | Bring the named items into scope; no chess search runs here. |
| 23 | <code>use make_move::make_move;</code> | Bring the named items into scope; no chess search runs here. |
| 24 | <code>use move_generation::{MoveList, generate_moves};</code> | Bring the named items into scope; no chess search runs here. |
| 25 | <code>use opening_book::{BookLine, book_moves, load_book};</code> | Bring the named items into scope; no chess search runs here. |
| 26 | <code>use search::Search;</code> | Bring the named items into scope; no chess search runs here. |
| 27 | <code></code> | Blank line separating related declarations or steps. |
| 28 | <code>const STACK_BYTES: usize = 64 * 1024 * 1024; // Room for the deepest search</code> | Define STACK_BYTES, including its explicit type and initial value. |
| 29 | <code></code> | Blank line separating related declarations or steps. |
| 30 | <code>/// Return the number after a word of &quot;go&quot;, such as wtime, or 0.</code> | Source explanation: Return the number after a word of &quot;go&quot;, such as wtime, or 0. |
| 31 | <code>fn read_number(words: &amp;[&amp;str], name: &amp;str) -&gt; i64 {</code> | Declare read_number(); its parameters and return type define the interface. |
| 32 | <code>    for i in 0..words.len().saturating_sub(1) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 33 | <code>        if words[i] == name {</code> | Test the condition and execute this branch only when it matches. |
| 34 | <code>            return words[i + 1].parse().unwrap_or(0);</code> | Leave the current function with the displayed result. |
| 35 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 36 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 37 | <code>    0</code> | Evaluate or continue this expression in read_number(); a final expression without a semicolon supplies its block&#x27;s value. |
| 38 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 39 | <code></code> | Blank line separating related declarations or steps. |
| 40 | <code>/// Return the move that UCI text such as &quot;e7e8q&quot; names.</code> | Source explanation: Return the move that UCI text such as &quot;e7e8q&quot; names. |
| 41 | <code>fn parse_move(text: &amp;str) -&gt; Move {</code> | Declare parse_move(); its parameters and return type define the interface. |
| 42 | <code>    let promotion = text</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 43 | <code>        .chars()</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 44 | <code>        .nth(4)</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 45 | <code>        .and_then(&#124;letter&#124; &quot;.pnbrqk&quot;.find(letter))</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 46 | <code>        .unwrap_or(0) as u8;</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 47 | <code>    new_move(</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 48 | <code>        parse_square(&amp;text[0..2]),</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 49 | <code>        parse_square(&amp;text[2..4]),</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 50 | <code>        promotion,</code> | Evaluate or continue this expression in parse_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 51 | <code>    )</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 53 | <code></code> | Blank line separating related declarations or steps. |
| 54 | <code>/// Return a legal move from the opening book, or NO_MOVE.</code> | Source explanation: Return a legal move from the opening book, or NO_MOVE. |
| 55 | <code>fn legal_book_move(book: &amp;[BookLine], position: &amp;Position, played: &amp;[&amp;str]) -&gt; Move {</code> | Declare legal_book_move(); its parameters and return type define the interface. |
| 56 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 57 | <code>    generate_moves(position, &amp;mut list);</code> | Evaluate or continue this expression in legal_book_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 58 | <code>    for text in book_moves(book, played, position.white) {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 59 | <code>        for &amp;mut mv in list.as_mut_slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 60 | <code>            if move_text(mv) == text &amp;&amp; !in_check(&amp;make_move(position, mv), position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>                return mv;</code> | Leave the current function with the displayed result. |
| 62 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 63 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 64 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 65 | <code>    NO_MOVE</code> | Evaluate or continue this expression in legal_book_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 66 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code></code> | Blank line separating related declarations or steps. |
| 68 | <code>/// Count the leaf positions of the legal move tree.</code> | Source explanation: Count the leaf positions of the legal move tree. |
| 69 | <code>fn perft(position: &amp;Position, depth: u32) -&gt; u64 {</code> | Declare perft(); its parameters and return type define the interface. |
| 70 | <code>    if depth == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 71 | <code>        return 1;</code> | Leave the current function with the displayed result. |
| 72 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 74 | <code>    generate_moves(position, &amp;mut list);</code> | Evaluate or continue this expression in perft(); a final expression without a semicolon supplies its block&#x27;s value. |
| 75 | <code>    let mut count = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 76 | <code>    for &amp;mut mv in list.as_mut_slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 77 | <code>        let child = make_move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 78 | <code>        if !in_check(&amp;child, position.white) {</code> | Test the condition and execute this branch only when it matches. |
| 79 | <code>            count += perft(&amp;child, depth - 1);</code> | Update the named state using the expression and assignment operator shown. |
| 80 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 81 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 82 | <code>    count</code> | Evaluate or continue this expression in perft(); a final expression without a semicolon supplies its block&#x27;s value. |
| 83 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 84 | <code></code> | Blank line separating related declarations or steps. |
| 85 | <code>fn run() {</code> | Declare run(); its parameters and return type define the interface. |
| 86 | <code>    let book = load_book();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 87 | <code>    let mut search = Search::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 88 | <code>    let mut position = Position::start();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 89 | <code>    let mut played: Vec&lt;String&gt; = Vec::new(); // The moves of a game from the start</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 90 | <code>    let mut use_book = true;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 91 | <code>    let mut game_history: Vec&lt;u64&gt; = Vec::new(); // The keys of the positions before this one</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 92 | <code>    let mut out = std::io::stdout();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 93 | <code></code> | Blank line separating related declarations or steps. |
| 94 | <code>    for line in std::io::stdin().lock().lines() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 95 | <code>        let Ok(line) = line else { break };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 96 | <code>        diagnostics::log(&amp;format!(&quot;&lt; {line}&quot;));</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 97 | <code>        let words: Vec&lt;&amp;str&gt; = line.split_whitespace().collect();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 98 | <code>        if words.is_empty() {</code> | Test the condition and execute this branch only when it matches. |
| 99 | <code>            continue;</code> | Skip the rest of this iteration and advance the loop. |
| 100 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 101 | <code>        match words[0] {</code> | Select an arm by matching the expression against the listed patterns. |
| 102 | <code>            &quot;uci&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 103 | <code>                let _ = writeln!(out, &quot;id name Remus5&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 104 | <code>                let _ = writeln!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 105 | <code>                    out,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 106 | <code>                    &quot;option name SearchDiagnostics type check default false&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 107 | <code>                );</code> | Close the preceding block, expression, call, or collection. |
| 108 | <code>                let _ = writeln!(out, &quot;uciok&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 109 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 110 | <code>            &quot;isready&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 111 | <code>                let _ = writeln!(out, &quot;readyok&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 112 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 113 | <code>            &quot;setoption&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 114 | <code>                if words.len() == 5</code> | Test the condition and execute this branch only when it matches. |
| 115 | <code>                    &amp;&amp; words[1] == &quot;name&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 116 | <code>                    &amp;&amp; words[2].eq_ignore_ascii_case(&quot;SearchDiagnostics&quot;)</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 117 | <code>                    &amp;&amp; words[3] == &quot;value&quot;</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 118 | <code>                {</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 119 | <code>                    diagnostics::ENABLED.store(</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 120 | <code>                        words[4].eq_ignore_ascii_case(&quot;true&quot;),</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 121 | <code>                        std::sync::atomic::Ordering::Relaxed,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 122 | <code>                    );</code> | Close the preceding block, expression, call, or collection. |
| 123 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 124 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 125 | <code>            &quot;position&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 126 | <code>                // &quot;position startpos moves e2e4 e7e5&quot; or &quot;position fen ... moves e2e4&quot;</code> | Source explanation: &quot;position startpos moves e2e4 e7e5&quot; or &quot;position fen ... moves e2e4&quot; |
| 127 | <code>                let moves_at = words.iter().position(&#124;word&#124; *word == &quot;moves&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 128 | <code>                use_book = words.get(1) == Some(&amp;&quot;startpos&quot;);</code> | Update the named state using the expression and assignment operator shown. |
| 129 | <code>                position = if use_book {</code> | Update the named state using the expression and assignment operator shown. |
| 130 | <code>                    Position::start()</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 131 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 132 | <code>                    Position::from_fen(&amp;words[2..moves_at.unwrap_or(words.len())])</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 133 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 134 | <code>                played.clear();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 135 | <code>                game_history.clear();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 136 | <code>                if let Some(at) = moves_at {</code> | Test the condition and execute this branch only when it matches. |
| 137 | <code>                    for text in &amp;words[at + 1..] {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 138 | <code>                        game_history.push(position.key);</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 139 | <code>                        played.push((*text).to_string());</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 140 | <code>                        position = make_move(&amp;position, parse_move(text));</code> | Update the named state using the expression and assignment operator shown. |
| 141 | <code>                    }</code> | Close the preceding block, expression, call, or collection. |
| 142 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 143 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 144 | <code>            &quot;go&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 145 | <code>                if words.contains(&amp;&quot;perft&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 146 | <code>                    let count = perft(&amp;position, read_number(&amp;words, &quot;perft&quot;) as u32);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 147 | <code>                    let _ = writeln!(out, &quot;perft {count}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 148 | <code>                    continue;</code> | Skip the rest of this iteration and advance the loop. |
| 149 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 150 | <code>                let depth_limit = if words.contains(&amp;&quot;depth&quot;) {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 151 | <code>                    Some(read_number(&amp;words, &quot;depth&quot;) as i32)</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 153 | <code>                    None</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 154 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>                let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 156 | <code>                let clock_left = read_number(&amp;words, if white { &quot;wtime&quot; } else { &quot;btime&quot; }) as f64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 157 | <code>                let increment = read_number(&amp;words, if white { &quot;winc&quot; } else { &quot;binc&quot; }) as f64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>                // Plan a thirtieth of the clock plus half the increment, and never more</code> | Source explanation: Plan a thirtieth of the clock plus half the increment, and never more |
| 159 | <code>                // than three times that or a fifth of the clock</code> | Source explanation: than three times that or a fifth of the clock |
| 160 | <code>                let mut seconds = 2.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 161 | <code>                let mut longest = 2.0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 162 | <code>                if clock_left &gt; 0.0 {</code> | Test the condition and execute this branch only when it matches. |
| 163 | <code>                    seconds = (clock_left / 30.0 + increment / 2.0) / 1000.0;</code> | Update the named state using the expression and assignment operator shown. |
| 164 | <code>                    longest = (3.0 * seconds).min(clock_left / 5000.0);</code> | Update the named state using the expression and assignment operator shown. |
| 165 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 166 | <code>                if words.contains(&amp;&quot;movetime&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 167 | <code>                    seconds = read_number(&amp;words, &quot;movetime&quot;) as f64 / 1000.0;</code> | Update the named state using the expression and assignment operator shown. |
| 168 | <code>                    longest = seconds;</code> | Update the named state using the expression and assignment operator shown. |
| 169 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 170 | <code>                search.diagnostics.begin();</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 171 | <code>                search.diagnostics.report(&amp;format!(</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 172 | <code>                    &quot;begin root_key={:016x} white={} halfmove={} history_plies={} plan_s={seconds:.3} hard_s={longest:.3} command={line}&quot;,</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 173 | <code>                    position.key, position.white, position.halfmove, game_history.len()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 174 | <code>                ));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 175 | <code>                let mut mv = NO_MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 176 | <code>                let mut source = &quot;book&quot;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 177 | <code>                if use_book &amp;&amp; depth_limit.is_none() {</code> | Test the condition and execute this branch only when it matches. |
| 178 | <code>                    let texts: Vec&lt;&amp;str&gt; = played.iter().map(String::as_str).collect();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 179 | <code>                    mv = legal_book_move(&amp;book, &amp;position, &amp;texts);</code> | Update the named state using the expression and assignment operator shown. |
| 180 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 181 | <code>                if mv == NO_MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 182 | <code>                    source = &quot;native&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 183 | <code>                    mv = search.think(&amp;position, &amp;game_history, seconds, longest, depth_limit);</code> | Update the named state using the expression and assignment operator shown. |
| 184 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 185 | <code>                // With no legal move (checkmate or stalemate) UCI says &quot;0000&quot;</code> | Source explanation: With no legal move (checkmate or stalemate) UCI says &quot;0000&quot; |
| 186 | <code>                let text = if mv == NO_MOVE {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 187 | <code>                    &quot;0000&quot;.to_string()</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 188 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 189 | <code>                    move_text(mv)</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 190 | <code>                };</code> | Close the preceding block, expression, call, or collection. |
| 191 | <code>                let _ = writeln!(out, &quot;bestmove {text}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 192 | <code>                let _ = out.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 193 | <code>                search</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 194 | <code>                    .diagnostics</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 195 | <code>                    .report(&amp;format!(&quot;result source={source} bestmove {text}&quot;));</code> | Evaluate or continue this expression in run(); a final expression without a semicolon supplies its block&#x27;s value. |
| 196 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 197 | <code>            &quot;eval&quot; =&gt; {</code> | Handle this matching pattern with the expression or block on its right. |
| 198 | <code>                let _ = writeln!(out, &quot;eval {}&quot;, evaluate(&amp;position));</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 199 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 200 | <code>            &quot;quit&quot; =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 201 | <code>            _ =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 202 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 203 | <code>        let _ = out.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 204 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 205 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 206 | <code></code> | Blank line separating related declarations or steps. |
| 207 | <code>fn main() {</code> | Declare main(); its parameters and return type define the interface. |
| 208 | <code>    // The search calls itself once per ply, so it runs on a thread with a large stack</code> | Source explanation: The search calls itself once per ply, so it runs on a thread with a large stack |
| 209 | <code>    let worker = std::thread::Builder::new()</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 210 | <code>        .stack_size(STACK_BYTES)</code> | Evaluate or continue this expression in main(); a final expression without a semicolon supplies its block&#x27;s value. |
| 211 | <code>        .spawn(run);</code> | Evaluate or continue this expression in main(); a final expression without a semicolon supplies its block&#x27;s value. |
| 212 | <code>    if let Ok(handle) = worker {</code> | Test the condition and execute this branch only when it matches. |
| 213 | <code>        let _ = handle.join();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 214 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 215 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### make_move.rs

Copy a position and apply captures, castling, promotion, and state changes.

[Open source](src/make_move.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Play a move. As in Remus4, make_move() copies the position, plays the move on the</code> | Source explanation: Play a move. As in Remus4, make_move() copies the position, plays the move on the |
| 2 | <code>//! copy, and returns the new position, so the search never takes a move back.</code> | Source explanation: copy, and returns the new position, so the search never takes a move back. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use crate::board::{EMPTY, KEYS, KING, Move, PAWN, Position, WHITE};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use crate::board::{move_promotion, move_start, move_target};</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>// The castling rights that a move from or to each square turns off for good</code> | Source explanation: The castling rights that a move from or to each square turns off for good |
| 8 | <code>const fn build_rights() -&gt; [u8; 64] {</code> | Declare build_rights(); its parameters and return type define the interface. |
| 9 | <code>    let mut table = [0; 64];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 10 | <code>    table[60] = 3; // e1</code> | Update the named state using the expression and assignment operator shown. |
| 11 | <code>    table[63] = 1; // h1</code> | Update the named state using the expression and assignment operator shown. |
| 12 | <code>    table[56] = 2; // a1</code> | Update the named state using the expression and assignment operator shown. |
| 13 | <code>    table[4] = 12; // e8</code> | Update the named state using the expression and assignment operator shown. |
| 14 | <code>    table[7] = 4; // h8</code> | Update the named state using the expression and assignment operator shown. |
| 15 | <code>    table[0] = 8; // a8</code> | Update the named state using the expression and assignment operator shown. |
| 16 | <code>    table</code> | Evaluate or continue this expression in build_rights(); a final expression without a semicolon supplies its block&#x27;s value. |
| 17 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 18 | <code></code> | Blank line separating related declarations or steps. |
| 19 | <code>static RIGHTS_OF_SQUARE: [u8; 64] = build_rights();</code> | Define RIGHTS_OF_SQUARE, including its explicit type and initial value. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>/// Return the position after the move, including the special moves.</code> | Source explanation: Return the position after the move, including the special moves. |
| 22 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 23 | <code>pub fn make_move(position: &amp;Position, mv: Move) -&gt; Position {</code> | Declare make_move(); its parameters and return type define the interface. |
| 24 | <code>    let mut next = *position;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 25 | <code>    let start = move_start(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 26 | <code>    let target = move_target(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 27 | <code>    let promotion = move_promotion(mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 28 | <code>    let mut piece = next.board[start];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 29 | <code>    let kind = piece &amp; 7;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 30 | <code>    let is_pawn = kind == PAWN;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 31 | <code>    let distance = start.abs_diff(target);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 32 | <code></code> | Blank line separating related declarations or steps. |
| 33 | <code>    // A capture or pawn move restarts the fifty-move count</code> | Source explanation: A capture or pawn move restarts the fifty-move count |
| 34 | <code>    if is_pawn &#124;&#124; next.board[target] != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 35 | <code>        next.halfmove = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 36 | <code>    } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 37 | <code>        next.halfmove += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 38 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 39 | <code>    // En passant: the captured pawn stands beside the pawn that moves</code> | Source explanation: En passant: the captured pawn stands beside the pawn that moves |
| 40 | <code>    if is_pawn &amp;&amp; target as i8 == next.en_passant {</code> | Test the condition and execute this branch only when it matches. |
| 41 | <code>        next.remove_piece(start / 8 * 8 + target % 8);</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 42 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 43 | <code>    // Castling: the king moves two squares, and the rook jumps to the square it crossed</code> | Source explanation: Castling: the king moves two squares, and the rook jumps to the square it crossed |
| 44 | <code>    if kind == KING &amp;&amp; distance == 2 {</code> | Test the condition and execute this branch only when it matches. |
| 45 | <code>        let rook = if target &gt; start { start + 3 } else { start - 4 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>        let rook_piece = next.board[rook];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 47 | <code>        next.remove_piece(rook);</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 48 | <code>        next.put_piece((start + target) / 2, rook_piece);</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 49 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 50 | <code>    // A two-square pawn move lets an enemy pawn capture it en passant on the next move</code> | Source explanation: A two-square pawn move lets an enemy pawn capture it en passant on the next move |
| 51 | <code>    if next.en_passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 52 | <code>        next.key ^= KEYS.en_passant[next.en_passant as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 54 | <code>    next.en_passant = -1;</code> | Update the named state using the expression and assignment operator shown. |
| 55 | <code>    if is_pawn &amp;&amp; distance == 16 {</code> | Test the condition and execute this branch only when it matches. |
| 56 | <code>        let skipped = (start + target) / 2;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 57 | <code>        next.en_passant = skipped as i8;</code> | Update the named state using the expression and assignment operator shown. |
| 58 | <code>        next.key ^= KEYS.en_passant[skipped];</code> | Update the named state using the expression and assignment operator shown. |
| 59 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 60 | <code>    if promotion != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>        piece = promotion + (piece &amp; WHITE);</code> | Update the named state using the expression and assignment operator shown. |
| 62 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 63 | <code>    if next.board[target] != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 64 | <code>        next.remove_piece(target);</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 66 | <code>    next.remove_piece(start);</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>    next.put_piece(target, piece);</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 68 | <code></code> | Blank line separating related declarations or steps. |
| 69 | <code>    let rights = next.castling &amp; !(RIGHTS_OF_SQUARE[start] &#124; RIGHTS_OF_SQUARE[target]);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 70 | <code>    if rights != next.castling {</code> | Test the condition and execute this branch only when it matches. |
| 71 | <code>        next.key ^= KEYS.castling[next.castling as usize] ^ KEYS.castling[rights as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 72 | <code>        next.castling = rights;</code> | Update the named state using the expression and assignment operator shown. |
| 73 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code>    next.white = !next.white;</code> | Update the named state using the expression and assignment operator shown. |
| 75 | <code>    next.key ^= KEYS.black;</code> | Update the named state using the expression and assignment operator shown. |
| 76 | <code>    next</code> | Evaluate or continue this expression in make_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 77 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 78 | <code></code> | Blank line separating related declarations or steps. |
| 79 | <code>/// Return the position with the other side to move and no en passant square.</code> | Source explanation: Return the position with the other side to move and no en passant square. |
| 80 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 81 | <code>pub fn make_null_move(position: &amp;Position) -&gt; Position {</code> | Declare make_null_move(); its parameters and return type define the interface. |
| 82 | <code>    let mut next = *position;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 83 | <code>    if next.en_passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 84 | <code>        next.key ^= KEYS.en_passant[next.en_passant as usize];</code> | Update the named state using the expression and assignment operator shown. |
| 85 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 86 | <code>    next.en_passant = -1;</code> | Update the named state using the expression and assignment operator shown. |
| 87 | <code>    next.halfmove += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 88 | <code>    next.white = !next.white;</code> | Update the named state using the expression and assignment operator shown. |
| 89 | <code>    next.key ^= KEYS.black;</code> | Update the named state using the expression and assignment operator shown. |
| 90 | <code>    next</code> | Evaluate or continue this expression in make_null_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 91 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### move_generation.rs

Generate candidate moves before the native king-safety filter.

[Open source](src/move_generation.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! List the moves of the side to move. As in Remus4 the list holds pseudo-legal moves:</code> | Source explanation: List the moves of the side to move. As in Remus4 the list holds pseudo-legal moves: |
| 2 | <code>//! some may leave the mover&#x27;s own king in check, and the search skips those after it</code> | Source explanation: some may leave the mover&#x27;s own king in check, and the search skips those after it |
| 3 | <code>//! makes them. The search sorts each list, so the order of generation does not matter.</code> | Source explanation: makes them. The search sorts each list, so the order of generation does not matter. |
| 4 | <code></code> | Blank line separating related declarations or steps. |
| 5 | <code>use std::mem::MaybeUninit;</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>use crate::attacks::{</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code>    KING_ATTACKS, KNIGHT_ATTACKS, PAWN_ATTACKS, bishop_attacks, is_attacked, rook_attacks,</code> | Continue the enclosing declaration or initialization expression. |
| 9 | <code>};</code> | Close the preceding block, expression, call, or collection. |
| 10 | <code>use crate::board::{BISHOP, EMPTY, KING, KNIGHT, Move, PAWN, Position, QUEEN, ROOK, new_move};</code> | Bring the named items into scope; no chess search runs here. |
| 11 | <code></code> | Blank line separating related declarations or steps. |
| 12 | <code>/// A fixed-size list on the stack whose unused slots stay unwritten. The search makes</code> | Source explanation: A fixed-size list on the stack whose unused slots stay unwritten. The search makes |
| 13 | <code>/// two of these at every position, and most positions use only a few slots, so filling</code> | Source explanation: two of these at every position, and most positions use only a few slots, so filling |
| 14 | <code>/// them with zeros first would cost more than the moves themselves.</code> | Source explanation: them with zeros first would cost more than the moves themselves. |
| 15 | <code>pub struct StackList&lt;T: Copy, const SIZE: usize&gt; {</code> | Declare the struct StackList used by this module. |
| 16 | <code>    items: [MaybeUninit&lt;T&gt;; SIZE],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 17 | <code>    count: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 18 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 19 | <code></code> | Blank line separating related declarations or steps. |
| 20 | <code>impl&lt;T: Copy, const SIZE: usize&gt; StackList&lt;T, SIZE&gt; {</code> | Begin the implementation of methods or a trait for the named type. |
| 21 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 22 | <code>    pub fn new() -&gt; Self {</code> | Declare new(); its parameters and return type define the interface. |
| 23 | <code>        StackList {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 24 | <code>            items: [MaybeUninit::uninit(); SIZE],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 25 | <code>            count: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 26 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 27 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 28 | <code></code> | Blank line separating related declarations or steps. |
| 29 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 30 | <code>    pub fn add(&amp;mut self, item: T) {</code> | Declare add(); its parameters and return type define the interface. |
| 31 | <code>        self.items[self.count] = MaybeUninit::new(item);</code> | Update the named state using the expression and assignment operator shown. |
| 32 | <code>        self.count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 33 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 34 | <code></code> | Blank line separating related declarations or steps. |
| 35 | <code>    /// The items added so far, ready to read or sort.</code> | Source explanation: The items added so far, ready to read or sort. |
| 36 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 37 | <code>    pub fn as_mut_slice(&amp;mut self) -&gt; &amp;mut [T] {</code> | Declare as_mut_slice(); its parameters and return type define the interface. |
| 38 | <code>        // Slots 0 up to count hold items that add() wrote</code> | Source explanation: Slots 0 up to count hold items that add() wrote |
| 39 | <code>        unsafe { std::slice::from_raw_parts_mut(self.items.as_mut_ptr().cast::&lt;T&gt;(), self.count) }</code> | Enter operations whose memory-safety preconditions the surrounding code must uphold. |
| 40 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 41 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code></code> | Blank line separating related declarations or steps. |
| 43 | <code>/// A list of moves, room for more than the 218 any chess position allows.</code> | Source explanation: A list of moves, room for more than the 218 any chess position allows. |
| 44 | <code>pub type MoveList = StackList&lt;Move, 256&gt;;</code> | Declare the type MoveList used by this module. |
| 45 | <code></code> | Blank line separating related declarations or steps. |
| 46 | <code>impl MoveList {</code> | Begin the implementation of methods or a trait for the named type. |
| 47 | <code>    /// Add one move for each bit of the targets.</code> | Source explanation: Add one move for each bit of the targets. |
| 48 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 49 | <code>    fn add_targets(&amp;mut self, start: usize, mut targets: u64) {</code> | Declare add_targets(); its parameters and return type define the interface. |
| 50 | <code>        while targets != 0 {</code> | Repeat this block while its condition remains true. |
| 51 | <code>            let target = targets.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 52 | <code>            targets &amp;= targets - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 53 | <code>            self.add(new_move(start, target, 0));</code> | Evaluate or continue this expression in add_targets(); a final expression without a semicolon supplies its block&#x27;s value. |
| 54 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 55 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 56 | <code></code> | Blank line separating related declarations or steps. |
| 57 | <code>    /// Add a pawn move, or its four promotions on the last rank.</code> | Source explanation: Add a pawn move, or its four promotions on the last rank. |
| 58 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 59 | <code>    fn add_pawn_move(&amp;mut self, start: usize, target: usize) {</code> | Declare add_pawn_move(); its parameters and return type define the interface. |
| 60 | <code>        if !(8..56).contains(&amp;target) {</code> | Test the condition and execute this branch only when it matches. |
| 61 | <code>            self.add(new_move(start, target, QUEEN));</code> | Evaluate or continue this expression in add_pawn_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 62 | <code>            self.add(new_move(start, target, ROOK));</code> | Evaluate or continue this expression in add_pawn_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 63 | <code>            self.add(new_move(start, target, BISHOP));</code> | Evaluate or continue this expression in add_pawn_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 64 | <code>            self.add(new_move(start, target, KNIGHT));</code> | Evaluate or continue this expression in add_pawn_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 65 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 66 | <code>            self.add(new_move(start, target, 0));</code> | Evaluate or continue this expression in add_pawn_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 67 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 68 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 69 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 70 | <code></code> | Blank line separating related declarations or steps. |
| 71 | <code>/// Add the moves of the side&#x27;s knights, bishops, rooks, queens, and king onto the</code> | Source explanation: Add the moves of the side&#x27;s knights, bishops, rooks, queens, and king onto the |
| 72 | <code>/// squares in the mask.</code> | Source explanation: squares in the mask. |
| 73 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 74 | <code>fn add_piece_moves(position: &amp;Position, own: u64, occupied: u64, mask: u64, list: &amp;mut MoveList) {</code> | Declare add_piece_moves(); its parameters and return type define the interface. |
| 75 | <code>    let kinds = &amp;position.by_kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 76 | <code>    let mut pieces = own &amp; kinds[KNIGHT as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 77 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 78 | <code>        let start = pieces.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 79 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 80 | <code>        list.add_targets(start, KNIGHT_ATTACKS[start] &amp; mask);</code> | Evaluate or continue this expression in add_piece_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 81 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 82 | <code>    let mut pieces = own &amp; kinds[BISHOP as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 83 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 84 | <code>        let start = pieces.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 85 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 86 | <code>        list.add_targets(start, bishop_attacks(start, occupied) &amp; mask);</code> | Evaluate or continue this expression in add_piece_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 87 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 88 | <code>    let mut pieces = own &amp; kinds[ROOK as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 89 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 90 | <code>        let start = pieces.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 91 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 92 | <code>        list.add_targets(start, rook_attacks(start, occupied) &amp; mask);</code> | Evaluate or continue this expression in add_piece_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 93 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 94 | <code>    let mut pieces = own &amp; kinds[QUEEN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 95 | <code>    while pieces != 0 {</code> | Repeat this block while its condition remains true. |
| 96 | <code>        let start = pieces.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 97 | <code>        pieces &amp;= pieces - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 98 | <code>        let reach = rook_attacks(start, occupied) &#124; bishop_attacks(start, occupied);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 99 | <code>        list.add_targets(start, reach &amp; mask);</code> | Evaluate or continue this expression in add_piece_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 100 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 101 | <code>    let king = (own &amp; kinds[KING as usize]).trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 102 | <code>    list.add_targets(king, KING_ATTACKS[king] &amp; mask);</code> | Evaluate or continue this expression in add_piece_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 103 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 104 | <code></code> | Blank line separating related declarations or steps. |
| 105 | <code>/// Add the castling moves. The search checks the square where the king lands.</code> | Source explanation: Add the castling moves. The search checks the square where the king lands. |
| 106 | <code>fn add_castling_moves(position: &amp;Position, list: &amp;mut MoveList) {</code> | Declare add_castling_moves(); its parameters and return type define the interface. |
| 107 | <code>    let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 108 | <code>    let board = &amp;position.board;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 109 | <code>    let king: usize = if white { 60 } else { 4 }; // e1 or e8</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 110 | <code>    let kingside: u8 = if white { 1 } else { 4 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 111 | <code>    let queenside = kingside * 2;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 112 | <code>    let kingside_open =</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 113 | <code>        position.castling &amp; kingside != 0 &amp;&amp; board[king + 1] == EMPTY &amp;&amp; board[king + 2] == EMPTY;</code> | Evaluate or continue this expression in add_castling_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 114 | <code>    let queenside_open = position.castling &amp; queenside != 0</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 115 | <code>        &amp;&amp; board[king - 3] == EMPTY</code> | Evaluate or continue this expression in add_castling_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 116 | <code>        &amp;&amp; board[king - 2] == EMPTY</code> | Evaluate or continue this expression in add_castling_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 117 | <code>        &amp;&amp; board[king - 1] == EMPTY;</code> | Evaluate or continue this expression in add_castling_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 118 | <code>    if !(kingside_open &#124;&#124; queenside_open) &#124;&#124; is_attacked(position, king, !white) {</code> | Test the condition and execute this branch only when it matches. |
| 119 | <code>        return;</code> | Leave the current function with the displayed result. |
| 120 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 121 | <code>    if kingside_open &amp;&amp; !is_attacked(position, king + 1, !white) {</code> | Test the condition and execute this branch only when it matches. |
| 122 | <code>        list.add(new_move(king, king + 2, 0));</code> | Evaluate or continue this expression in add_castling_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 123 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 124 | <code>    if queenside_open &amp;&amp; !is_attacked(position, king - 1, !white) {</code> | Test the condition and execute this branch only when it matches. |
| 125 | <code>        list.add(new_move(king, king - 2, 0));</code> | Evaluate or continue this expression in add_castling_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 126 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 127 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 128 | <code></code> | Blank line separating related declarations or steps. |
| 129 | <code>/// List the pseudo-legal moves of the side to move.</code> | Source explanation: List the pseudo-legal moves of the side to move. |
| 130 | <code>pub fn generate_moves(position: &amp;Position, list: &amp;mut MoveList) {</code> | Declare generate_moves(); its parameters and return type define the interface. |
| 131 | <code>    let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 132 | <code>    let side = usize::from(!white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 133 | <code>    let own = position.by_side[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 134 | <code>    let enemy = position.by_side[1 - side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 135 | <code>    let occupied = own &#124; enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 136 | <code>    let mut pawns = own &amp; position.by_kind[PAWN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 137 | <code>    // A pawn may capture on the en passant square as well as onto an enemy piece</code> | Source explanation: A pawn may capture on the en passant square as well as onto an enemy piece |
| 138 | <code>    let mut pawn_targets = enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 139 | <code>    if position.en_passant &gt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 140 | <code>        pawn_targets &#124;= 1_u64 &lt;&lt; position.en_passant;</code> | Update the named state using the expression and assignment operator shown. |
| 141 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 142 | <code>    let first_row = if white { 6 } else { 1 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 143 | <code>    while pawns != 0 {</code> | Repeat this block while its condition remains true. |
| 144 | <code>        let start = pawns.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 145 | <code>        pawns &amp;= pawns - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 146 | <code>        // White pawns move toward row 0, and one row is 8 squares</code> | Source explanation: White pawns move toward row 0, and one row is 8 squares |
| 147 | <code>        let ahead = if white { start - 8 } else { start + 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 148 | <code>        if position.board[ahead] == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 149 | <code>            list.add_pawn_move(start, ahead);</code> | Evaluate or continue this expression in generate_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 150 | <code>            if start / 8 == first_row {</code> | Test the condition and execute this branch only when it matches. |
| 151 | <code>                let two_ahead = if white { ahead - 8 } else { ahead + 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 152 | <code>                if position.board[two_ahead] == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 153 | <code>                    list.add(new_move(start, two_ahead, 0));</code> | Evaluate or continue this expression in generate_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 154 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 155 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 156 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 157 | <code>        let mut captures = PAWN_ATTACKS[side][start] &amp; pawn_targets;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 158 | <code>        while captures != 0 {</code> | Repeat this block while its condition remains true. |
| 159 | <code>            let target = captures.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 160 | <code>            captures &amp;= captures - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 161 | <code>            list.add_pawn_move(start, target);</code> | Evaluate or continue this expression in generate_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 162 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 163 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 164 | <code>    add_piece_moves(position, own, occupied, !own, list);</code> | Evaluate or continue this expression in generate_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 165 | <code>    if position.castling != 0 {</code> | Test the condition and execute this branch only when it matches. |
| 166 | <code>        add_castling_moves(position, list);</code> | Evaluate or continue this expression in generate_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 167 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 168 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 169 | <code></code> | Blank line separating related declarations or steps. |
| 170 | <code>/// List only the moves quiescence can search: captures and queen promotions. Remus4</code> | Source explanation: List only the moves quiescence can search: captures and queen promotions. Remus4 |
| 171 | <code>/// sorts every move and stops at the first quiet one, which tries these same moves.</code> | Source explanation: sorts every move and stops at the first quiet one, which tries these same moves. |
| 172 | <code>pub fn generate_captures(position: &amp;Position, list: &amp;mut MoveList) {</code> | Declare generate_captures(); its parameters and return type define the interface. |
| 173 | <code>    let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 174 | <code>    let side = usize::from(!white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 175 | <code>    let own = position.by_side[side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 176 | <code>    let enemy = position.by_side[1 - side];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 177 | <code>    let occupied = own &#124; enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 178 | <code>    let mut pawns = own &amp; position.by_kind[PAWN as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 179 | <code>    while pawns != 0 {</code> | Repeat this block while its condition remains true. |
| 180 | <code>        let start = pawns.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 181 | <code>        pawns &amp;= pawns - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 182 | <code>        let ahead = if white { start - 8 } else { start + 8 };</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 183 | <code>        if !(8..56).contains(&amp;ahead) &amp;&amp; position.board[ahead] == EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 184 | <code>            list.add(new_move(start, ahead, QUEEN));</code> | Evaluate or continue this expression in generate_captures(); a final expression without a semicolon supplies its block&#x27;s value. |
| 185 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 186 | <code>        let mut captures = PAWN_ATTACKS[side][start] &amp; enemy;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 187 | <code>        while captures != 0 {</code> | Repeat this block while its condition remains true. |
| 188 | <code>            let target = captures.trailing_zeros() as usize;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 189 | <code>            captures &amp;= captures - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 190 | <code>            list.add_pawn_move(start, target);</code> | Evaluate or continue this expression in generate_captures(); a final expression without a semicolon supplies its block&#x27;s value. |
| 191 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 192 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 193 | <code>    add_piece_moves(position, own, occupied, enemy, list);</code> | Evaluate or continue this expression in generate_captures(); a final expression without a semicolon supplies its block&#x27;s value. |
| 194 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### opening_book.rs

Load embedded opening continuations for validation at the UCI boundary.

[Open source](src/opening_book.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Play a stored opening line while the game follows one. The compiler copies</code> | Source explanation: Play a stored opening line while the game follows one. The compiler copies |
| 2 | <code>//! opening_book.txt into the program, so the book cannot go missing.</code> | Source explanation: opening_book.txt into the program, so the book cannot go missing. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use std::time::{SystemTime, UNIX_EPOCH};</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code></code> | Blank line separating related declarations or steps. |
| 6 | <code>const BOOK_TEXT: &amp;str = include_str!(&quot;../opening_book.txt&quot;);</code> | Define BOOK_TEXT, including its explicit type and initial value. |
| 7 | <code></code> | Blank line separating related declarations or steps. |
| 8 | <code>/// One line of the book: the side it defends (&quot;&quot; for both) and its moves.</code> | Source explanation: One line of the book: the side it defends (&quot;&quot; for both) and its moves. |
| 9 | <code>pub struct BookLine {</code> | Declare the struct BookLine used by this module. |
| 10 | <code>    side: &amp;&#x27;static str,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 11 | <code>    moves: Vec&lt;&amp;&#x27;static str&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 12 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 13 | <code></code> | Blank line separating related declarations or steps. |
| 14 | <code>/// Read the book.</code> | Source explanation: Read the book. |
| 15 | <code>pub fn load_book() -&gt; Vec&lt;BookLine&gt; {</code> | Declare load_book(); its parameters and return type define the interface. |
| 16 | <code>    let mut book = Vec::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 17 | <code>    for text in BOOK_TEXT.lines() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 18 | <code>        let mut side = &quot;&quot;;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 19 | <code>        let mut line = text;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 20 | <code>        if let Some(rest) = text.strip_prefix(&quot;White:&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 21 | <code>            side = &quot;White&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 22 | <code>            line = rest;</code> | Update the named state using the expression and assignment operator shown. |
| 23 | <code>        } else if let Some(rest) = text.strip_prefix(&quot;Black:&quot;) {</code> | Test the condition and execute this branch only when it matches. |
| 24 | <code>            side = &quot;Black&quot;;</code> | Update the named state using the expression and assignment operator shown. |
| 25 | <code>            line = rest;</code> | Update the named state using the expression and assignment operator shown. |
| 26 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 27 | <code>        if !line.trim().is_empty() &amp;&amp; !line.starts_with(&#x27;#&#x27;) {</code> | Test the condition and execute this branch only when it matches. |
| 28 | <code>            book.push(BookLine {</code> | Evaluate or continue this expression in load_book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 29 | <code>                side,</code> | Evaluate or continue this expression in load_book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>                moves: line.split_whitespace().collect(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 31 | <code>            });</code> | Evaluate or continue this expression in load_book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 32 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 33 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 34 | <code>    book</code> | Evaluate or continue this expression in load_book(); a final expression without a semicolon supplies its block&#x27;s value. |
| 35 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 36 | <code></code> | Blank line separating related declarations or steps. |
| 37 | <code>/// Return a number that differs from one call to the next.</code> | Source explanation: Return a number that differs from one call to the next. |
| 38 | <code>fn clock_seed() -&gt; u64 {</code> | Declare clock_seed(); its parameters and return type define the interface. |
| 39 | <code>    let since = SystemTime::now().duration_since(UNIX_EPOCH);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 40 | <code>    since.map_or(2016, &#124;time&#124; time.as_nanos() as u64) &#124; 1</code> | Evaluate or continue this expression in clock_seed(); a final expression without a semicolon supplies its block&#x27;s value. |
| 41 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 42 | <code></code> | Blank line separating related declarations or steps. |
| 43 | <code>/// Return the book&#x27;s next moves after the moves played, in random order.</code> | Source explanation: Return the book&#x27;s next moves after the moves played, in random order. |
| 44 | <code>pub fn book_moves(book: &amp;[BookLine], played: &amp;[&amp;str], white: bool) -&gt; Vec&lt;&amp;&#x27;static str&gt; {</code> | Declare book_moves(); its parameters and return type define the interface. |
| 45 | <code>    let mut moves = Vec::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 46 | <code>    for line in book {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 47 | <code>        let defends = line.side.is_empty() &#124;&#124; (line.side == &quot;White&quot;) == white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 48 | <code>        if defends &amp;&amp; line.moves.len() &gt; played.len() &amp;&amp; line.moves[..played.len()] == *played {</code> | Test the condition and execute this branch only when it matches. |
| 49 | <code>            moves.push(line.moves[played.len()]);</code> | Evaluate or continue this expression in book_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 50 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 51 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code>    // Shuffle with a small xorshift generator seeded from the clock</code> | Source explanation: Shuffle with a small xorshift generator seeded from the clock |
| 53 | <code>    let mut state = clock_seed();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 54 | <code>    for i in (1..moves.len()).rev() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 55 | <code>        state ^= state &lt;&lt; 13;</code> | Update the named state using the expression and assignment operator shown. |
| 56 | <code>        state ^= state &gt;&gt; 7;</code> | Update the named state using the expression and assignment operator shown. |
| 57 | <code>        state ^= state &lt;&lt; 17;</code> | Update the named state using the expression and assignment operator shown. |
| 58 | <code>        moves.swap(i, (state % (i as u64 + 1)) as usize);</code> | Evaluate or continue this expression in book_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 59 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 60 | <code>    moves</code> | Evaluate or continue this expression in book_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 61 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### search.rs

Native move selection, iterative deepening, alpha-beta, quiescence, and ordering.

[Open source](src/search.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Choose a move by alpha-beta search. Every step matches search.py in Remus4, so a</code> | Source explanation: Choose a move by alpha-beta search. Every step matches search.py in Remus4, so a |
| 2 | <code>//! search to a fixed depth visits the same positions and reports the same score.</code> | Source explanation: search to a fixed depth visits the same positions and reports the same score. |
| 3 | <code></code> | Blank line separating related declarations or steps. |
| 4 | <code>use std::io::Write;</code> | Bring the named items into scope; no chess search runs here. |
| 5 | <code>use std::time::Instant;</code> | Bring the named items into scope; no chess search runs here. |
| 6 | <code></code> | Blank line separating related declarations or steps. |
| 7 | <code>use crate::attacks::in_check;</code> | Bring the named items into scope; no chess search runs here. |
| 8 | <code>use crate::board::{BISHOP, KNIGHT, ROOK};</code> | Bring the named items into scope; no chess search runs here. |
| 9 | <code>use crate::board::{</code> | Bring the named items into scope; no chess search runs here. |
| 10 | <code>    EMPTY, MATE_SCORE, Move, NO_MOVE, Position, QUEEN, move_promotion, move_start, move_target,</code> | Continue the enclosing declaration or initialization expression. |
| 11 | <code>    move_text,</code> | Continue the enclosing declaration or initialization expression. |
| 12 | <code>};</code> | Close the preceding block, expression, call, or collection. |
| 13 | <code>use crate::diagnostics::Diagnostics;</code> | Bring the named items into scope; no chess search runs here. |
| 14 | <code>use crate::evaluation::{PIECE_VALUES, evaluate};</code> | Bring the named items into scope; no chess search runs here. |
| 15 | <code>use crate::exchange::exchange_score;</code> | Bring the named items into scope; no chess search runs here. |
| 16 | <code>use crate::make_move::{make_move, make_null_move};</code> | Bring the named items into scope; no chess search runs here. |
| 17 | <code>use crate::move_generation::{MoveList, StackList, generate_captures, generate_moves};</code> | Bring the named items into scope; no chess search runs here. |
| 18 | <code>use crate::table_metrics::TableMetrics;</code> | Bring the named items into scope; no chess search runs here. |
| 19 | <code>use crate::transposition_table::{EXACT, LOWER, Table, UPPER};</code> | Bring the named items into scope; no chess search runs here. |
| 20 | <code></code> | Blank line separating related declarations or steps. |
| 21 | <code>const FUTILITY_MARGIN: i32 = 200; // A quiet move seldom gains more than this</code> | Define FUTILITY_MARGIN, including its explicit type and initial value. |
| 22 | <code>const DELTA_MARGIN: i32 = 200; // A capture seldom gains more than its victim plus this</code> | Define DELTA_MARGIN, including its explicit type and initial value. |
| 23 | <code>const ASPIRATION_WINDOW: i32 = 50; // The first window around the last score</code> | Define ASPIRATION_WINDOW, including its explicit type and initial value. |
| 24 | <code>const CAPTURE: i32 = 10_000; // Every capture and queen promotion scores above this</code> | Define CAPTURE, including its explicit type and initial value. |
| 25 | <code>const MAX_PLY: usize = 128; // Killer moves are kept for this many plies</code> | Define MAX_PLY, including its explicit type and initial value. |
| 26 | <code>const MAX_DEPTH: i32 = 64; // Iterative deepening stops here</code> | Define MAX_DEPTH, including its explicit type and initial value. |
| 27 | <code></code> | Blank line separating related declarations or steps. |
| 28 | <code>/// The sort entries of one position&#x27;s moves.</code> | Source explanation: The sort entries of one position&#x27;s moves. |
| 29 | <code>type EntryList = StackList&lt;u64, 256&gt;;</code> | Declare the type EntryList used by this module. |
| 30 | <code></code> | Blank line separating related declarations or steps. |
| 31 | <code>/// The search stops by returning this error up through every call.</code> | Source explanation: The search stops by returning this error up through every call. |
| 32 | <code>struct SearchTimeout;</code> | Declare the struct SearchTimeout used by this module. |
| 33 | <code></code> | Blank line separating related declarations or steps. |
| 34 | <code>/// Everything the search keeps from one position to the next.</code> | Source explanation: Everything the search keeps from one position to the next. |
| 35 | <code>pub struct Search {</code> | Declare the struct Search used by this module. |
| 36 | <code>    pub diagnostics: Diagnostics,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 37 | <code>    nodes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 38 | <code>    metrics: TableMetrics,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 39 | <code>    deadline: Option&lt;Instant&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 40 | <code>    best_move: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 41 | <code>    history: Vec&lt;u64&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 42 | <code>    killers: [[Move; 2]; MAX_PLY],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 43 | <code>    table: Table,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 44 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 45 | <code></code> | Blank line separating related declarations or steps. |
| 46 | <code>/// Return a list entry that sorts by priority and then by move, as Remus4 sorts the</code> | Source explanation: Return a list entry that sorts by priority and then by move, as Remus4 sorts the |
| 47 | <code>/// pairs (priority, move).</code> | Source explanation: pairs (priority, move). |
| 48 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 49 | <code>fn sort_entry(priority: i32, mv: Move) -&gt; u64 {</code> | Declare sort_entry(); its parameters and return type define the interface. |
| 50 | <code>    (u64::from(priority as u32) &lt;&lt; 16) &#124; u64::from(mv)</code> | Evaluate or continue this expression in sort_entry(); a final expression without a semicolon supplies its block&#x27;s value. |
| 51 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code></code> | Blank line separating related declarations or steps. |
| 53 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 54 | <code>fn entry_move(entry: u64) -&gt; Move {</code> | Declare entry_move(); its parameters and return type define the interface. |
| 55 | <code>    entry as Move</code> | Evaluate or continue this expression in entry_move(); a final expression without a semicolon supplies its block&#x27;s value. |
| 56 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 57 | <code></code> | Blank line separating related declarations or steps. |
| 58 | <code>/// The priority of a move before the table and the killer moves have their say.</code> | Source explanation: The priority of a move before the table and the killer moves have their say. |
| 59 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 60 | <code>fn capture_priority(position: &amp;Position, mv: Move) -&gt; i32 {</code> | Declare capture_priority(); its parameters and return type define the interface. |
| 61 | <code>    let mut priority = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 62 | <code>    let target = position.board[move_target(mv)];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 63 | <code>    if target != EMPTY {</code> | Test the condition and execute this branch only when it matches. |
| 64 | <code>        let victim = PIECE_VALUES[(target &amp; 7) as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 65 | <code>        let attacker = PIECE_VALUES[(position.board[move_start(mv)] &amp; 7) as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 66 | <code>        priority = CAPTURE + 10 * victim - attacker;</code> | Update the named state using the expression and assignment operator shown. |
| 67 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 68 | <code>    if move_promotion(mv) == QUEEN {</code> | Test the condition and execute this branch only when it matches. |
| 69 | <code>        priority += CAPTURE + 9000;</code> | Update the named state using the expression and assignment operator shown. |
| 70 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 71 | <code>    priority</code> | Evaluate or continue this expression in capture_priority(); a final expression without a semicolon supplies its block&#x27;s value. |
| 72 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code></code> | Blank line separating related declarations or steps. |
| 74 | <code>/// Fill the entries with the moves of the position, the likely best first.</code> | Source explanation: Fill the entries with the moves of the position, the likely best first. |
| 75 | <code>#[inline]</code> | Compiler attribute controlling the following declaration. |
| 76 | <code>fn ordered_moves(position: &amp;Position, first: Move, killers: [Move; 2], entries: &amp;mut EntryList) {</code> | Declare ordered_moves(); its parameters and return type define the interface. |
| 77 | <code>    let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 78 | <code>    generate_moves(position, &amp;mut list);</code> | Evaluate or continue this expression in ordered_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 79 | <code>    for &amp;mut mv in list.as_mut_slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 80 | <code>        let mut priority = capture_priority(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 81 | <code>        if mv == killers[0] {</code> | Test the condition and execute this branch only when it matches. |
| 82 | <code>            priority = 5000;</code> | Update the named state using the expression and assignment operator shown. |
| 83 | <code>        } else if mv == killers[1] {</code> | Test the condition and execute this branch only when it matches. |
| 84 | <code>            priority = 4000;</code> | Update the named state using the expression and assignment operator shown. |
| 85 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 86 | <code>        if mv == first {</code> | Test the condition and execute this branch only when it matches. |
| 87 | <code>            priority = 1_000_000;</code> | Update the named state using the expression and assignment operator shown. |
| 88 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 89 | <code>        entries.add(sort_entry(priority, mv));</code> | Evaluate or continue this expression in ordered_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 90 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 91 | <code>    entries.as_mut_slice().sort_unstable_by(&#124;a, b&#124; b.cmp(a));</code> | Evaluate or continue this expression in ordered_moves(); a final expression without a semicolon supplies its block&#x27;s value. |
| 92 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 93 | <code></code> | Blank line separating related declarations or steps. |
| 94 | <code>/// Return true if the side has a piece other than its king and pawns.</code> | Source explanation: Return true if the side has a piece other than its king and pawns. |
| 95 | <code>#[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 96 | <code>fn has_pieces(position: &amp;Position) -&gt; bool {</code> | Declare has_pieces(); its parameters and return type define the interface. |
| 97 | <code>    let kinds = &amp;position.by_kind;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 98 | <code>    let pieces = kinds[KNIGHT as usize]</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 99 | <code>        &#124; kinds[BISHOP as usize]</code> | Evaluate or continue this expression in has_pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 100 | <code>        &#124; kinds[ROOK as usize]</code> | Evaluate or continue this expression in has_pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 101 | <code>        &#124; kinds[QUEEN as usize];</code> | Evaluate or continue this expression in has_pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 102 | <code>    pieces &amp; position.by_side[usize::from(!position.white)] != 0</code> | Evaluate or continue this expression in has_pieces(); a final expression without a semicolon supplies its block&#x27;s value. |
| 103 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 104 | <code></code> | Blank line separating related declarations or steps. |
| 105 | <code>impl Search {</code> | Begin the implementation of methods or a trait for the named type. |
| 106 | <code>    pub fn new() -&gt; Search {</code> | Declare new(); its parameters and return type define the interface. |
| 107 | <code>        Search {</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 108 | <code>            diagnostics: Diagnostics::new(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 109 | <code>            nodes: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 110 | <code>            metrics: TableMetrics::default(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 111 | <code>            deadline: None,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 112 | <code>            best_move: NO_MOVE,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 113 | <code>            history: Vec::with_capacity(1024),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 114 | <code>            killers: [[NO_MOVE; 2]; MAX_PLY],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 115 | <code>            table: Table::new(),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 116 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 117 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 118 | <code></code> | Blank line separating related declarations or steps. |
| 119 | <code>    /// Count one more position, and look at the clock every 1,024.</code> | Source explanation: Count one more position, and look at the clock every 1,024. |
| 120 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 121 | <code>    fn count_node(&amp;mut self) -&gt; Result&lt;(), SearchTimeout&gt; {</code> | Declare count_node(); its parameters and return type define the interface. |
| 122 | <code>        self.nodes += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 123 | <code>        if self.nodes.is_multiple_of(1024)</code> | Test the condition and execute this branch only when it matches. |
| 124 | <code>            &amp;&amp; self.best_move != NO_MOVE</code> | Evaluate or continue this expression in count_node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 125 | <code>            &amp;&amp; let Some(deadline) = self.deadline</code> | Evaluate or continue this expression in count_node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 126 | <code>            &amp;&amp; Instant::now() &gt; deadline</code> | Evaluate or continue this expression in count_node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 127 | <code>        {</code> | Evaluate or continue this expression in count_node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 128 | <code>            return Err(SearchTimeout);</code> | Leave the current function with the displayed result. |
| 129 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 130 | <code>        Ok(())</code> | Evaluate or continue this expression in count_node(); a final expression without a semicolon supplies its block&#x27;s value. |
| 131 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 132 | <code></code> | Blank line separating related declarations or steps. |
| 133 | <code>    /// Search only winning or even captures and queen promotions until the position is</code> | Source explanation: Search only winning or even captures and queen promotions until the position is |
| 134 | <code>    /// quiet. The side to move may also &quot;stand pat&quot;.</code> | Source explanation: quiet. The side to move may also &quot;stand pat&quot;. |
| 135 | <code>    fn quiescence(</code> | Declare quiescence(); its parameters and return type define the interface. |
| 136 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 137 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 138 | <code>        mut alpha: i32,</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 139 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 140 | <code>    ) -&gt; Result&lt;i32, SearchTimeout&gt; {</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 141 | <code>        self.count_node()?;</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 142 | <code>        let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 143 | <code>        let stand_pat = evaluate(position);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 144 | <code>        if stand_pat &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 145 | <code>            return Ok(stand_pat);</code> | Leave the current function with the displayed result. |
| 146 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 147 | <code>        alpha = alpha.max(stand_pat);</code> | Update the named state using the expression and assignment operator shown. |
| 148 | <code></code> | Blank line separating related declarations or steps. |
| 149 | <code>        let mut list = MoveList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 150 | <code>        generate_captures(position, &amp;mut list);</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 151 | <code>        let mut entries = EntryList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 152 | <code>        for &amp;mut mv in list.as_mut_slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 153 | <code>            let priority = capture_priority(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 154 | <code>            if priority &gt;= CAPTURE {</code> | Test the condition and execute this branch only when it matches. |
| 155 | <code>                entries.add(sort_entry(priority, mv));</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 156 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 157 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 158 | <code>        let moves = entries.as_mut_slice();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 159 | <code>        moves.sort_unstable_by(&#124;a, b&#124; b.cmp(a));</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 160 | <code></code> | Blank line separating related declarations or steps. |
| 161 | <code>        for &amp;mut entry in moves {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 162 | <code>            let mv = entry_move(entry);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 163 | <code>            if move_promotion(mv) == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 164 | <code>                // Delta pruning: even winning the victim cannot reach alpha</code> | Source explanation: Delta pruning: even winning the victim cannot reach alpha |
| 165 | <code>                let victim = PIECE_VALUES[(position.board[move_target(mv)] &amp; 7) as usize];</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 166 | <code>                if stand_pat + victim + DELTA_MARGIN &lt; alpha {</code> | Test the condition and execute this branch only when it matches. |
| 167 | <code>                    continue;</code> | Skip the rest of this iteration and advance the loop. |
| 168 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 169 | <code>                let exchange =</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 170 | <code>                    exchange_score(&amp;position.board, &amp;position.by_kind, &amp;position.by_side, mv);</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 171 | <code>                if exchange &lt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 172 | <code>                    continue; // The capture loses material</code> | Skip the rest of this iteration and advance the loop. |
| 173 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 174 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 175 | <code>            let child = make_move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 176 | <code>            if in_check(&amp;child, white) {</code> | Test the condition and execute this branch only when it matches. |
| 177 | <code>                continue; // Illegal: it leaves the king attacked</code> | Skip the rest of this iteration and advance the loop. |
| 178 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 179 | <code>            let score = -self.quiescence(&amp;child, -beta, -alpha)?;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 180 | <code>            if score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 181 | <code>                return Ok(score);</code> | Leave the current function with the displayed result. |
| 182 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 183 | <code>            alpha = alpha.max(score);</code> | Update the named state using the expression and assignment operator shown. |
| 184 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 185 | <code>        Ok(alpha)</code> | Evaluate or continue this expression in quiescence(); a final expression without a semicolon supplies its block&#x27;s value. |
| 186 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 187 | <code></code> | Blank line separating related declarations or steps. |
| 188 | <code>    /// Return true for the fifty-move rule or a repeated position.</code> | Source explanation: Return true for the fifty-move rule or a repeated position. |
| 189 | <code>    #[inline(always)]</code> | Compiler attribute controlling the following declaration. |
| 190 | <code>    fn is_draw(&amp;self, position: &amp;Position) -&gt; bool {</code> | Declare is_draw(); its parameters and return type define the interface. |
| 191 | <code>        if position.halfmove &gt;= 100 {</code> | Test the condition and execute this branch only when it matches. |
| 192 | <code>            return true;</code> | Leave the current function with the displayed result. |
| 193 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 194 | <code>        // The slice history[len - halfmove:] of Remus4, where a negative start counts</code> | Source explanation: The slice history[len - halfmove:] of Remus4, where a negative start counts |
| 195 | <code>        // back from the end of the list</code> | Source explanation: back from the end of the list |
| 196 | <code>        let length = self.history.len() as i64;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 197 | <code>        let mut start = length - i64::from(position.halfmove);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 198 | <code>        if start &lt; 0 {</code> | Test the condition and execute this branch only when it matches. |
| 199 | <code>            start = (length + start).max(0);</code> | Update the named state using the expression and assignment operator shown. |
| 200 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 201 | <code>        self.history[start as usize..].contains(&amp;position.key)</code> | Evaluate or continue this expression in is_draw(); a final expression without a semicolon supplies its block&#x27;s value. |
| 202 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 203 | <code></code> | Blank line separating related declarations or steps. |
| 204 | <code>    /// Return the score for the side to move, searching depth moves ahead.</code> | Source explanation: Return the score for the side to move, searching depth moves ahead. |
| 205 | <code>    fn alpha_beta(</code> | Declare alpha_beta(); its parameters and return type define the interface. |
| 206 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 207 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 208 | <code>        mut depth: i32,</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 209 | <code>        ply: usize,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 210 | <code>        mut alpha: i32,</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 211 | <code>        beta: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 212 | <code>        can_null: bool,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 213 | <code>    ) -&gt; Result&lt;i32, SearchTimeout&gt; {</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 214 | <code>        self.count_node()?;</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 215 | <code>        let white = position.white;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 216 | <code>        let key = position.key;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 217 | <code>        if ply &gt; 0 &amp;&amp; self.is_draw(position) {</code> | Test the condition and execute this branch only when it matches. |
| 218 | <code>            return Ok(0);</code> | Leave the current function with the displayed result. |
| 219 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 220 | <code>        if ply &gt;= MAX_PLY {</code> | Test the condition and execute this branch only when it matches. |
| 221 | <code>            return Ok(evaluate(position));</code> | Leave the current function with the displayed result. |
| 222 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 223 | <code>        let check = in_check(position, white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 224 | <code>        if check {</code> | Test the condition and execute this branch only when it matches. |
| 225 | <code>            depth += 1; // Never stop the search in check</code> | Update the named state using the expression and assignment operator shown. |
| 226 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 227 | <code>        if depth &lt;= 0 {</code> | Test the condition and execute this branch only when it matches. |
| 228 | <code>            return self.quiescence(position, alpha, beta);</code> | Leave the current function with the displayed result. |
| 229 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 230 | <code>        let mut first = NO_MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 231 | <code>        self.metrics.probes += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 232 | <code>        if let Some(stored) = self.table.probe(key, ply as i32) {</code> | Test the condition and execute this branch only when it matches. |
| 233 | <code>            self.metrics.hits += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 234 | <code>            first = stored.mv;</code> | Update the named state using the expression and assignment operator shown. |
| 235 | <code>            if ply &gt; 0</code> | Test the condition and execute this branch only when it matches. |
| 236 | <code>                &amp;&amp; stored.depth &gt;= depth</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 237 | <code>                &amp;&amp; (stored.bound == EXACT</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 238 | <code>                    &#124;&#124; (stored.bound == LOWER &amp;&amp; stored.score &gt;= beta)</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 239 | <code>                    &#124;&#124; (stored.bound == UPPER &amp;&amp; stored.score &lt;= alpha))</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 240 | <code>            {</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 241 | <code>                self.metrics.cutoffs += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 242 | <code>                return Ok(stored.score);</code> | Leave the current function with the displayed result. |
| 243 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 244 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 245 | <code>        let static_score = evaluate(position);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 246 | <code>        // Null move: let the opponent move twice. If the side to move still stands above</code> | Source explanation: Null move: let the opponent move twice. If the side to move still stands above |
| 247 | <code>        // beta with a shallower search, its real moves will too.</code> | Source explanation: beta with a shallower search, its real moves will too. |
| 248 | <code>        if can_null</code> | Test the condition and execute this branch only when it matches. |
| 249 | <code>            &amp;&amp; !check</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 250 | <code>            &amp;&amp; ply &gt; 0</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 251 | <code>            &amp;&amp; depth &gt;= 3</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 252 | <code>            &amp;&amp; static_score &gt;= beta</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 253 | <code>            &amp;&amp; has_pieces(position)</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 254 | <code>        {</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 255 | <code>            let null = make_null_move(position);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 256 | <code>            self.history.push(key);</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 257 | <code>            let result = self.alpha_beta(&amp;null, depth - 3, ply + 1, -beta, -beta + 1, false);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 258 | <code>            self.history.pop();</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 259 | <code>            if -result? &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 260 | <code>                return Ok(beta);</code> | Leave the current function with the displayed result. |
| 261 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 262 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 263 | <code>        self.history.push(key);</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 264 | <code>        let original_alpha = alpha;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 265 | <code>        let mut best_score = -MATE_SCORE - 1;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 266 | <code>        let mut best = NO_MOVE;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 267 | <code>        let mut legal_count = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 268 | <code>        let mut entries = EntryList::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 269 | <code>        ordered_moves(position, first, self.killers[ply], &amp;mut entries);</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 270 | <code>        for &amp;mut entry in entries.as_mut_slice() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 271 | <code>            let mv = entry_move(entry);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 272 | <code>            let child = make_move(position, mv);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 273 | <code>            if in_check(&amp;child, white) {</code> | Test the condition and execute this branch only when it matches. |
| 274 | <code>                continue; // Illegal: it leaves the king attacked</code> | Skip the rest of this iteration and advance the loop. |
| 275 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 276 | <code>            legal_count += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 277 | <code>            let quiet = position.board[move_target(mv)] == EMPTY &amp;&amp; move_promotion(mv) == 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 278 | <code>            let calm = quiet &amp;&amp; !check &amp;&amp; !in_check(&amp;child, !white);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 279 | <code>            // Futility: one move from the leaves, a quiet move cannot lift a position far</code> | Source explanation: Futility: one move from the leaves, a quiet move cannot lift a position far |
| 280 | <code>            // below alpha</code> | Source explanation: below alpha |
| 281 | <code>            if depth == 1 &amp;&amp; calm &amp;&amp; legal_count &gt; 1 &amp;&amp; static_score + FUTILITY_MARGIN &lt;= alpha {</code> | Test the condition and execute this branch only when it matches. |
| 282 | <code>                continue;</code> | Skip the rest of this iteration and advance the loop. |
| 283 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 284 | <code>            let mut score;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 285 | <code>            if legal_count == 1 {</code> | Test the condition and execute this branch only when it matches. |
| 286 | <code>                score = -self.alpha_beta(&amp;child, depth - 1, ply + 1, -beta, -alpha, true)?;</code> | Update the named state using the expression and assignment operator shown. |
| 287 | <code>            } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 288 | <code>                // Late move reduction: a late quiet move gets one ply less</code> | Source explanation: Late move reduction: a late quiet move gets one ply less |
| 289 | <code>                let reduce = depth &gt;= 3</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 290 | <code>                    &amp;&amp; legal_count &gt; 3</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 291 | <code>                    &amp;&amp; calm</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 292 | <code>                    &amp;&amp; mv != self.killers[ply][0]</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 293 | <code>                    &amp;&amp; mv != self.killers[ply][1];</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 294 | <code>                // Principal variation search: prove the move is no better than alpha with</code> | Source explanation: Principal variation search: prove the move is no better than alpha with |
| 295 | <code>                // a null window, and search again if it is</code> | Source explanation: a null window, and search again if it is |
| 296 | <code>                let reduced_depth = depth - 1 - i32::from(reduce);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 297 | <code>                score =</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 298 | <code>                    -self.alpha_beta(&amp;child, reduced_depth, ply + 1, -alpha - 1, -alpha, true)?;</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 299 | <code>                if score &gt; alpha &amp;&amp; (reduce &#124;&#124; score &lt; beta) {</code> | Test the condition and execute this branch only when it matches. |
| 300 | <code>                    score = -self.alpha_beta(&amp;child, depth - 1, ply + 1, -beta, -alpha, true)?;</code> | Update the named state using the expression and assignment operator shown. |
| 301 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 302 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 303 | <code>            if score &gt; best_score {</code> | Test the condition and execute this branch only when it matches. |
| 304 | <code>                best_score = score;</code> | Update the named state using the expression and assignment operator shown. |
| 305 | <code>                best = mv;</code> | Update the named state using the expression and assignment operator shown. |
| 306 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 307 | <code>            if score &gt; alpha {</code> | Test the condition and execute this branch only when it matches. |
| 308 | <code>                alpha = score;</code> | Update the named state using the expression and assignment operator shown. |
| 309 | <code>                if ply == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 310 | <code>                    self.best_move = mv;</code> | Update the named state using the expression and assignment operator shown. |
| 311 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 312 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 313 | <code>            if score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 314 | <code>                if quiet &amp;&amp; self.killers[ply][0] != mv {</code> | Test the condition and execute this branch only when it matches. |
| 315 | <code>                    self.killers[ply] = [mv, self.killers[ply][0]];</code> | Update the named state using the expression and assignment operator shown. |
| 316 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 317 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 318 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 319 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 320 | <code>        self.history.pop();</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 321 | <code>        if legal_count == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 322 | <code>            // Checkmate, worse the sooner it comes, or stalemate</code> | Source explanation: Checkmate, worse the sooner it comes, or stalemate |
| 323 | <code>            return Ok(if check { -(MATE_SCORE - ply as i32) } else { 0 });</code> | Leave the current function with the displayed result. |
| 324 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 325 | <code>        let mut bound = EXACT;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 326 | <code>        if best_score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 327 | <code>            bound = LOWER;</code> | Update the named state using the expression and assignment operator shown. |
| 328 | <code>        } else if best_score &lt;= original_alpha {</code> | Test the condition and execute this branch only when it matches. |
| 329 | <code>            bound = UPPER;</code> | Update the named state using the expression and assignment operator shown. |
| 330 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 331 | <code>        self.metrics.stores += 1;</code> | Update the named state using the expression and assignment operator shown. |
| 332 | <code>        self.metrics.replacements += u64::from(</code> | Update the named state using the expression and assignment operator shown. |
| 333 | <code>            self.table</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 334 | <code>                .store(key, depth, best_score, bound, best, ply as i32),</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 335 | <code>        );</code> | Close the preceding block, expression, call, or collection. |
| 336 | <code>        Ok(best_score)</code> | Evaluate or continue this expression in alpha_beta(); a final expression without a semicolon supplies its block&#x27;s value. |
| 337 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 338 | <code></code> | Blank line separating related declarations or steps. |
| 339 | <code>    /// Return the line of best play that the table holds, as UCI text.</code> | Source explanation: Return the line of best play that the table holds, as UCI text. |
| 340 | <code>    fn principal_variation(&amp;self, position: &amp;Position, depth: i32) -&gt; String {</code> | Declare principal_variation(); its parameters and return type define the interface. |
| 341 | <code>        let mut line = String::new();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 342 | <code>        let mut current = *position;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 343 | <code>        for _ in 0..depth {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 344 | <code>            let Some(stored) = self.table.probe(current.key, 0) else {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 345 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 346 | <code>            };</code> | Close the preceding block, expression, call, or collection. |
| 347 | <code>            if stored.mv == NO_MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 348 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 349 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 350 | <code>            line.push(&#x27; &#x27;);</code> | Evaluate or continue this expression in principal_variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 351 | <code>            line.push_str(&amp;move_text(stored.mv));</code> | Evaluate or continue this expression in principal_variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 352 | <code>            current = make_move(&amp;current, stored.mv);</code> | Update the named state using the expression and assignment operator shown. |
| 353 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 354 | <code>        line</code> | Evaluate or continue this expression in principal_variation(); a final expression without a semicolon supplies its block&#x27;s value. |
| 355 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 356 | <code></code> | Blank line separating related declarations or steps. |
| 357 | <code>    /// Search one move deeper at a time and return the best move, or NO_MOVE when there</code> | Source explanation: Search one move deeper at a time and return the best move, or NO_MOVE when there |
| 358 | <code>    /// is no legal move. The search plans to use seconds, takes longer when the best</code> | Source explanation: is no legal move. The search plans to use seconds, takes longer when the best |
| 359 | <code>    /// move keeps changing, and never runs past longest. A depth limit replaces the</code> | Source explanation: move keeps changing, and never runs past longest. A depth limit replaces the |
| 360 | <code>    /// clock, for the workshop designer&#x27;s checks.</code> | Source explanation: clock, for the workshop designer&#x27;s checks. |
| 361 | <code>    pub fn think(</code> | Declare think(); its parameters and return type define the interface. |
| 362 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 363 | <code>        position: &amp;Position,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 364 | <code>        game_history: &amp;[u64],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 365 | <code>        mut seconds: f64,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 366 | <code>        longest: f64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 367 | <code>        depth_limit: Option&lt;i32&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 368 | <code>    ) -&gt; Move {</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 369 | <code>        let start = Instant::now();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 370 | <code>        self.deadline = if depth_limit.is_some() {</code> | Update the named state using the expression and assignment operator shown. |
| 371 | <code>            None</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 372 | <code>        } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 373 | <code>            Some(start + std::time::Duration::from_secs_f64(longest.max(0.0)))</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 374 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 375 | <code>        self.nodes = 0;</code> | Update the named state using the expression and assignment operator shown. |
| 376 | <code>        self.metrics = TableMetrics::default();</code> | Update the named state using the expression and assignment operator shown. |
| 377 | <code>        self.best_move = NO_MOVE;</code> | Update the named state using the expression and assignment operator shown. |
| 378 | <code>        self.table.new_search();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 379 | <code>        let mut score = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 380 | <code>        let mut stdout = std::io::stdout();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 381 | <code>        &#x27;deepening: for depth in 1..=MAX_DEPTH {</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 382 | <code>            let previous = self.best_move;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 383 | <code>            let (mut alpha, mut beta) = (-MATE_SCORE - 1, MATE_SCORE + 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 384 | <code>            if depth &gt;= 4 {</code> | Test the condition and execute this branch only when it matches. |
| 385 | <code>                alpha = score - ASPIRATION_WINDOW;</code> | Update the named state using the expression and assignment operator shown. |
| 386 | <code>                beta = score + ASPIRATION_WINDOW;</code> | Update the named state using the expression and assignment operator shown. |
| 387 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 388 | <code>            loop {</code> | Repeat until an explicit break, return, or other control transfer ends the loop. |
| 389 | <code>                self.history.clear();</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 390 | <code>                self.history.extend_from_slice(game_history);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 391 | <code>                match self.alpha_beta(position, depth, 0, alpha, beta, true) {</code> | Select an arm by matching the expression against the listed patterns. |
| 392 | <code>                    Ok(found) =&gt; score = found,</code> | Handle this matching pattern with the expression or block on its right. |
| 393 | <code>                    Err(SearchTimeout) =&gt; break &#x27;deepening,</code> | Handle this matching pattern with the expression or block on its right. |
| 394 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 395 | <code>                // Outside the window: search again with that side open</code> | Source explanation: Outside the window: search again with that side open |
| 396 | <code>                if score &lt;= alpha {</code> | Test the condition and execute this branch only when it matches. |
| 397 | <code>                    alpha = -MATE_SCORE - 1;</code> | Update the named state using the expression and assignment operator shown. |
| 398 | <code>                } else if score &gt;= beta {</code> | Test the condition and execute this branch only when it matches. |
| 399 | <code>                    beta = MATE_SCORE + 1;</code> | Update the named state using the expression and assignment operator shown. |
| 400 | <code>                } else {</code> | Begin the alternative branch when the preceding condition did not match. |
| 401 | <code>                    break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 402 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 403 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 404 | <code>            if self.best_move == NO_MOVE {</code> | Test the condition and execute this branch only when it matches. |
| 405 | <code>                break; // No legal move</code> | Leave the selected loop, with a value where the expression supplies one. |
| 406 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 407 | <code>            let line = self.principal_variation(position, depth);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 408 | <code>            let info = format!(</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 409 | <code>                &quot;info depth {depth} score cp {score} nodes {} pv{line}&quot;,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 410 | <code>                self.nodes</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 411 | <code>            );</code> | Close the preceding block, expression, call, or collection. |
| 412 | <code>            let _ = writeln!(stdout, &quot;{info}&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 413 | <code>            let _ = stdout.flush();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 414 | <code>            self.diagnostics.completed_depth = depth;</code> | Update the named state using the expression and assignment operator shown. |
| 415 | <code>            self.diagnostics.report(&amp;info);</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 416 | <code>            // A best move that changes needs more time to settle</code> | Source explanation: A best move that changes needs more time to settle |
| 417 | <code>            if previous != NO_MOVE &amp;&amp; self.best_move != previous {</code> | Test the condition and execute this branch only when it matches. |
| 418 | <code>                seconds = (seconds * 1.5).min(longest);</code> | Update the named state using the expression and assignment operator shown. |
| 419 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 420 | <code>            if score.abs() &gt; MATE_SCORE - 100 {</code> | Test the condition and execute this branch only when it matches. |
| 421 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 422 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 423 | <code>            match depth_limit {</code> | Select an arm by matching the expression against the listed patterns. |
| 424 | <code>                Some(limit) if depth &gt;= limit =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 425 | <code>                None if start.elapsed().as_secs_f64() &gt; seconds / 2.0 =&gt; break,</code> | Handle this matching pattern with the expression or block on its right. |
| 426 | <code>                _ =&gt; {}</code> | Handle this matching pattern with the expression or block on its right. |
| 427 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 428 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 429 | <code>        let seconds = start.elapsed().as_secs_f64();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 430 | <code>        self.diagnostics.report(&amp;format!(</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 431 | <code>            &quot;info string serial workers=1 nodes={} native_ms={:.1} nps={:.0} hash_mib={} {}&quot;,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 432 | <code>            self.nodes,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 433 | <code>            seconds * 1000.0,</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 434 | <code>            self.nodes as f64 / seconds.max(0.000_001),</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 435 | <code>            self.table.size_mib(),</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 436 | <code>            self.metrics</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 437 | <code>        ));</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 438 | <code>        self.best_move</code> | Evaluate or continue this expression in think(); a final expression without a semicolon supplies its block&#x27;s value. |
| 439 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 440 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### table_metrics.rs

Collect transposition-table usage and replacement measurements.

[Open source](src/table_metrics.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Worker-local table counters; no shared writes in the recursive search.</code> | Source explanation: Worker-local table counters; no shared writes in the recursive search. |
| 2 | <code></code> | Blank line separating related declarations or steps. |
| 3 | <code>#[derive(Clone, Copy, Default)]</code> | Compiler attribute controlling the following declaration. |
| 4 | <code>pub struct TableMetrics {</code> | Declare the struct TableMetrics used by this module. |
| 5 | <code>    pub probes: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 6 | <code>    pub hits: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 7 | <code>    pub cutoffs: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 8 | <code>    pub stores: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 9 | <code>    pub replacements: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 10 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 11 | <code></code> | Blank line separating related declarations or steps. |
| 12 | <code>#[allow(dead_code)]</code> | Compiler attribute controlling the following declaration. |
| 13 | <code>impl TableMetrics {</code> | Begin the implementation of methods or a trait for the named type. |
| 14 | <code>    pub fn add(&amp;mut self, other: Self) {</code> | Declare add(); its parameters and return type define the interface. |
| 15 | <code>        self.probes += other.probes;</code> | Update the named state using the expression and assignment operator shown. |
| 16 | <code>        self.hits += other.hits;</code> | Update the named state using the expression and assignment operator shown. |
| 17 | <code>        self.cutoffs += other.cutoffs;</code> | Update the named state using the expression and assignment operator shown. |
| 18 | <code>        self.stores += other.stores;</code> | Update the named state using the expression and assignment operator shown. |
| 19 | <code>        self.replacements += other.replacements;</code> | Update the named state using the expression and assignment operator shown. |
| 20 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 21 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 22 | <code></code> | Blank line separating related declarations or steps. |
| 23 | <code>impl std::fmt::Display for TableMetrics {</code> | Begin the implementation of methods or a trait for the named type. |
| 24 | <code>    fn fmt(&amp;self, out: &amp;mut std::fmt::Formatter&lt;&#x27;_&gt;) -&gt; std::fmt::Result {</code> | Declare fmt(); its parameters and return type define the interface. |
| 25 | <code>        write!(</code> | Format and send text to the output destination shown. |
| 26 | <code>            out,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 27 | <code>            &quot;tt_probes={} tt_hits={} tt_cutoffs={} tt_stores={} tt_replacements={} tt_hit_pct={:.2} tt_cutoff_pct={:.2} tt_replacement_pct={:.2}&quot;,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 28 | <code>            self.probes,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 29 | <code>            self.hits,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 30 | <code>            self.cutoffs,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 31 | <code>            self.stores,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 32 | <code>            self.replacements,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 33 | <code>            100.0 * self.hits as f64 / self.probes.max(1) as f64,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 34 | <code>            100.0 * self.cutoffs as f64 / self.probes.max(1) as f64,</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 35 | <code>            100.0 * self.replacements as f64 / self.stores.max(1) as f64</code> | Evaluate or continue this expression in fmt(); a final expression without a semicolon supplies its block&#x27;s value. |
| 36 | <code>        )</code> | Close the preceding block, expression, call, or collection. |
| 37 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 38 | <code>}</code> | Close the preceding block, expression, call, or collection. |

### transposition_table.rs

Store and retrieve bounded search results using position keys.

[Open source](src/transposition_table.rs)

| Line | Exact source | Explanation |
| ---: | --- | --- |
| 1 | <code>//! Remember what the search learned about each position.</code> | Source explanation: Remember what the search learned about each position. |
| 2 | <code>//!</code> | Source explanation:  |
| 3 | <code>//! The table has a fixed 512 MiB capacity. Each bucket is one</code> | Source explanation: The table has a fixed 512 MiB capacity. Each bucket is one |
| 4 | <code>//! 64-byte cache line, so a look at all four entries costs one trip to memory. A position</code> | Source explanation: 64-byte cache line, so a look at all four entries costs one trip to memory. A position |
| 5 | <code>//! can live in any of its bucket&#x27;s four entries. A full bucket gives up its least useful</code> | Source explanation: can live in any of its bucket&#x27;s four entries. A full bucket gives up its least useful |
| 6 | <code>//! entry: the one left by the oldest search, then the shallowest one. Until a bucket</code> | Source explanation: entry: the one left by the oldest search, then the shallowest one. Until a bucket |
| 7 | <code>//! fills, the table behaves exactly like the Python dict of Remus4.</code> | Source explanation: fills, the table behaves exactly like the Python dict of Remus4. |
| 8 | <code></code> | Blank line separating related declarations or steps. |
| 9 | <code>use std::alloc::{Layout, alloc_zeroed};</code> | Bring the named items into scope; no chess search runs here. |
| 10 | <code></code> | Blank line separating related declarations or steps. |
| 11 | <code>use crate::board::{MATE_SCORE, Move};</code> | Bring the named items into scope; no chess search runs here. |
| 12 | <code></code> | Blank line separating related declarations or steps. |
| 13 | <code>pub const EXACT: u8 = 0;</code> | Define EXACT, including its explicit type and initial value. |
| 14 | <code>pub const LOWER: u8 = 1;</code> | Define LOWER, including its explicit type and initial value. |
| 15 | <code>pub const UPPER: u8 = 2;</code> | Define UPPER, including its explicit type and initial value. |
| 16 | <code></code> | Blank line separating related declarations or steps. |
| 17 | <code>const MIB: usize = 1024 * 1024;</code> | Define MIB, including its explicit type and initial value. |
| 18 | <code>const TABLE_BYTES: usize = 512 * MIB;</code> | Define TABLE_BYTES, including its explicit type and initial value. |
| 19 | <code>const ENTRIES_PER_BUCKET: usize = 4;</code> | Define ENTRIES_PER_BUCKET, including its explicit type and initial value. |
| 20 | <code>const AGE_LIMIT: u8 = 64; // Ages wrap around at this count of searches</code> | Define AGE_LIMIT, including its explicit type and initial value. |
| 21 | <code>const AGE_WEIGHT: i32 = 8; // One search of age counts as much as this many plies</code> | Define AGE_WEIGHT, including its explicit type and initial value. |
| 22 | <code></code> | Blank line separating related declarations or steps. |
| 23 | <code>#[derive(Clone, Copy)]</code> | Compiler attribute controlling the following declaration. |
| 24 | <code>struct Entry {</code> | Declare the struct Entry used by this module. |
| 25 | <code>    key: u64, // 0 marks an empty entry</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 26 | <code>    score: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 27 | <code>    mv: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 28 | <code>    depth: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 29 | <code>    bound_and_age: u8, // The bound in the low 2 bits and the age above them</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 30 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 31 | <code></code> | Blank line separating related declarations or steps. |
| 32 | <code>const _: () = assert!(size_of::&lt;Entry&gt;() == 16 &amp;&amp; size_of::&lt;Bucket&gt;() == 64);</code> | Define _, including its explicit type and initial value. |
| 33 | <code></code> | Blank line separating related declarations or steps. |
| 34 | <code>#[repr(align(64))]</code> | Compiler attribute controlling the following declaration. |
| 35 | <code>#[derive(Clone, Copy)]</code> | Compiler attribute controlling the following declaration. |
| 36 | <code>struct Bucket {</code> | Declare the struct Bucket used by this module. |
| 37 | <code>    entries: [Entry; ENTRIES_PER_BUCKET],</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 38 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 39 | <code></code> | Blank line separating related declarations or steps. |
| 40 | <code>/// What probe() found: the depth, score, bound, and best move of a position.</code> | Source explanation: What probe() found: the depth, score, bound, and best move of a position. |
| 41 | <code>pub struct Stored {</code> | Declare the struct Stored used by this module. |
| 42 | <code>    pub depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 43 | <code>    pub score: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 44 | <code>    pub bound: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 45 | <code>    pub mv: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 46 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 47 | <code></code> | Blank line separating related declarations or steps. |
| 48 | <code>pub struct Table {</code> | Declare the struct Table used by this module. |
| 49 | <code>    buckets: Box&lt;[Bucket]&gt;,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 50 | <code>    age: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 51 | <code>}</code> | Close the preceding block, expression, call, or collection. |
| 52 | <code></code> | Blank line separating related declarations or steps. |
| 53 | <code>impl Table {</code> | Begin the implementation of methods or a trait for the named type. |
| 54 | <code>    pub fn new() -&gt; Table {</code> | Declare new(); its parameters and return type define the interface. |
| 55 | <code>        let count = TABLE_BYTES / size_of::&lt;Bucket&gt;();</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 56 | <code>        let layout = Layout::array::&lt;Bucket&gt;(count).expect(&quot;the table size fits in memory&quot;);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 57 | <code>        let buckets = unsafe {</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 58 | <code>            let memory = alloc_zeroed(layout);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 59 | <code>            if memory.is_null() {</code> | Test the condition and execute this branch only when it matches. |
| 60 | <code>                return Self::disabled();</code> | Leave the current function with the displayed result. |
| 61 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 62 | <code>            let slice = std::ptr::slice_from_raw_parts_mut(memory.cast::&lt;Bucket&gt;(), count);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 63 | <code>            Box::from_raw(slice)</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 64 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 65 | <code>        Table { buckets, age: 0 }</code> | Evaluate or continue this expression in new(); a final expression without a semicolon supplies its block&#x27;s value. |
| 66 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 67 | <code></code> | Blank line separating related declarations or steps. |
| 68 | <code>    pub fn disabled() -&gt; Table {</code> | Declare disabled(); its parameters and return type define the interface. |
| 69 | <code>        Table {</code> | Evaluate or continue this expression in disabled(); a final expression without a semicolon supplies its block&#x27;s value. |
| 70 | <code>            buckets: Box::new([]),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 71 | <code>            age: 0,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 72 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 73 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 74 | <code></code> | Blank line separating related declarations or steps. |
| 75 | <code>    pub fn size_mib(&amp;self) -&gt; usize {</code> | Declare size_mib(); its parameters and return type define the interface. |
| 76 | <code>        self.buckets.len() * size_of::&lt;Bucket&gt;() / MIB</code> | Evaluate or continue this expression in size_mib(); a final expression without a semicolon supplies its block&#x27;s value. |
| 77 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 78 | <code></code> | Blank line separating related declarations or steps. |
| 79 | <code>    /// Mark the start of a new search, so older entries give way first.</code> | Source explanation: Mark the start of a new search, so older entries give way first. |
| 80 | <code>    pub fn new_search(&amp;mut self) {</code> | Declare new_search(); its parameters and return type define the interface. |
| 81 | <code>        self.age = (self.age + 1) % AGE_LIMIT;</code> | Update the named state using the expression and assignment operator shown. |
| 82 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 83 | <code></code> | Blank line separating related declarations or steps. |
| 84 | <code>    /// Save one search result. A mate score is stored as mate from here.</code> | Source explanation: Save one search result. A mate score is stored as mate from here. |
| 85 | <code>    #[inline]</code> | Compiler attribute controlling the following declaration. |
| 86 | <code>    pub fn store(</code> | Declare store(); its parameters and return type define the interface. |
| 87 | <code>        &amp;mut self,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 88 | <code>        key: u64,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 89 | <code>        depth: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 90 | <code>        mut score: i32,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 91 | <code>        bound: u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 92 | <code>        mv: Move,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 93 | <code>        ply: i32,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 94 | <code>    ) -&gt; bool {</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 95 | <code>        if self.buckets.is_empty() {</code> | Test the condition and execute this branch only when it matches. |
| 96 | <code>            return false;</code> | Leave the current function with the displayed result. |
| 97 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 98 | <code>        if score &gt; MATE_SCORE - 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 99 | <code>            score += ply;</code> | Update the named state using the expression and assignment operator shown. |
| 100 | <code>        } else if score &lt; -MATE_SCORE + 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 101 | <code>            score -= ply;</code> | Update the named state using the expression and assignment operator shown. |
| 102 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 103 | <code>        let age = self.age;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 104 | <code>        let index = key as usize &amp; (self.buckets.len() - 1);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 105 | <code>        let entries = &amp;mut self.buckets[index].entries;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 106 | <code>        let mut chosen = 0;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 107 | <code>        let mut lowest_value = i32::MAX;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 108 | <code>        for (i, entry) in entries.iter().enumerate() {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 109 | <code>            if entry.key == key &#124;&#124; entry.key == 0 {</code> | Test the condition and execute this branch only when it matches. |
| 110 | <code>                chosen = i;</code> | Update the named state using the expression and assignment operator shown. |
| 111 | <code>                break;</code> | Leave the selected loop, with a value where the expression supplies one. |
| 112 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 113 | <code>            let searches_ago = i32::from(age.wrapping_sub(entry.bound_and_age &gt;&gt; 2) % AGE_LIMIT);</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 114 | <code>            let value = i32::from(entry.depth) - AGE_WEIGHT * searches_ago;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 115 | <code>            if value &lt; lowest_value {</code> | Test the condition and execute this branch only when it matches. |
| 116 | <code>                lowest_value = value;</code> | Update the named state using the expression and assignment operator shown. |
| 117 | <code>                chosen = i;</code> | Update the named state using the expression and assignment operator shown. |
| 118 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 119 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 120 | <code>        let replacement = entries[chosen].key != 0 &amp;&amp; entries[chosen].key != key;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 121 | <code>        entries[chosen] = Entry {</code> | Update the named state using the expression and assignment operator shown. |
| 122 | <code>            key,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 123 | <code>            score,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 124 | <code>            mv,</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 125 | <code>            depth: depth as u8,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 126 | <code>            bound_and_age: bound &#124; (age &lt;&lt; 2),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 127 | <code>        };</code> | Close the preceding block, expression, call, or collection. |
| 128 | <code>        replacement</code> | Evaluate or continue this expression in store(); a final expression without a semicolon supplies its block&#x27;s value. |
| 129 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 130 | <code></code> | Blank line separating related declarations or steps. |
| 131 | <code>    /// Return what the table holds for the position, or None.</code> | Source explanation: Return what the table holds for the position, or None. |
| 132 | <code>    #[inline]</code> | Compiler attribute controlling the following declaration. |
| 133 | <code>    pub fn probe(&amp;self, key: u64, ply: i32) -&gt; Option&lt;Stored&gt; {</code> | Declare probe(); its parameters and return type define the interface. |
| 134 | <code>        if self.buckets.is_empty() {</code> | Test the condition and execute this branch only when it matches. |
| 135 | <code>            return None;</code> | Leave the current function with the displayed result. |
| 136 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 137 | <code>        let entries = &amp;self.buckets[key as usize &amp; (self.buckets.len() - 1)].entries;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 138 | <code>        for entry in entries {</code> | Visit the items in the displayed iterator, binding each item for this block. |
| 139 | <code>            if entry.key == key {</code> | Test the condition and execute this branch only when it matches. |
| 140 | <code>                let mut score = entry.score;</code> | Bind the local name or destructured fields to the expression; mut permits later reassignment. |
| 141 | <code>                if score &gt; MATE_SCORE - 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 142 | <code>                    score -= ply;</code> | Update the named state using the expression and assignment operator shown. |
| 143 | <code>                } else if score &lt; -MATE_SCORE + 1000 {</code> | Test the condition and execute this branch only when it matches. |
| 144 | <code>                    score += ply;</code> | Update the named state using the expression and assignment operator shown. |
| 145 | <code>                }</code> | Close the preceding block, expression, call, or collection. |
| 146 | <code>                return Some(Stored {</code> | Leave the current function with the displayed result. |
| 147 | <code>                    depth: i32::from(entry.depth),</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 148 | <code>                    score,</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 149 | <code>                    bound: entry.bound_and_age &amp; 3,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 150 | <code>                    mv: entry.mv,</code> | Specify this field, argument, or element&#x27;s type or value in the enclosing declaration. |
| 151 | <code>                });</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 152 | <code>            }</code> | Close the preceding block, expression, call, or collection. |
| 153 | <code>        }</code> | Close the preceding block, expression, call, or collection. |
| 154 | <code>        None</code> | Evaluate or continue this expression in probe(); a final expression without a semicolon supplies its block&#x27;s value. |
| 155 | <code>    }</code> | Close the preceding block, expression, call, or collection. |
| 156 | <code>}</code> | Close the preceding block, expression, call, or collection. |

## Source coverage and identity

The reference covers all 12 runtime Rust modules and all 2,166 physical source lines. These are source-reference line numbers, not student typing totals. The overview's Lines of code count excludes blank and comment lines. SHA-256 identifies exact source bytes, including line endings.

| Source | SHA-256 |
| --- | --- |
| [src/attacks.rs](src/attacks.rs) | `20b307128d4eaa9572122f924da86d682762a2fcbf24908540b187dbc39c9700` |
| [src/board.rs](src/board.rs) | `4b50b9e774a893e4a297a8ee21322a6eb27bf166a67eb6a06067da75d33d1f6a` |
| [src/diagnostics.rs](src/diagnostics.rs) | `5a590e2148b22e649428199e7506615e90700a02175b1e00ad9d145d1dad378a` |
| [src/evaluation.rs](src/evaluation.rs) | `429f953bedc038e97bc660bfc1a45b9f859d88a9f48a401ae5003a24630f381e` |
| [src/exchange.rs](src/exchange.rs) | `0d17d128305c95edd191a4265a2d429cd76bd9477e96ec3de30a26922fa450a4` |
| [src/main.rs](src/main.rs) | `751ae8bbbd8803007da6776e42b647c60f91894c25290a9e71b1ca16018f2ac1` |
| [src/make_move.rs](src/make_move.rs) | `4f701e4f884f7b92116f8e8aa2a036f5a35717d09d730fedf3231f01d2c56524` |
| [src/move_generation.rs](src/move_generation.rs) | `0672d0d647dc6c3cab6f21633d28fd13e2746de1b36c42a7f11e82a435f1cd64` |
| [src/opening_book.rs](src/opening_book.rs) | `37dbe9750c830b6a4132d199555ffe06fd63d834303ceb874866dd821e69f865` |
| [src/search.rs](src/search.rs) | `ec1bd3f10d5cbd757e74723f7dd1630867c57130a92627a247bfd28a2d53d0ac` |
| [src/table_metrics.rs](src/table_metrics.rs) | `29bd32f01788a3e8a749a45f97f802cb02ebd81806bfca55b09870cbed2e4eb9` |
| [src/transposition_table.rs](src/transposition_table.rs) | `3bc583ff92927ec8319defc4b6edbb0a9a19a5fc861b11201ce17a3b00b357ac` |
