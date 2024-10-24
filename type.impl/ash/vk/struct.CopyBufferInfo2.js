(function() {
    var type_impls = Object.fromEntries([["ash",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-CopyBufferInfo2%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36310\">source</a><a href=\"#impl-Clone-for-CopyBufferInfo2%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36310\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'a&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.82.0/src/core/clone.rs.html#174\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: &amp;Self)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.82.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","ash::vk::aliases::CopyBufferInfo2KHR"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-CopyBufferInfo2%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36341-36358\">source</a><a href=\"#impl-CopyBufferInfo2%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"method.src_buffer\" class=\"method\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36343-36346\">source</a><h4 class=\"code-header\">pub fn <a href=\"ash/vk/struct.CopyBufferInfo2.html#tymethod.src_buffer\" class=\"fn\">src_buffer</a>(self, src_buffer: <a class=\"struct\" href=\"ash/vk/struct.Buffer.html\" title=\"struct ash::vk::Buffer\">Buffer</a>) -&gt; Self</h4></section><section id=\"method.dst_buffer\" class=\"method\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36348-36351\">source</a><h4 class=\"code-header\">pub fn <a href=\"ash/vk/struct.CopyBufferInfo2.html#tymethod.dst_buffer\" class=\"fn\">dst_buffer</a>(self, dst_buffer: <a class=\"struct\" href=\"ash/vk/struct.Buffer.html\" title=\"struct ash::vk::Buffer\">Buffer</a>) -&gt; Self</h4></section><section id=\"method.regions\" class=\"method\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36353-36357\">source</a><h4 class=\"code-header\">pub fn <a href=\"ash/vk/struct.CopyBufferInfo2.html#tymethod.regions\" class=\"fn\">regions</a>(self, regions: &amp;'a [<a class=\"struct\" href=\"ash/vk/struct.BufferCopy2.html\" title=\"struct ash::vk::BufferCopy2\">BufferCopy2</a>&lt;'a&gt;]) -&gt; Self</h4></section></div></details>",0,"ash::vk::aliases::CopyBufferInfo2KHR"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-CopyBufferInfo2%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36309\">source</a><a href=\"#impl-Debug-for-CopyBufferInfo2%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36309\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.82.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.82.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","ash::vk::aliases::CopyBufferInfo2KHR"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-CopyBufferInfo2%3C'_%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36324-36337\">source</a><a href=\"#impl-Default-for-CopyBufferInfo2%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'_&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36326-36336\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.82.0/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.82.0/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","ash::vk::aliases::CopyBufferInfo2KHR"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TaggedStructure-for-CopyBufferInfo2%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36338-36340\">source</a><a href=\"#impl-TaggedStructure-for-CopyBufferInfo2%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"ash/vk/trait.TaggedStructure.html\" title=\"trait ash::vk::TaggedStructure\">TaggedStructure</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'a&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedconstant.STRUCTURE_TYPE\" class=\"associatedconstant trait-impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36339\">source</a><a href=\"#associatedconstant.STRUCTURE_TYPE\" class=\"anchor\">§</a><h4 class=\"code-header\">const <a href=\"ash/vk/trait.TaggedStructure.html#associatedconstant.STRUCTURE_TYPE\" class=\"constant\">STRUCTURE_TYPE</a>: <a class=\"struct\" href=\"ash/vk/struct.StructureType.html\" title=\"struct ash::vk::StructureType\">StructureType</a> = StructureType::COPY_BUFFER_INFO_2</h4></section></div></details>","TaggedStructure","ash::vk::aliases::CopyBufferInfo2KHR"],["<section id=\"impl-Copy-for-CopyBufferInfo2%3C'a%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36310\">source</a><a href=\"#impl-Copy-for-CopyBufferInfo2%3C'a%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'a&gt;</h3></section>","Copy","ash::vk::aliases::CopyBufferInfo2KHR"],["<section id=\"impl-Send-for-CopyBufferInfo2%3C'_%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36322\">source</a><a href=\"#impl-Send-for-CopyBufferInfo2%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'_&gt;</h3></section>","Send","ash::vk::aliases::CopyBufferInfo2KHR"],["<section id=\"impl-Sync-for-CopyBufferInfo2%3C'_%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/ash/vk/definitions.rs.html#36323\">source</a><a href=\"#impl-Sync-for-CopyBufferInfo2%3C'_%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.82.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"ash/vk/struct.CopyBufferInfo2.html\" title=\"struct ash::vk::CopyBufferInfo2\">CopyBufferInfo2</a>&lt;'_&gt;</h3></section>","Sync","ash::vk::aliases::CopyBufferInfo2KHR"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[10129]}