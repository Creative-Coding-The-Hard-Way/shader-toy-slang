(function() {
    var type_impls = Object.fromEntries([["image",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ImageReader%3CBufReader%3CFile%3E%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#284-307\">source</a><a href=\"#impl-ImageReader%3CBufReader%3CFile%3E%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"image/struct.ImageReader.html\" title=\"struct image::ImageReader\">ImageReader</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/io/buffered/bufreader/struct.BufReader.html\" title=\"struct std::io::buffered::bufreader::BufReader\">BufReader</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/fs/struct.File.html\" title=\"struct std::fs::File\">File</a>&gt;&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.open\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#293-298\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.open\" class=\"fn\">open</a>&lt;P&gt;(path: P) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;Self&gt;<div class=\"where\">where\n    P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/convert/trait.AsRef.html\" title=\"trait core::convert::AsRef\">AsRef</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/std/path/struct.Path.html\" title=\"struct std::path::Path\">Path</a>&gt;,</div></h4></section></summary><div class=\"docblock\"><p>Open a file to read, format will be guessed from path.</p>\n<p>This will not attempt any io operation on the opened file.</p>\n<p>If you want to inspect the content for a better guess on the format, which does not depend\non file extensions, follow this call with a call to <a href=\"#method.with_guessed_format\"><code>with_guessed_format</code></a>.</p>\n</div></details></div></details>",0,"image::io::Reader"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ImageReader%3CR%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#70-282\">source</a><a href=\"#impl-ImageReader%3CR%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a, R: 'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/std/io/trait.Seek.html\" title=\"trait std::io::Seek\">Seek</a>&gt; <a class=\"struct\" href=\"image/struct.ImageReader.html\" title=\"struct image::ImageReader\">ImageReader</a>&lt;R&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#81-87\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.new\" class=\"fn\">new</a>(buffered_reader: R) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Create a new image reader without a preset format.</p>\n<p>Assumes the reader is already buffered. For optimal performance,\nconsider wrapping the reader with a <code>BufReader::new()</code>.</p>\n<p>It is possible to guess the format based on the content of the read object with\n<a href=\"#method.with_guessed_format\"><code>with_guessed_format</code></a>, or to set the format directly with <a href=\"method.set_format\"><code>set_format</code></a>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_format\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#93-99\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.with_format\" class=\"fn\">with_format</a>(buffered_reader: R, format: <a class=\"enum\" href=\"image/enum.ImageFormat.html\" title=\"enum image::ImageFormat\">ImageFormat</a>) -&gt; Self</h4></section></summary><div class=\"docblock\"><p>Construct a reader with specified format.</p>\n<p>Assumes the reader is already buffered. For optimal performance,\nconsider wrapping the reader with a <code>BufReader::new()</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.format\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#102-104\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.format\" class=\"fn\">format</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.82.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"enum\" href=\"image/enum.ImageFormat.html\" title=\"enum image::ImageFormat\">ImageFormat</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Get the currently determined format.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.set_format\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#107-109\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.set_format\" class=\"fn\">set_format</a>(&amp;mut self, format: <a class=\"enum\" href=\"image/enum.ImageFormat.html\" title=\"enum image::ImageFormat\">ImageFormat</a>)</h4></section></summary><div class=\"docblock\"><p>Supply the format as which to interpret the read image.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clear_format\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#115-117\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.clear_format\" class=\"fn\">clear_format</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Remove the current information on the image format.</p>\n<p>Note that many operations require format information to be present and will return e.g. an\n<code>ImageError::Unsupported</code> when the image format has not been set.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.no_limits\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#120-122\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.no_limits\" class=\"fn\">no_limits</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Disable all decoding limits.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.limits\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#125-127\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.limits\" class=\"fn\">limits</a>(&amp;mut self, limits: <a class=\"struct\" href=\"image/struct.Limits.html\" title=\"struct image::Limits\">Limits</a>)</h4></section></summary><div class=\"docblock\"><p>Set a custom set of decoding limits.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_inner\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#130-132\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.into_inner\" class=\"fn\">into_inner</a>(self) -&gt; R</h4></section></summary><div class=\"docblock\"><p>Unwrap the reader.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_decoder\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#189-194\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.into_decoder\" class=\"fn\">into_decoder</a>(self) -&gt; <a class=\"type\" href=\"image/error/type.ImageResult.html\" title=\"type image::error::ImageResult\">ImageResult</a>&lt;impl <a class=\"trait\" href=\"image/trait.ImageDecoder.html\" title=\"trait image::ImageDecoder\">ImageDecoder</a> + 'a&gt;</h4></section></summary><div class=\"docblock\"><p>Convert the reader into a decoder.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.with_guessed_format\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#224-229\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.with_guessed_format\" class=\"fn\">with_guessed_format</a>(self) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/std/io/error/type.Result.html\" title=\"type std::io::error::Result\">Result</a>&lt;Self&gt;</h4></section></summary><div class=\"docblock\"><p>Make a format guess based on the content, replacing it on success.</p>\n<p>Returns <code>Ok</code> with the guess if no io error occurs. Additionally, replaces the current\nformat if the guess was successful. If the guess was unable to determine a format then\nthe current format of the reader is unchanged.</p>\n<p>Returns an error if the underlying reader fails. The format is unchanged. The error is a\n<code>std::io::Error</code> and not <code>ImageError</code> since the only error case is an error when the\nunderlying reader seeks.</p>\n<p>When an error occurs, the reader may not have been properly reset and it is potentially\nhazardous to continue with more io.</p>\n<h6 id=\"usage\"><a class=\"doc-anchor\" href=\"#usage\">§</a>Usage</h6>\n<p>This supplements the path based type deduction from <a href=\"image/struct.ImageReader.html#method.open\" title=\"associated function image::ImageReader::open\"><code>ImageReader::open()</code></a> with content based deduction.\nThis is more common in Linux and UNIX operating systems and also helpful if the path can\nnot be directly controlled.</p>\n\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">let </span>image = ImageReader::open(<span class=\"string\">\"image.unknown\"</span>)<span class=\"question-mark\">?\n    </span>.with_guessed_format()<span class=\"question-mark\">?\n    </span>.decode()<span class=\"question-mark\">?</span>;</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_dimensions\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#251-253\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.into_dimensions\" class=\"fn\">into_dimensions</a>(self) -&gt; <a class=\"type\" href=\"image/error/type.ImageResult.html\" title=\"type image::error::ImageResult\">ImageResult</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u32.html\">u32</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.82.0/std/primitive.u32.html\">u32</a>)&gt;</h4></section></summary><div class=\"docblock\"><p>Read the image dimensions.</p>\n<p>Uses the current format to construct the correct reader for the format.</p>\n<p>If no format was determined, returns an <code>ImageError::Unsupported</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.decode\" class=\"method\"><a class=\"src rightside\" href=\"src/image/image_reader/image_reader_type.rs.html#260-272\">source</a><h4 class=\"code-header\">pub fn <a href=\"image/struct.ImageReader.html#tymethod.decode\" class=\"fn\">decode</a>(self) -&gt; <a class=\"type\" href=\"image/error/type.ImageResult.html\" title=\"type image::error::ImageResult\">ImageResult</a>&lt;<a class=\"enum\" href=\"image/enum.DynamicImage.html\" title=\"enum image::DynamicImage\">DynamicImage</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Read the image (replaces <code>load</code>).</p>\n<p>Uses the current format to construct the correct reader for the format.</p>\n<p>If no format was determined, returns an <code>ImageError::Unsupported</code>.</p>\n</div></details></div></details>",0,"image::io::Reader"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[12412]}