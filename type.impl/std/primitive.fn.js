(function() {
    var type_impls = Object.fromEntries([["ash",[]],["flexi_logger",[]],["glfw",[]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[10,20,12]}