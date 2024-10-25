(function() {var type_impls = {
"io_lifetimes":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsFd-for-BorrowedFd%3C'_%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0\">1.63.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#277\">source</a></span><a href=\"#impl-AsFd-for-BorrowedFd%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"io_lifetimes/trait.AsFd.html\" title=\"trait io_lifetimes::AsFd\">AsFd</a> for <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_fd\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#279\">source</a><a href=\"#method.as_fd\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"io_lifetimes/trait.AsFd.html#tymethod.as_fd\" class=\"fn\">as_fd</a>(&amp;self) -&gt; <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h4></section></summary><div class='docblock'>Borrows the file descriptor. <a href=\"io_lifetimes/trait.AsFd.html#tymethod.as_fd\">Read more</a></div></details></div></details>","AsFd","io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-AsRawFd-for-BorrowedFd%3C'_%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0\">1.63.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#132\">source</a></span><a href=\"#impl-AsRawFd-for-BorrowedFd%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/std/os/fd/raw/trait.AsRawFd.html\" title=\"trait std::os::fd::raw::AsRawFd\">AsRawFd</a> for <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.as_raw_fd\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#134\">source</a><a href=\"#method.as_raw_fd\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/std/os/fd/raw/trait.AsRawFd.html#tymethod.as_raw_fd\" class=\"fn\">as_raw_fd</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.i32.html\">i32</a></h4></section></summary><div class='docblock'>Extracts the raw file descriptor. <a href=\"https://doc.rust-lang.org/1.81.0/std/os/fd/raw/trait.AsRawFd.html#tymethod.as_raw_fd\">Read more</a></div></details></div></details>","AsRawFd","io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BorrowedFd%3C'_%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#72\">source</a><a href=\"#impl-BorrowedFd%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.borrow_raw\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0, const since 1.63.0\">1.63.0 (const: 1.63.0)</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#82\">source</a></span><h4 class=\"code-header\">pub const unsafe fn <a href=\"io_lifetimes/struct.BorrowedFd.html#tymethod.borrow_raw\" class=\"fn\">borrow_raw</a>(fd: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.i32.html\">i32</a>) -&gt; <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h4></section></summary><div class=\"docblock\"><p>Return a <code>BorrowedFd</code> holding the given raw file descriptor.</p>\n<h5 id=\"safety\"><a class=\"doc-anchor\" href=\"#safety\">§</a>Safety</h5>\n<p>The resource pointed to by <code>fd</code> must remain open for the duration of\nthe returned <code>BorrowedFd</code>, and it must not have the value <code>-1</code>.</p>\n</div></details></div></details>",0,"io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BorrowedFd%3C'_%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#98\">source</a><a href=\"#impl-BorrowedFd%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_clone_to_owned\" class=\"method\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0\">1.63.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#103\">source</a></span><h4 class=\"code-header\">pub fn <a href=\"io_lifetimes/struct.BorrowedFd.html#tymethod.try_clone_to_owned\" class=\"fn\">try_clone_to_owned</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"struct\" href=\"io_lifetimes/struct.OwnedFd.html\" title=\"struct io_lifetimes::OwnedFd\">OwnedFd</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new <code>OwnedFd</code> instance that shares the same underlying file\ndescription as the existing <code>BorrowedFd</code> instance.</p>\n</div></details></div></details>",0,"io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BorrowedFd%3C'fd%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0\">1.63.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#35\">source</a></span><a href=\"#impl-Clone-for-BorrowedFd%3C'fd%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'fd&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'fd&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#35\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'fd&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BorrowedFd%3C'_%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0\">1.63.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#204\">source</a></span><a href=\"#impl-Debug-for-BorrowedFd%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#205\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.81.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-IsTerminal-for-BorrowedFd%3C'_%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.70.0\">1.70.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#232\">source</a></span><a href=\"#impl-IsTerminal-for-BorrowedFd%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/std/io/stdio/trait.IsTerminal.html\" title=\"trait std::io::stdio::IsTerminal\">IsTerminal</a> for <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_terminal\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#232\">source</a><a href=\"#method.is_terminal\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/std/io/stdio/trait.IsTerminal.html#tymethod.is_terminal\" class=\"fn\">is_terminal</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns <code>true</code> if the descriptor/handle refers to a terminal/tty. <a href=\"https://doc.rust-lang.org/1.81.0/std/io/stdio/trait.IsTerminal.html#tymethod.is_terminal\">Read more</a></div></details></div></details>","IsTerminal","io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"],["<section id=\"impl-Copy-for-BorrowedFd%3C'fd%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.63.0\">1.63.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/std/os/fd/owned.rs.html#35\">source</a></span><a href=\"#impl-Copy-for-BorrowedFd%3C'fd%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'fd&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"io_lifetimes/struct.BorrowedFd.html\" title=\"struct io_lifetimes::BorrowedFd\">BorrowedFd</a>&lt;'fd&gt;</h3></section>","Copy","io_lifetimes::portability::BorrowedFilelike","io_lifetimes::portability::BorrowedSocketlike"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()