(function() {var type_impls = {
"bitstream_io":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BigEndian\" class=\"impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#451\">source</a><a href=\"#impl-Clone-for-BigEndian\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"bitstream_io/struct.BigEndian.html\" title=\"struct bitstream_io::BigEndian\">BigEndian</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#451\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"bitstream_io/struct.BigEndian.html\" title=\"struct bitstream_io::BigEndian\">BigEndian</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","bitstream_io::BE"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BigEndian\" class=\"impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#451\">source</a><a href=\"#impl-Debug-for-BigEndian\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"bitstream_io/struct.BigEndian.html\" title=\"struct bitstream_io::BigEndian\">BigEndian</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#451\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","bitstream_io::BE"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Endianness-for-BigEndian\" class=\"impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#457-652\">source</a><a href=\"#impl-Endianness-for-BigEndian\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"bitstream_io/trait.Endianness.html\" title=\"trait bitstream_io::Endianness\">Endianness</a> for <a class=\"struct\" href=\"bitstream_io/struct.BigEndian.html\" title=\"struct bitstream_io::BigEndian\">BigEndian</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.push\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#459-468\">source</a><a href=\"#method.push\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.push\" class=\"fn\">push</a>&lt;N&gt;(queue: &amp;mut <a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;, bits: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>, value: N)<div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Pushes the given bits and value onto an accumulator\nwith the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.push_fixed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#471-480\">source</a><a href=\"#method.push_fixed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.push_fixed\" class=\"fn\">push_fixed</a>&lt;const B: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>, N&gt;(queue: &amp;mut <a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;, value: N)<div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Pushes the given constant number of bits and value onto an accumulator\nwith the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#483-499\">source</a><a href=\"#method.pop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.pop\" class=\"fn\">pop</a>&lt;N&gt;(queue: &amp;mut <a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;, bits: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>) -&gt; N<div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Pops a value with the given number of bits from an accumulator\nwith the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.pop_fixed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#502-518\">source</a><a href=\"#method.pop_fixed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.pop_fixed\" class=\"fn\">pop_fixed</a>&lt;const B: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>, N&gt;(queue: &amp;mut <a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;) -&gt; N<div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Pops a value with the given number of constant bits\nfrom an accumulator with the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#521-532\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.drop\" class=\"fn\">drop</a>&lt;N&gt;(queue: &amp;mut <a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;, bits: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>)<div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Drops the given number of bits from an accumulator\nwith the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.next_zeros\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#535-540\">source</a><a href=\"#method.next_zeros\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.next_zeros\" class=\"fn\">next_zeros</a>&lt;N&gt;(queue: &amp;<a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a><div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Returns the next number of 0 bits from an accumulator\nwith the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.next_ones\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#543-553\">source</a><a href=\"#method.next_ones\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.next_ones\" class=\"fn\">next_ones</a>&lt;N&gt;(queue: &amp;<a class=\"struct\" href=\"bitstream_io/struct.BitQueue.html\" title=\"struct bitstream_io::BitQueue\">BitQueue</a>&lt;Self, N&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a><div class=\"where\">where\n    N: <a class=\"trait\" href=\"bitstream_io/trait.Numeric.html\" title=\"trait bitstream_io::Numeric\">Numeric</a>,</div></h4></section></summary><div class='docblock'>Returns the next number of 1 bits from an accumulator\nwith the given bits and value.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.read_signed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#555-567\">source</a><a href=\"#method.read_signed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.read_signed\" class=\"fn\">read_signed</a>&lt;R, S&gt;(r: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut R</a>, bits: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;S&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"bitstream_io/read/trait.BitRead.html\" title=\"trait bitstream_io::read::BitRead\">BitRead</a>,\n    S: <a class=\"trait\" href=\"bitstream_io/trait.SignedNumeric.html\" title=\"trait bitstream_io::SignedNumeric\">SignedNumeric</a>,</div></h4></section></summary><div class='docblock'>Reads signed value from reader in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.read_signed_fixed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#569-581\">source</a><a href=\"#method.read_signed_fixed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.read_signed_fixed\" class=\"fn\">read_signed_fixed</a>&lt;R, const B: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>, S&gt;(r: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut R</a>) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;S&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"bitstream_io/read/trait.BitRead.html\" title=\"trait bitstream_io::read::BitRead\">BitRead</a>,\n    S: <a class=\"trait\" href=\"bitstream_io/trait.SignedNumeric.html\" title=\"trait bitstream_io::SignedNumeric\">SignedNumeric</a>,</div></h4></section></summary><div class='docblock'>Reads signed value from reader in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_signed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#583-596\">source</a><a href=\"#method.write_signed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.write_signed\" class=\"fn\">write_signed</a>&lt;W, S&gt;(w: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut W</a>, bits: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>, value: S) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"bitstream_io/write/trait.BitWrite.html\" title=\"trait bitstream_io::write::BitWrite\">BitWrite</a>,\n    S: <a class=\"trait\" href=\"bitstream_io/trait.SignedNumeric.html\" title=\"trait bitstream_io::SignedNumeric\">SignedNumeric</a>,</div></h4></section></summary><div class='docblock'>Writes signed value to writer in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_signed_fixed\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#598-611\">source</a><a href=\"#method.write_signed_fixed\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.write_signed_fixed\" class=\"fn\">write_signed_fixed</a>&lt;W, const B: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.u32.html\">u32</a>, S&gt;(w: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut W</a>, value: S) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"bitstream_io/write/trait.BitWrite.html\" title=\"trait bitstream_io::write::BitWrite\">BitWrite</a>,\n    S: <a class=\"trait\" href=\"bitstream_io/trait.SignedNumeric.html\" title=\"trait bitstream_io::SignedNumeric\">SignedNumeric</a>,</div></h4></section></summary><div class='docblock'>Writes signed value to writer in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.read_primitive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#614-622\">source</a><a href=\"#method.read_primitive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.read_primitive\" class=\"fn\">read_primitive</a>&lt;R, V&gt;(r: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut R</a>) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;V&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"bitstream_io/read/trait.BitRead.html\" title=\"trait bitstream_io::read::BitRead\">BitRead</a>,\n    V: <a class=\"trait\" href=\"bitstream_io/trait.Primitive.html\" title=\"trait bitstream_io::Primitive\">Primitive</a>,</div></h4></section></summary><div class='docblock'>Reads convertable numeric value from reader in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_primitive\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#625-631\">source</a><a href=\"#method.write_primitive\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.write_primitive\" class=\"fn\">write_primitive</a>&lt;W, V&gt;(w: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;mut W</a>, value: V) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"bitstream_io/write/trait.BitWrite.html\" title=\"trait bitstream_io::write::BitWrite\">BitWrite</a>,\n    V: <a class=\"trait\" href=\"bitstream_io/trait.Primitive.html\" title=\"trait bitstream_io::Primitive\">Primitive</a>,</div></h4></section></summary><div class='docblock'>Writes convertable numeric value to writer in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.read_numeric\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#634-642\">source</a><a href=\"#method.read_numeric\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.read_numeric\" class=\"fn\">read_numeric</a>&lt;R, V&gt;(r: R) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;V&gt;<div class=\"where\">where\n    R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a>,\n    V: <a class=\"trait\" href=\"bitstream_io/trait.Primitive.html\" title=\"trait bitstream_io::Primitive\">Primitive</a>,</div></h4></section></summary><div class='docblock'>Reads entire numeric value from reader in this endianness</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.write_numeric\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#645-651\">source</a><a href=\"#method.write_numeric\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"bitstream_io/trait.Endianness.html#tymethod.write_numeric\" class=\"fn\">write_numeric</a>&lt;W, V&gt;(w: W, value: V) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>&gt;<div class=\"where\">where\n    W: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/std/io/trait.Write.html\" title=\"trait std::io::Write\">Write</a>,\n    V: <a class=\"trait\" href=\"bitstream_io/trait.Primitive.html\" title=\"trait bitstream_io::Primitive\">Primitive</a>,</div></h4></section></summary><div class='docblock'>Writes entire numeric value to writer in this endianness</div></details></div></details>","Endianness","bitstream_io::BE"],["<section id=\"impl-Copy-for-BigEndian\" class=\"impl\"><a class=\"src rightside\" href=\"src/bitstream_io/lib.rs.html#451\">source</a><a href=\"#impl-Copy-for-BigEndian\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"bitstream_io/struct.BigEndian.html\" title=\"struct bitstream_io::BigEndian\">BigEndian</a></h3></section>","Copy","bitstream_io::BE"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()