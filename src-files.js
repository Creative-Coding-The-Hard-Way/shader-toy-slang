var srcIndex = new Map(JSON.parse('[\
["aho_corasick",["",[["nfa",[],["contiguous.rs","mod.rs","noncontiguous.rs"]],["packed",[["teddy",[],["builder.rs","generic.rs","mod.rs"]]],["api.rs","ext.rs","mod.rs","pattern.rs","rabinkarp.rs","vector.rs"]],["util",[],["alphabet.rs","buffer.rs","byte_frequencies.rs","debug.rs","error.rs","int.rs","mod.rs","prefilter.rs","primitives.rs","remapper.rs","search.rs","special.rs"]]],["ahocorasick.rs","automaton.rs","dfa.rs","lib.rs","macros.rs"]]],\
["anstream",["",[["adapter",[],["mod.rs","strip.rs","wincon.rs"]]],["auto.rs","buffer.rs","fmt.rs","lib.rs","macros.rs","stream.rs","strip.rs"]]],\
["anstyle",["",[],["color.rs","effect.rs","lib.rs","macros.rs","reset.rs","style.rs"]]],\
["anstyle_parse",["",[["state",[],["definitions.rs","mod.rs","table.rs"]]],["lib.rs","params.rs"]]],\
["anstyle_query",["",[],["lib.rs","windows.rs"]]],\
["anyhow",["",[],["backtrace.rs","chain.rs","context.rs","ensure.rs","error.rs","fmt.rs","kind.rs","lib.rs","macros.rs","ptr.rs","wrapper.rs"]]],\
["argfile",["",[],["argument.rs","fromfile.rs","lib.rs"]]],\
["ash",["",[["extensions",[["amd",[],["buffer_marker.rs","mod.rs","shader_info.rs"]],["amdx",[],["mod.rs","shader_enqueue.rs"]],["android",[],["external_memory_android_hardware_buffer.rs","mod.rs"]],["ext",[],["acquire_drm_display.rs","buffer_device_address.rs","calibrated_timestamps.rs","debug_marker.rs","debug_report.rs","debug_utils.rs","descriptor_buffer.rs","extended_dynamic_state.rs","extended_dynamic_state2.rs","extended_dynamic_state3.rs","full_screen_exclusive.rs","hdr_metadata.rs","headless_surface.rs","host_image_copy.rs","image_compression_control.rs","image_drm_format_modifier.rs","mesh_shader.rs","metal_surface.rs","mod.rs","pipeline_properties.rs","private_data.rs","sample_locations.rs","shader_object.rs","swapchain_maintenance1.rs","tooling_info.rs","vertex_input_dynamic_state.rs"]],["google",[],["display_timing.rs","mod.rs"]],["khr",[],["acceleration_structure.rs","android_surface.rs","buffer_device_address.rs","calibrated_timestamps.rs","cooperative_matrix.rs","copy_commands2.rs","create_renderpass2.rs","deferred_host_operations.rs","device_group.rs","device_group_creation.rs","display.rs","display_swapchain.rs","draw_indirect_count.rs","dynamic_rendering.rs","dynamic_rendering_local_read.rs","external_fence_fd.rs","external_fence_win32.rs","external_memory_fd.rs","external_memory_win32.rs","external_semaphore_fd.rs","external_semaphore_win32.rs","get_memory_requirements2.rs","get_physical_device_properties2.rs","get_surface_capabilities2.rs","line_rasterization.rs","maintenance1.rs","maintenance3.rs","maintenance4.rs","maintenance5.rs","maintenance6.rs","mod.rs","performance_query.rs","pipeline_executable_properties.rs","present_wait.rs","push_descriptor.rs","ray_tracing_maintenance1.rs","ray_tracing_pipeline.rs","sampler_ycbcr_conversion.rs","surface.rs","swapchain.rs","synchronization2.rs","timeline_semaphore.rs","wayland_surface.rs","win32_surface.rs","xcb_surface.rs","xlib_surface.rs"]],["mvk",[],["ios_surface.rs","macos_surface.rs","mod.rs"]],["nn",[],["mod.rs","vi_surface.rs"]],["nv",[],["copy_memory_indirect.rs","coverage_reduction_mode.rs","cuda_kernel_launch.rs","device_diagnostic_checkpoints.rs","device_generated_commands_compute.rs","low_latency2.rs","memory_decompression.rs","mesh_shader.rs","mod.rs","ray_tracing.rs"]]],["mod.rs"]],["vk",[],["aliases.rs","bitflags.rs","const_debugs.rs","constants.rs","definitions.rs","enums.rs","extensions.rs","feature_extensions.rs","features.rs","macros.rs","native.rs","platform_types.rs","prelude.rs"]]],["device.rs","entry.rs","extensions_generated.rs","instance.rs","lib.rs","prelude.rs","tables.rs","util.rs","vk.rs"]]],\
["bitflags",["",[],["external.rs","internal.rs","iter.rs","lib.rs","parser.rs","public.rs","traits.rs"]]],\
["cfg_if",["",[],["lib.rs"]]],\
["chrono",["",[["datetime",[],["mod.rs"]],["format",[],["formatting.rs","locales.rs","mod.rs","parse.rs","parsed.rs","scan.rs","strftime.rs"]],["naive",[["date",[],["mod.rs"]],["datetime",[],["mod.rs"]],["time",[],["mod.rs"]]],["internals.rs","isoweek.rs","mod.rs"]],["offset",[["local",[["tz_info",[],["mod.rs","parser.rs","rule.rs","timezone.rs"]]],["mod.rs","unix.rs"]]],["fixed.rs","mod.rs","utc.rs"]]],["date.rs","lib.rs","month.rs","round.rs","time_delta.rs","traits.rs","weekday.rs"]]],\
["clap",["",[],["lib.rs"]]],\
["clap_builder",["",[["builder",[],["action.rs","app_settings.rs","arg.rs","arg_group.rs","arg_predicate.rs","arg_settings.rs","command.rs","debug_asserts.rs","ext.rs","mod.rs","os_str.rs","possible_value.rs","range.rs","resettable.rs","str.rs","styled_str.rs","styling.rs","value_hint.rs","value_parser.rs"]],["error",[],["context.rs","format.rs","kind.rs","mod.rs"]],["output",[["textwrap",[],["core.rs","mod.rs","word_separators.rs","wrap_algorithms.rs"]]],["fmt.rs","help.rs","help_template.rs","mod.rs","usage.rs"]],["parser",[["features",[],["mod.rs","suggestions.rs"]],["matches",[],["arg_matches.rs","matched_arg.rs","mod.rs","value_source.rs"]]],["arg_matcher.rs","error.rs","mod.rs","parser.rs","validator.rs"]],["util",[],["any_value.rs","color.rs","flat_map.rs","flat_set.rs","graph.rs","id.rs","mod.rs","str_to_bool.rs"]]],["derive.rs","lib.rs","macros.rs","mkeymap.rs"]]],\
["clap_derive",["",[["derives",[],["args.rs","into_app.rs","mod.rs","parser.rs","subcommand.rs","value_enum.rs"]],["utils",[],["doc_comments.rs","error.rs","mod.rs","spanned.rs","ty.rs"]]],["attr.rs","dummies.rs","item.rs","lib.rs","macros.rs"]]],\
["clap_lex",["",[],["ext.rs","lib.rs"]]],\
["colorchoice",["",[],["lib.rs"]]],\
["crossbeam_channel",["",[["flavors",[],["array.rs","at.rs","list.rs","mod.rs","never.rs","tick.rs","zero.rs"]]],["channel.rs","context.rs","counter.rs","err.rs","lib.rs","select.rs","select_macro.rs","utils.rs","waker.rs"]]],\
["crossbeam_queue",["",[],["array_queue.rs","lib.rs","seg_queue.rs"]]],\
["crossbeam_utils",["",[["atomic",[],["atomic_cell.rs","consume.rs","mod.rs","seq_lock.rs"]],["sync",[],["mod.rs","once_lock.rs","parker.rs","sharded_lock.rs","wait_group.rs"]]],["backoff.rs","cache_padded.rs","lib.rs","thread.rs"]]],\
["file_id",["",[],["lib.rs"]]],\
["filetime",["",[["unix",[],["linux.rs","mod.rs","utimes.rs"]]],["lib.rs"]]],\
["flexi_logger",["",[["parameters",[],["age.rs","cleanup.rs","criterion.rs","file_spec.rs","naming.rs"]],["primary_writer",[],["multi_writer.rs","std_stream.rs","std_writer.rs","test_writer.rs"]],["writers",[["file_log_writer",[["state",[],["list_and_cleanup.rs","numbers.rs","timestamps.rs"]]],["builder.rs","config.rs","state.rs","state_handle.rs","threads.rs"]]],["file_log_writer.rs","log_writer.rs"]]],["code_examples.rs","deferred_now.rs","error_info.rs","filter.rs","flexi_error.rs","flexi_logger.rs","formats.rs","lib.rs","log_specification.rs","logger.rs","logger_handle.rs","parameters.rs","primary_writer.rs","threads.rs","util.rs","write_mode.rs","writers.rs"]]],\
["fs_err",["",[["os",[],["unix.rs"]]],["dir.rs","errors.rs","file.rs","lib.rs","open_options.rs","os.rs","path.rs"]]],\
["glfw",["",[["ffi",[],["link.rs","mod.rs"]]],["callbacks.rs","lib.rs"]]],\
["glfw_sys",["",[],["lib.rs"]]],\
["heck",["",[],["kebab.rs","lib.rs","lower_camel.rs","shouty_kebab.rs","shouty_snake.rs","snake.rs","title.rs","train.rs","upper_camel.rs"]]],\
["iana_time_zone",["",[],["ffi_utils.rs","lib.rs","tz_linux.rs"]]],\
["indoc",["",[],["error.rs","expr.rs","lib.rs","unindent.rs"]]],\
["inotify",["",[],["events.rs","fd_guard.rs","inotify.rs","lib.rs","util.rs","watches.rs"]]],\
["inotify_sys",["",[],["lib.rs"]]],\
["is_terminal_polyfill",["",[],["lib.rs"]]],\
["libc",["",[["unix",[["linux_like",[["linux",[["arch",[["generic",[],["mod.rs"]]],["mod.rs"]],["gnu",[["b64",[["x86_64",[],["align.rs","mod.rs","not_x32.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["align.rs","mod.rs","non_exhaustive.rs"]]],["mod.rs"]]],["align.rs","mod.rs"]]],["fixed_width_ints.rs","lib.rs","macros.rs"]]],\
["libloading",["",[["os",[["unix",[],["consts.rs","mod.rs"]]],["mod.rs"]]],["changelog.rs","error.rs","lib.rs","safe.rs","util.rs"]]],\
["linux_raw_sys",["",[["x86_64",[],["errno.rs","general.rs","ioctl.rs"]]],["elf.rs","lib.rs"]]],\
["lock_api",["",[],["lib.rs","mutex.rs","remutex.rs","rwlock.rs"]]],\
["log",["",[],["__private_api.rs","lib.rs","macros.rs"]]],\
["memchr",["",[["arch",[["all",[["packedpair",[],["default_rank.rs","mod.rs"]]],["memchr.rs","mod.rs","rabinkarp.rs","shiftor.rs","twoway.rs"]],["generic",[],["memchr.rs","mod.rs","packedpair.rs"]],["x86_64",[["avx2",[],["memchr.rs","mod.rs","packedpair.rs"]],["sse2",[],["memchr.rs","mod.rs","packedpair.rs"]]],["memchr.rs","mod.rs"]]],["mod.rs"]],["memmem",[],["mod.rs","searcher.rs"]]],["cow.rs","ext.rs","lib.rs","macros.rs","memchr.rs","vector.rs"]]],\
["mio",["",[["event",[],["event.rs","events.rs","mod.rs","source.rs"]],["sys",[["unix",[["selector",[],["epoll.rs","mod.rs"]]],["mod.rs","pipe.rs","sourcefd.rs","waker.rs"]]],["mod.rs"]]],["interest.rs","io_source.rs","lib.rs","macros.rs","poll.rs","token.rs","waker.rs"]]],\
["notify",["",[],["config.rs","error.rs","event.rs","inotify.rs","lib.rs","null.rs","poll.rs"]]],\
["notify_debouncer_full",["",[],["cache.rs","debounced_event.rs","lib.rs"]]],\
["nu_ansi_term",["",[],["ansi.rs","debug.rs","difference.rs","display.rs","gradient.rs","lib.rs","rgb.rs","style.rs","util.rs","windows.rs","write.rs"]]],\
["num_traits",["",[["ops",[],["bytes.rs","checked.rs","euclid.rs","inv.rs","mod.rs","mul_add.rs","overflowing.rs","saturating.rs","wrapping.rs"]]],["bounds.rs","cast.rs","float.rs","identities.rs","int.rs","lib.rs","macros.rs","pow.rs","sign.rs"]]],\
["os_str_bytes",["",[["common",[],["convert_io.rs","mod.rs"]]],["ext.rs","iter.rs","lib.rs","pattern.rs","raw_str.rs","util.rs"]]],\
["parking_lot",["",[],["condvar.rs","deadlock.rs","elision.rs","fair_mutex.rs","lib.rs","mutex.rs","once.rs","raw_fair_mutex.rs","raw_mutex.rs","raw_rwlock.rs","remutex.rs","rwlock.rs","util.rs"]]],\
["parking_lot_core",["",[["thread_parker",[],["linux.rs","mod.rs"]]],["lib.rs","parking_lot.rs","spinwait.rs","util.rs","word_lock.rs"]]],\
["proc_macro2",["",[],["detection.rs","extra.rs","fallback.rs","lib.rs","marker.rs","parse.rs","rcvec.rs","wrapper.rs"]]],\
["quote",["",[],["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]]],\
["raw_window_handle",["",[],["android.rs","appkit.rs","borrowed.rs","haiku.rs","lib.rs","ohos.rs","redox.rs","uikit.rs","unix.rs","web.rs","windows.rs"]]],\
["regex",["",[["regex",[],["bytes.rs","mod.rs","string.rs"]],["regexset",[],["bytes.rs","mod.rs","string.rs"]]],["builders.rs","bytes.rs","error.rs","find_byte.rs","lib.rs"]]],\
["regex_automata",["",[["dfa",[],["mod.rs","onepass.rs","remapper.rs"]],["hybrid",[],["dfa.rs","error.rs","id.rs","mod.rs","regex.rs","search.rs"]],["meta",[],["error.rs","limited.rs","literal.rs","mod.rs","regex.rs","reverse_inner.rs","stopat.rs","strategy.rs","wrappers.rs"]],["nfa",[["thompson",[],["backtrack.rs","builder.rs","compiler.rs","error.rs","literal_trie.rs","map.rs","mod.rs","nfa.rs","pikevm.rs","range_trie.rs"]]],["mod.rs"]],["util",[["determinize",[],["mod.rs","state.rs"]],["prefilter",[],["aho_corasick.rs","byteset.rs","memchr.rs","memmem.rs","mod.rs","teddy.rs"]],["unicode_data",[],["mod.rs"]]],["alphabet.rs","captures.rs","empty.rs","escape.rs","int.rs","interpolate.rs","iter.rs","lazy.rs","look.rs","memchr.rs","mod.rs","pool.rs","primitives.rs","search.rs","sparse_set.rs","start.rs","syntax.rs","utf8.rs","wire.rs"]]],["lib.rs","macros.rs"]]],\
["regex_syntax",["",[["ast",[],["mod.rs","parse.rs","print.rs","visitor.rs"]],["hir",[],["interval.rs","literal.rs","mod.rs","print.rs","translate.rs","visitor.rs"]],["unicode_tables",[],["age.rs","case_folding_simple.rs","general_category.rs","grapheme_cluster_break.rs","mod.rs","perl_word.rs","property_bool.rs","property_names.rs","property_values.rs","script.rs","script_extension.rs","sentence_break.rs","word_break.rs"]]],["debug.rs","either.rs","error.rs","lib.rs","parser.rs","rank.rs","unicode.rs","utf8.rs"]]],\
["rustix",["",[["backend",[["linux_raw",[["arch",[],["mod.rs","x86_64.rs"]],["io",[],["errno.rs","mod.rs","syscalls.rs","types.rs"]],["termios",[],["mod.rs","syscalls.rs"]]],["c.rs","conv.rs","mod.rs","reg.rs"]]]],["io",[],["close.rs","dup.rs","errno.rs","fcntl.rs","ioctl.rs","mod.rs","read_write.rs"]],["ioctl",[],["linux.rs","mod.rs","patterns.rs"]],["maybe_polyfill",[["std",[],["mod.rs"]]]],["termios",[],["ioctl.rs","mod.rs","tc.rs","tty.rs","types.rs"]]],["bitcast.rs","buffer.rs","cstr.rs","ffi.rs","lib.rs","pid.rs","utils.rs","weak.rs"]]],\
["same_file",["",[],["lib.rs","unix.rs"]]],\
["scopeguard",["",[],["lib.rs"]]],\
["smallvec",["",[],["lib.rs"]]],\
["smawk",["",[],["lib.rs","monge.rs"]]],\
["strsim",["",[],["lib.rs"]]],\
["sts",["",[["app",[],["fullscreen_toggle.rs","logging.rs","mod.rs"]],["graphics",[["fullscreen_quad",[],["descriptors.rs","mod.rs","pipeline.rs","uniform_buffer.rs"]],["vulkan",[["device",[["instance",[],["debug.rs","mod.rs"]]],["logical_device.rs","mod.rs","physical_device.rs"]],["raii",[],["device.rs","device_extensions.rs","device_resources.rs","instance.rs","instance_extensions.rs","mod.rs"]],["swapchain",[],["mod.rs","settings.rs"]]],["frames_in_flight.rs","mod.rs"]]],["mod.rs","recompiler.rs"]]],["lib.rs"]]],\
["syn",["",[["gen",[],["clone.rs"]]],["attr.rs","bigint.rs","buffer.rs","classify.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","drops.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","fixup.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","meta.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","precedence.rs","print.rs","punctuated.rs","restriction.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","ty.rs","verbatim.rs","whitespace.rs"]]],\
["terminal_size",["",[],["lib.rs","unix.rs"]]],\
["textwrap",["",[["wrap_algorithms",[],["optimal_fit.rs"]]],["columns.rs","core.rs","fill.rs","indentation.rs","lib.rs","line_ending.rs","options.rs","refill.rs","termwidth.rs","word_separators.rs","word_splitters.rs","wrap.rs","wrap_algorithms.rs"]]],\
["thiserror",["",[],["aserror.rs","display.rs","lib.rs"]]],\
["thiserror_impl",["",[],["ast.rs","attr.rs","expand.rs","fmt.rs","generics.rs","lib.rs","prop.rs","span.rs","valid.rs"]]],\
["unicode_ident",["",[],["lib.rs","tables.rs"]]],\
["unicode_linebreak",["",[],["lib.rs","shared.rs"]]],\
["unicode_width",["",[],["lib.rs","tables.rs"]]],\
["utf8parse",["",[],["lib.rs","types.rs"]]],\
["walkdir",["",[],["dent.rs","error.rs","lib.rs","util.rs"]]]\
]'));
createSrcSidebar();
