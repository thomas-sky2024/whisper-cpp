-------------------------------------
Translated Report (Full Report Below)
-------------------------------------
Process:             auto-sub [42450]
Path:                /Users/USER/Documents/*/auto-sub
Identifier:          auto-sub
Version:             0.1.2 (0.1.2)
Code Type:           ARM-64 (Native)
Role:                Foreground
Parent Process:      node [42215]
Coalition:           com.google.antigravity [515]
Responsible Process: Electron [453]
User ID:             501

Date/Time:           2026-04-07 09:23:15.4815 +0700
Launch Time:         2026-04-07 09:16:06.6605 +0700
Hardware Model:      MacBookPro18,2
OS Version:          macOS 26.4 (25E246)
Release Type:        User

Crash Reporter Key:  E601F452-0C7D-49A2-E3D9-1DAD517AD943
Incident Identifier: 4594628D-3084-4D43-8D27-607FA8A382D9

Time Awake Since Boot: 6500 seconds

System Integrity Protection: enabled

Triggered by Thread: 0  main, Dispatch Queue: com.apple.main-thread

Exception Type:    EXC_CRASH (SIGABRT)
Exception Codes:   0x0000000000000000, 0x0000000000000000

Termination Reason:  Namespace SIGNAL, Code 6, Abort trap: 6
Terminating Process: auto-sub [42450]


Application Specific Information:
abort() called


Thread 0 Crashed:: main Dispatch queue: com.apple.main-thread
0   libsystem_kernel.dylib        	       0x18a4905e8 __pthread_kill + 8
1   libsystem_pthread.dylib       	       0x18a4cb8d8 pthread_kill + 296
2   libsystem_c.dylib             	       0x18a3d2790 abort + 148
3   auto-sub                      	       0x1056a7e54 ggml_abort + 160
4   auto-sub                      	       0x105726000 ggml_metal_rsets_free + 152
5   auto-sub                      	       0x1057268fc ggml_metal_device_free + 24
6   auto-sub                      	       0x105727b28 std::__1::unique_ptr<ggml_metal_device, ggml_metal_device_deleter>::~unique_ptr[abi:ne200100]() + 32
7   libsystem_c.dylib             	       0x18a381e48 __cxa_finalize_ranges + 416
8   libsystem_c.dylib             	       0x18a381c48 exit + 44
9   AppKit                        	       0x18ebb4a18 -[NSApplication terminate:] + 2004
10  AppKit                        	       0x18eb4d70c -[NSApplication(NSResponder) sendAction:to:from:] + 560
11  AppKit                        	       0x18f3e3c00 -[NSMenuItem _corePerformAction:] + 540
12  AppKit                        	       0x18f5733a8 _NSMenuPerformActionWithHighlighting + 160
13  AppKit                        	       0x18f3ceeec -[NSMenu _performKeyEquivalentForItemAtIndex:] + 172
14  AppKit                        	       0x18ec14888 -[NSMenu performKeyEquivalent:] + 356
15  AppKit                        	       0x18f549214 routeKeyEquivalent + 444
16  AppKit                        	       0x18f547430 -[NSApplication(NSEventRouting) sendEvent:] + 1844
17  auto-sub                      	       0x10674ca40 _$LT$$LP$A$C$$RP$$u20$as$u20$objc2..encode..EncodeArguments$GT$::__invoke::h47eafae6461ef75c + 88
18  auto-sub                      	       0x1065cf90c objc2::runtime::message_receiver::msg_send_primitive::send_super::h25ece48a0f2119d1 + 160
19  auto-sub                      	       0x1065a4b44 objc2::runtime::message_receiver::MessageReceiver::send_super_message::h4f5128159097e5cb + 224
20  auto-sub                      	       0x106596d3c _$LT$MethodFamily$u20$as$u20$objc2..__macro_helpers..msg_send_retained..MsgSendSuper$LT$Receiver$C$Return$GT$$GT$::send_super_message::h6a04df32c702f6a4 + 192
21  auto-sub                      	       0x10659fac4 tao::platform_impl::platform::app::send_event::h083c0f4ab32778e3 + 332
22  WebKit                        	       0x1b5136e20 WebKit::WebViewImpl::doneWithKeyEvent(NSEvent*, bool) + 168
23  WebKit                        	       0x1b46aed94 WebKit::PageClientImpl::doneWithKeyEvent(WebKit::NativeWebKeyboardEvent const&, bool) + 84
24  WebKit                        	       0x1b52b7f40 WebKit::WebPageProxy::didReceiveEvent(IPC::Connection*, WebKit::WebEventType, bool, std::__1::optional<WebCore::RemoteUserInputEventData>&&) + 1284
25  WebKit                        	       0x1b4ce8128 WebKit::WebPageProxy::didReceiveMessage(IPC::Connection&, IPC::Decoder&) + 14024
26  WebKit                        	       0x1b588da04 IPC::MessageReceiverMap::dispatchMessage(IPC::Connection&, IPC::Decoder&) + 348
27  WebKit                        	       0x1b5338654 WebKit::WebProcessProxy::dispatchMessage(IPC::Connection&, IPC::Decoder&) + 40
28  WebKit                        	       0x1b4d63a40 WebKit::WebProcessProxy::didReceiveMessage(IPC::Connection&, IPC::Decoder&) + 1016
29  WebKit                        	       0x1b58641d0 IPC::Connection::dispatchMessage(WTF::UniqueRef<IPC::Decoder>) + 324
30  WebKit                        	       0x1b58646c0 IPC::Connection::dispatchIncomingMessages() + 536
31  JavaScriptCore                	       0x1ac4fa480 WTF::RunLoop::performWork() + 460
32  JavaScriptCore                	       0x1ac4fba28 WTF::RunLoop::performWork(void*) + 36
33  CoreFoundation                	       0x18a588bc0 __CFRUNLOOP_IS_CALLING_OUT_TO_A_SOURCE0_PERFORM_FUNCTION__ + 28
34  CoreFoundation                	       0x18a588b54 __CFRunLoopDoSource0 + 172
35  CoreFoundation                	       0x18a5888c0 __CFRunLoopDoSources0 + 232
36  CoreFoundation                	       0x18a5874e4 __CFRunLoopRun + 820
37  CoreFoundation                	       0x18a659be0 _CFRunLoopRunSpecificWithOptions + 532
38  HIToolbox                     	       0x19735c560 RunCurrentEventLoopInMode + 320
39  HIToolbox                     	       0x19735f8bc ReceiveNextEventCommon + 488
40  HIToolbox                     	       0x1974e913c _BlockUntilNextEventMatchingListInMode + 48
41  AppKit                        	       0x18f05f1a4 _DPSBlockUntilNextEventMatchingListInMode + 228
42  AppKit                        	       0x18e9b3084 _DPSNextEvent + 576
43  AppKit                        	       0x18f54869c -[NSApplication(NSEventRouting) _nextEventMatchingEventMask:untilDate:inMode:dequeue:] + 688
44  AppKit                        	       0x18f5483a8 -[NSApplication(NSEventRouting) nextEventMatchingMask:untilDate:inMode:dequeue:] + 72
45  AppKit                        	       0x18e9a613c -[NSApplication run] + 368
46  auto-sub                      	       0x10676b5ec _$LT$$LP$$RP$$u20$as$u20$objc2..encode..EncodeArguments$GT$::__invoke::hf7ebbc423677f3ce + 52
47  auto-sub                      	       0x10676810c objc2::runtime::message_receiver::msg_send_primitive::send::ha386c6e6d62d3b39 + 60
48  auto-sub                      	       0x10675f988 objc2::runtime::message_receiver::MessageReceiver::send_message::h2635635de8835a70 + 176
49  auto-sub                      	       0x1051365a0 _$LT$MethodFamily$u20$as$u20$objc2..__macro_helpers..msg_send_retained..MsgSend$LT$Receiver$C$Return$GT$$GT$::send_message::heebd85b400119f91 + 152
50  auto-sub                      	       0x1051d6a0c tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run_return::hadf717a172d1ad3e + 640
51  auto-sub                      	       0x1051d6f08 tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run::h5bedd1a1d0393d84 + 52
52  auto-sub                      	       0x10525bb0c tao::event_loop::EventLoop$LT$T$GT$::run::h2f3698526c56c9cf + 88
53  auto-sub                      	       0x104f2b50c _$LT$tauri_runtime_wry..Wry$LT$T$GT$$u20$as$u20$tauri_runtime..Runtime$LT$T$GT$$GT$::run::h0edfc140ae436836 + 172
54  auto-sub                      	       0x104f66620 tauri::app::App$LT$R$GT$::run::h5a76840d935e3800 + 640
55  auto-sub                      	       0x104f66ca0 tauri::app::Builder$LT$R$GT$::run::h907919b5b4253f48 + 284
56  auto-sub                      	       0x105135510 auto_sub_lib::run::h8256e938c1cb1ebf + 776
57  auto-sub                      	       0x104e913c8 auto_sub::main::ha0c220c0bb59a533 + 12 (main.rs:5)
58  auto-sub                      	       0x104e9129c core::ops::function::FnOnce::call_once::ha9e8ad9b93c8b446 + 20 (function.rs:250)
59  auto-sub                      	       0x104e913a0 std::sys::backtrace::__rust_begin_short_backtrace::h48345fc226584c3b + 24 (backtrace.rs:166)
60  auto-sub                      	       0x104e91370 std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h77db1f6bead99b3b + 28 (rt.rs:206)
61  auto-sub                      	       0x106ad78fc std::rt::lang_start_internal::h11fc0900699d88c7 + 952
62  auto-sub                      	       0x104e91348 std::rt::lang_start::h04c53eeb3f84f887 + 84 (rt.rs:205)
63  auto-sub                      	       0x104e913f4 main + 36
64  dyld                          	       0x18a10fda4 start + 6992

Thread 1::  Dispatch queue: com.apple.root.default-qos
0   libsystem_kernel.dylib        	       0x18a48b308 __semwait_signal + 8
1   libsystem_c.dylib             	       0x18a367cc0 nanosleep + 220
2   libsystem_c.dylib             	       0x18a367bd8 usleep + 68
3   auto-sub                      	       0x105725f00 __ggml_metal_rsets_init_block_invoke + 92
4   libdispatch.dylib             	       0x18a310a28 _dispatch_call_block_and_release + 32
5   libdispatch.dylib             	       0x18a32a4b0 _dispatch_client_callout + 16
6   libdispatch.dylib             	       0x18a347d9c <deduplicated_symbol> + 76
7   libdispatch.dylib             	       0x18a322adc _dispatch_root_queue_drain + 708
8   libdispatch.dylib             	       0x18a323120 _dispatch_worker_thread2 + 184
9   libsystem_pthread.dylib       	       0x18a4c7e84 _pthread_wqthread + 232
10  libsystem_pthread.dylib       	       0x18a4c6c10 start_wqthread + 8

Thread 2:: com.apple.NSEventThread
0   libsystem_kernel.dylib        	       0x18a487c34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x18a49a574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x18a4909c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x18a487fc0 mach_msg + 24
4   CoreFoundation                	       0x18a588d68 __CFRunLoopServiceMachPort + 160
5   CoreFoundation                	       0x18a587654 __CFRunLoopRun + 1188
6   CoreFoundation                	       0x18a659be0 _CFRunLoopRunSpecificWithOptions + 532
7   AppKit                        	       0x18ead4c64 _NSEventThread + 184
8   libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
9   libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 3:: WebCore: Scrolling
0   libsystem_kernel.dylib        	       0x18a487c34 mach_msg2_trap + 8
1   libsystem_kernel.dylib        	       0x18a49a574 mach_msg2_internal + 76
2   libsystem_kernel.dylib        	       0x18a4909c0 mach_msg_overwrite + 480
3   libsystem_kernel.dylib        	       0x18a487fc0 mach_msg + 24
4   CoreFoundation                	       0x18a588d68 __CFRunLoopServiceMachPort + 160
5   CoreFoundation                	       0x18a587654 __CFRunLoopRun + 1188
6   CoreFoundation                	       0x18a659be0 _CFRunLoopRunSpecificWithOptions + 532
7   CoreFoundation                	       0x18a5fd524 CFRunLoopRun + 64
8   JavaScriptCore                	       0x1ac4fb030 WTF::Detail::CallableWrapper<WTF::RunLoop::create(WTF::ASCIILiteral, WTF::ThreadType, WTF::Thread::QOS)::$_0, void>::call() + 240
9   JavaScriptCore                	       0x1ac5428cc WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*) + 296
10  JavaScriptCore                	       0x1ac30f088 WTF::wtfThreadEntryPoint(void*) + 16
11  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
12  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 4:: Log work queue
0   libsystem_kernel.dylib        	       0x18a487bb0 semaphore_wait_trap + 8
1   WebKit                        	       0x1b5893ab0 IPC::StreamConnectionWorkQueue::startProcessingThread()::$_0::operator()() + 48
2   JavaScriptCore                	       0x1ac5428cc WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*) + 296
3   JavaScriptCore                	       0x1ac30f088 WTF::wtfThreadEntryPoint(void*) + 16
4   libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
5   libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 5:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 6:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 7:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 8:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 9:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 10:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 11:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48dfc4 kevent + 8
1   auto-sub                      	       0x106704840 mio::sys::unix::selector::Selector::select::hafab6b932786ff95 + 200
2   auto-sub                      	       0x1067008a0 mio::poll::Poll::poll::hbae0e0a074cec30d + 80
3   auto-sub                      	       0x1066dfcfc tokio::runtime::io::driver::Driver::turn::h8955633b70a6deec + 200
4   auto-sub                      	       0x1066dfc28 tokio::runtime::io::driver::Driver::park::h02b5ad2959686a4d + 80
5   auto-sub                      	       0x1066f0f74 tokio::runtime::signal::Driver::park::h1a2a4813cae6b2c0 + 36
6   auto-sub                      	       0x1066d2280 tokio::runtime::process::Driver::park::h7225895c596a7a4c + 32
7   auto-sub                      	       0x1066f0b18 tokio::runtime::driver::IoStack::park::h2b268599b72bfcf2 + 104
8   auto-sub                      	       0x1066bb6e0 tokio::runtime::time::Driver::park_internal::he14f614974152d7b + 424
9   auto-sub                      	       0x1066bbb14 tokio::runtime::time::Driver::park::h5cc4a76dcd959357 + 40
10  auto-sub                      	       0x1066efdf0 tokio::runtime::driver::TimeDriver::park::h02c9f591939bf24f + 92
11  auto-sub                      	       0x1066f08ec tokio::runtime::driver::Driver::park::h271244f23ac59555 + 32
12  auto-sub                      	       0x1066e92a8 tokio::runtime::scheduler::multi_thread::park::Inner::park_driver::hd4e08710b10a9ece + 264
13  auto-sub                      	       0x1066e9ce4 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 240
14  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
15  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
16  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
17  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
18  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
19  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
20  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
21  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
22  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
23  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
24  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
25  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
26  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
27  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
28  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
29  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
30  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
31  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
32  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
33  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
34  auto-sub                      	       0x10668c2dc __rust_try + 32
35  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
36  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
37  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
38  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
39  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
40  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
41  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
42  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
43  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
44  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
45  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
46  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
47  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
48  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
49  auto-sub                      	       0x1066bd560 __rust_try + 32
50  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
51  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
52  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
53  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
54  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 12:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 13:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 14:: tokio-rt-worker
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   auto-sub                      	       0x10670d1ec _$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104 + 228
3   auto-sub                      	       0x10670be4c parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a + 748
4   auto-sub                      	       0x10670a9f4 parking_lot_core::parking_lot::park::h5f72b57337891465 + 296
5   auto-sub                      	       0x10670d8b8 parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed + 160
6   auto-sub                      	       0x1066db48c parking_lot::condvar::Condvar::wait::hc638ca3865a39696 + 68
7   auto-sub                      	       0x10668d97c tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65 + 36
8   auto-sub                      	       0x1066e95cc tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349 + 412
9   auto-sub                      	       0x1066e9ca8 tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9 + 180
10  auto-sub                      	       0x1066ea0e8 tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7 + 40
11  auto-sub                      	       0x1066d4be0 tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49 + 872
12  auto-sub                      	       0x1066d5bd4 tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25 + 968
13  auto-sub                      	       0x1066d56d0 tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d + 1764
14  auto-sub                      	       0x1066d281c tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f + 104
15  auto-sub                      	       0x1066ce570 tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e + 148
16  auto-sub                      	       0x1066e0e54 tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb + 40
17  auto-sub                      	       0x1066c4fc4 std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b + 196
18  auto-sub                      	       0x1066c3878 std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb + 24
19  auto-sub                      	       0x1066e0d98 tokio::runtime::context::set_scheduler::hc53b125ae381926a + 68
20  auto-sub                      	       0x1066d2740 tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f + 248
21  auto-sub                      	       0x1066bfaa8 tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16 + 176
22  auto-sub                      	       0x1066d25e8 tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d + 600
23  auto-sub                      	       0x1066d37a8 tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3 + 24
24  auto-sub                      	       0x1066c8f14 _$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68 + 136
25  auto-sub                      	       0x106689cb4 tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6 + 192
26  auto-sub                      	       0x1066896dc tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c + 72
27  auto-sub                      	       0x10668522c tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce + 64
28  auto-sub                      	       0x1066ded14 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39 + 44
29  auto-sub                      	       0x1066e5468 std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f + 64
30  auto-sub                      	       0x10668c2dc __rust_try + 32
31  auto-sub                      	       0x10668b934 std::panic::catch_unwind::h7cbcd30bdfdad2a1 + 116
32  auto-sub                      	       0x106684efc tokio::runtime::task::harness::poll_future::h8fecbf895a76008b + 96
33  auto-sub                      	       0x106685a38 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356 + 172
34  auto-sub                      	       0x1066866e8 tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd + 28
35  auto-sub                      	       0x1066c5e9c tokio::runtime::task::raw::poll::h9424b663b968901f + 36
36  auto-sub                      	       0x1066c665c tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a + 52
37  auto-sub                      	       0x1066ed780 tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305 + 64
38  auto-sub                      	       0x1066af398 tokio::runtime::blocking::pool::Task::run::h53008843b290fab9 + 28
39  auto-sub                      	       0x1066af5ac tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c + 520
40  auto-sub                      	       0x1066b0488 tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee + 144
41  auto-sub                      	       0x1066aacf0 std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5 + 16
42  auto-sub                      	       0x1066b55e8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab + 116
43  auto-sub                      	       0x1066df048 _$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4 + 44
44  auto-sub                      	       0x1066e59b8 std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0 + 52
45  auto-sub                      	       0x1066bd560 __rust_try + 32
46  auto-sub                      	       0x1066b51f8 std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b + 260
47  auto-sub                      	       0x1066998b4 core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401 + 24
48  auto-sub                      	       0x106add0a0 std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48 + 408
49  libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
50  libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8

Thread 15:

Thread 16:

Thread 17:

Thread 18:

Thread 19:

Thread 20:: JavaScriptCore libpas scavenger
0   libsystem_kernel.dylib        	       0x18a48b50c __psynch_cvwait + 8
1   libsystem_pthread.dylib       	       0x18a4cc128 _pthread_cond_wait + 980
2   JavaScriptCore                	       0x1adbb6c38 scavenger_thread_main + 1416
3   libsystem_pthread.dylib       	       0x18a4cbc58 _pthread_start + 136
4   libsystem_pthread.dylib       	       0x18a4c6c1c thread_start + 8


Thread 0 crashed with ARM Thread State (64-bit):
    x0: 0x0000000000000000   x1: 0x0000000000000000   x2: 0x0000000000000000   x3: 0x0000000000000000
    x4: 0x000000018a3d58b7   x5: 0x000000016af619c0   x6: 0x0000000000000032   x7: 0x0000000000000000
    x8: 0x15b6ecc7d3ea81ff   x9: 0x15b6ecc625c7d93f  x10: 0x0000000000000002  x11: 0x00000000fffffffd
   x12: 0x0000000000000000  x13: 0x0000000000000000  x14: 0x0000000000000000  x15: 0x0000000000000000
   x16: 0x0000000000000148  x17: 0x00000001f790df20  x18: 0x0000000000000000  x19: 0x0000000000000006
   x20: 0x0000000000000103  x21: 0x00000001f62d59a0  x22: 0x0000000a3532de00  x23: 0x0000000000000002
   x24: 0x0000000000000008  x25: 0x00000001f62df000  x26: 0x0000000000000015  x27: 0x0000000a3532de10
   x28: 0x0000000000000014   fp: 0x000000016af622b0   lr: 0x000000018a4cb8d8
    sp: 0x000000016af62290   pc: 0x000000018a4905e8 cpsr: 0x40001000
   far: 0x0000000000000000  esr: 0x56000080 (Syscall)

Binary Images:
       0x104e90000 -        0x10793ffff auto-sub (*) <bd8d513f-f78d-3466-a1d3-102ad81e6268> /Users/USER/Documents/*/auto-sub
       0x109a20000 -        0x109a2bfff libobjc-trampolines.dylib (*) <a4dd56f1-375a-3540-844b-5e397f0b78b3> /usr/lib/libobjc-trampolines.dylib
       0x12345c000 -        0x123be7fff com.apple.AGXMetalG13X (350.38) <95c14223-fa99-3e3d-9570-8e7862d86a54> /System/Library/Extensions/AGXMetalG13X.bundle/Contents/MacOS/AGXMetalG13X
       0x1220a0000 -        0x122103fff com.apple.AppleMetalOpenGLRenderer (1.0) <0a5a2e2b-9899-3606-af8c-84850db23fea> /System/Library/Extensions/AppleMetalOpenGLRenderer.bundle/Contents/MacOS/AppleMetalOpenGLRenderer
       0x18a487000 -        0x18a4c428f libsystem_kernel.dylib (*) <51565b39-f595-3e96-a217-fef29815057a> /usr/lib/system/libsystem_kernel.dylib
       0x18a4c5000 -        0x18a4d1b3b libsystem_pthread.dylib (*) <e7a73008-0c09-31e3-9dd9-0c61652f0e85> /usr/lib/system/libsystem_pthread.dylib
       0x18a35a000 -        0x18a3daef7 libsystem_c.dylib (*) <66ebd321-6899-3863-ba24-5cfc3076a0cb> /usr/lib/system/libsystem_c.dylib
       0x18e97a000 -        0x19009cabf com.apple.AppKit (6.9) <59e23bd5-d01e-305a-b96f-a5790356049a> /System/Library/Frameworks/AppKit.framework/Versions/C/AppKit
       0x1b4682000 -        0x1b5c5b3df com.apple.WebKit (21624) <e7e04c4f-689c-3e21-bd4a-c7591fd3d5ca> /System/Library/Frameworks/WebKit.framework/Versions/A/WebKit
       0x1ac309000 -        0x1addb1ddf com.apple.JavaScriptCore (21624) <cae1f78c-542e-30f7-8e08-fdca4b880e04> /System/Library/Frameworks/JavaScriptCore.framework/Versions/A/JavaScriptCore
       0x18a50b000 -        0x18aa68c5f com.apple.CoreFoundation (6.9) <04941709-2330-3bf8-9213-6d33964db448> /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
       0x19729f000 -        0x19759a05f com.apple.HIToolbox (2.1.1) <bcb81496-c81f-3d3e-a617-ccca047989e0> /System/Library/Frameworks/Carbon.framework/Versions/A/Frameworks/HIToolbox.framework/Versions/A/HIToolbox
       0x18a0f0000 -        0x18a195ec7 dyld (*) <9f682dcf-340c-3bfa-bcdd-dd702f30313e> /usr/lib/dyld
               0x0 - 0xffffffffffffffff ??? (*) <00000000-0000-0000-0000-000000000000> ???
       0x18a30f000 -        0x18a35623f libdispatch.dylib (*) <e17aa23f-db2a-3302-b14c-f6b08c540fcf> /usr/lib/system/libdispatch.dylib

External Modification Summary:
  Calls made by other processes targeting this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by this process:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0
  Calls made by all processes on this machine:
    task_for_pid: 0
    thread_create: 0
    thread_set_state: 0

-----------
Full Report
-----------

{"app_name":"auto-sub","timestamp":"2026-04-07 09:23:19.00 +0700","app_version":"0.1.2","slice_uuid":"bd8d513f-f78d-3466-a1d3-102ad81e6268","build_version":"0.1.2","platform":1,"share_with_app_devs":0,"is_first_party":1,"bug_type":"309","os_version":"macOS 26.4 (25E246)","roots_installed":0,"incident_id":"4594628D-3084-4D43-8D27-607FA8A382D9","name":"auto-sub"}
{
  "uptime" : 6500,
  "procRole" : "Foreground",
  "version" : 2,
  "userID" : 501,
  "deployVersion" : 210,
  "modelCode" : "MacBookPro18,2",
  "coalitionID" : 515,
  "osVersion" : {
    "train" : "macOS 26.4",
    "build" : "25E246",
    "releaseType" : "User"
  },
  "captureTime" : "2026-04-07 09:23:15.4815 +0700",
  "codeSigningMonitor" : 2,
  "incident" : "4594628D-3084-4D43-8D27-607FA8A382D9",
  "pid" : 42450,
  "translated" : false,
  "cpuType" : "ARM-64",
  "procLaunch" : "2026-04-07 09:16:06.6605 +0700",
  "procStartAbsTime" : 146579324115,
  "procExitAbsTime" : 156870308085,
  "procName" : "auto-sub",
  "procPath" : "\/Users\/USER\/Documents\/*\/auto-sub",
  "bundleInfo" : {"CFBundleVersion":"0.1.2","CFBundleShortVersionString":"0.1.2"},
  "parentProc" : "node",
  "parentPid" : 42215,
  "coalitionName" : "com.google.antigravity",
  "crashReporterKey" : "E601F452-0C7D-49A2-E3D9-1DAD517AD943",
  "appleIntelligenceStatus" : {"state":"available"},
  "developerMode" : 1,
  "responsiblePid" : 453,
  "responsibleProc" : "Electron",
  "codeSigningID" : "auto_sub-5604497674e4fca0",
  "codeSigningTeamID" : "",
  "codeSigningFlags" : 570556929,
  "codeSigningValidationCategory" : 10,
  "codeSigningTrustLevel" : 4294967295,
  "codeSigningAuxiliaryInfo" : 0,
  "instructionByteStream" : {"beforePC":"fyMD1f17v6n9AwCRCuD\/l78DAJH9e8Go\/w9f1sADX9YQKYDSARAA1A==","atPC":"AwEAVH8jA9X9e7+p\/QMAkf\/f\/5e\/AwCR\/XvBqP8PX9bAA1\/WcAqA0g=="},
  "bootSessionUUID" : "58BF86FC-3BA7-4DA5-BF70-944AE9731E03",
  "sip" : "enabled",
  "exception" : {"codes":"0x0000000000000000, 0x0000000000000000","rawCodes":[0,0],"type":"EXC_CRASH","signal":"SIGABRT"},
  "termination" : {"flags":0,"code":6,"namespace":"SIGNAL","indicator":"Abort trap: 6","byProc":"auto-sub","byPid":42450},
  "asi" : {"libsystem_c.dylib":["abort() called"]},
  "extMods" : {"caller":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"system":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"targeted":{"thread_create":0,"thread_set_state":0,"task_for_pid":0},"warnings":0},
  "faultingThread" : 0,
  "threads" : [{"threadState":{"x":[{"value":0},{"value":0},{"value":0},{"value":0},{"value":6614243511,"symbolLocation":0,"symbol":"__vfprintf.xdigs_lower"},{"value":6089480640},{"value":50},{"value":0},{"value":1564698263555834367},{"value":1564698256339360063},{"value":2},{"value":4294967293},{"value":0},{"value":0},{"value":0},{"value":0},{"value":328},{"value":8448433952},{"value":0},{"value":6},{"value":259},{"value":8425134496,"symbolLocation":224,"symbol":"_main_thread"},{"value":43842199040},{"value":2},{"value":8},{"value":8425172992,"symbolLocation":3288,"symbol":"usual_extra"},{"value":21},{"value":43842199056},{"value":20}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615251160},"cpsr":{"value":1073745920},"fp":{"value":6089482928},"sp":{"value":6089482896},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6615008744,"matchesCrashFrame":1},"far":{"value":0}},"id":222106,"triggered":true,"name":"main","queue":"com.apple.main-thread","frames":[{"imageOffset":38376,"symbol":"__pthread_kill","symbolLocation":8,"imageIndex":4},{"imageOffset":26840,"symbol":"pthread_kill","symbolLocation":296,"imageIndex":5},{"imageOffset":493456,"symbol":"abort","symbolLocation":148,"imageIndex":6},{"imageOffset":8486484,"symbol":"ggml_abort","symbolLocation":160,"imageIndex":0},{"imageOffset":9003008,"symbol":"ggml_metal_rsets_free","symbolLocation":152,"imageIndex":0},{"imageOffset":9005308,"symbol":"ggml_metal_device_free","symbolLocation":24,"imageIndex":0},{"imageOffset":9009960,"symbol":"std::__1::unique_ptr<ggml_metal_device, ggml_metal_device_deleter>::~unique_ptr[abi:ne200100]()","symbolLocation":32,"imageIndex":0},{"imageOffset":163400,"symbol":"__cxa_finalize_ranges","symbolLocation":416,"imageIndex":6},{"imageOffset":162888,"symbol":"exit","symbolLocation":44,"imageIndex":6},{"imageOffset":2337304,"symbol":"-[NSApplication terminate:]","symbolLocation":2004,"imageIndex":7},{"imageOffset":1914636,"symbol":"-[NSApplication(NSResponder) sendAction:to:from:]","symbolLocation":560,"imageIndex":7},{"imageOffset":10918912,"symbol":"-[NSMenuItem _corePerformAction:]","symbolLocation":540,"imageIndex":7},{"imageOffset":12555176,"symbol":"_NSMenuPerformActionWithHighlighting","symbolLocation":160,"imageIndex":7},{"imageOffset":10833644,"symbol":"-[NSMenu _performKeyEquivalentForItemAtIndex:]","symbolLocation":172,"imageIndex":7},{"imageOffset":2730120,"symbol":"-[NSMenu performKeyEquivalent:]","symbolLocation":356,"imageIndex":7},{"imageOffset":12382740,"symbol":"routeKeyEquivalent","symbolLocation":444,"imageIndex":7},{"imageOffset":12375088,"symbol":"-[NSApplication(NSEventRouting) sendEvent:]","symbolLocation":1844,"imageIndex":7},{"imageOffset":25938496,"symbol":"_$LT$$LP$A$C$$RP$$u20$as$u20$objc2..encode..EncodeArguments$GT$::__invoke::h47eafae6461ef75c","symbolLocation":88,"imageIndex":0},{"imageOffset":24377612,"symbol":"objc2::runtime::message_receiver::msg_send_primitive::send_super::h25ece48a0f2119d1","symbolLocation":160,"imageIndex":0},{"imageOffset":24202052,"symbol":"objc2::runtime::message_receiver::MessageReceiver::send_super_message::h4f5128159097e5cb","symbolLocation":224,"imageIndex":0},{"imageOffset":24145212,"symbol":"_$LT$MethodFamily$u20$as$u20$objc2..__macro_helpers..msg_send_retained..MsgSendSuper$LT$Receiver$C$Return$GT$$GT$::send_super_message::h6a04df32c702f6a4","symbolLocation":192,"imageIndex":0},{"imageOffset":24181444,"symbol":"tao::platform_impl::platform::app::send_event::h083c0f4ab32778e3","symbolLocation":332,"imageIndex":0},{"imageOffset":11226656,"symbol":"WebKit::WebViewImpl::doneWithKeyEvent(NSEvent*, bool)","symbolLocation":168,"imageIndex":8},{"imageOffset":183700,"symbol":"WebKit::PageClientImpl::doneWithKeyEvent(WebKit::NativeWebKeyboardEvent const&, bool)","symbolLocation":84,"imageIndex":8},{"imageOffset":12803904,"symbol":"WebKit::WebPageProxy::didReceiveEvent(IPC::Connection*, WebKit::WebEventType, bool, std::__1::optional<WebCore::RemoteUserInputEventData>&&)","symbolLocation":1284,"imageIndex":8},{"imageOffset":6709544,"symbol":"WebKit::WebPageProxy::didReceiveMessage(IPC::Connection&, IPC::Decoder&)","symbolLocation":14024,"imageIndex":8},{"imageOffset":18921988,"symbol":"IPC::MessageReceiverMap::dispatchMessage(IPC::Connection&, IPC::Decoder&)","symbolLocation":348,"imageIndex":8},{"imageOffset":13330004,"symbol":"WebKit::WebProcessProxy::dispatchMessage(IPC::Connection&, IPC::Decoder&)","symbolLocation":40,"imageIndex":8},{"imageOffset":7215680,"symbol":"WebKit::WebProcessProxy::didReceiveMessage(IPC::Connection&, IPC::Decoder&)","symbolLocation":1016,"imageIndex":8},{"imageOffset":18751952,"symbol":"IPC::Connection::dispatchMessage(WTF::UniqueRef<IPC::Decoder>)","symbolLocation":324,"imageIndex":8},{"imageOffset":18753216,"symbol":"IPC::Connection::dispatchIncomingMessages()","symbolLocation":536,"imageIndex":8},{"imageOffset":2036864,"symbol":"WTF::RunLoop::performWork()","symbolLocation":460,"imageIndex":9},{"imageOffset":2042408,"symbol":"WTF::RunLoop::performWork(void*)","symbolLocation":36,"imageIndex":9},{"imageOffset":515008,"symbol":"__CFRUNLOOP_IS_CALLING_OUT_TO_A_SOURCE0_PERFORM_FUNCTION__","symbolLocation":28,"imageIndex":10},{"imageOffset":514900,"symbol":"__CFRunLoopDoSource0","symbolLocation":172,"imageIndex":10},{"imageOffset":514240,"symbol":"__CFRunLoopDoSources0","symbolLocation":232,"imageIndex":10},{"imageOffset":509156,"symbol":"__CFRunLoopRun","symbolLocation":820,"imageIndex":10},{"imageOffset":1371104,"symbol":"_CFRunLoopRunSpecificWithOptions","symbolLocation":532,"imageIndex":10},{"imageOffset":775520,"symbol":"RunCurrentEventLoopInMode","symbolLocation":320,"imageIndex":11},{"imageOffset":788668,"symbol":"ReceiveNextEventCommon","symbolLocation":488,"imageIndex":11},{"imageOffset":2400572,"symbol":"_BlockUntilNextEventMatchingListInMode","symbolLocation":48,"imageIndex":11},{"imageOffset":7229860,"symbol":"_DPSBlockUntilNextEventMatchingListInMode","symbolLocation":228,"imageIndex":7},{"imageOffset":233604,"symbol":"_DPSNextEvent","symbolLocation":576,"imageIndex":7},{"imageOffset":12379804,"symbol":"-[NSApplication(NSEventRouting) _nextEventMatchingEventMask:untilDate:inMode:dequeue:]","symbolLocation":688,"imageIndex":7},{"imageOffset":12379048,"symbol":"-[NSApplication(NSEventRouting) nextEventMatchingMask:untilDate:inMode:dequeue:]","symbolLocation":72,"imageIndex":7},{"imageOffset":180540,"symbol":"-[NSApplication run]","symbolLocation":368,"imageIndex":7},{"imageOffset":26064364,"symbol":"_$LT$$LP$$RP$$u20$as$u20$objc2..encode..EncodeArguments$GT$::__invoke::hf7ebbc423677f3ce","symbolLocation":52,"imageIndex":0},{"imageOffset":26050828,"symbol":"objc2::runtime::message_receiver::msg_send_primitive::send::ha386c6e6d62d3b39","symbolLocation":60,"imageIndex":0},{"imageOffset":26016136,"symbol":"objc2::runtime::message_receiver::MessageReceiver::send_message::h2635635de8835a70","symbolLocation":176,"imageIndex":0},{"imageOffset":2778528,"symbol":"_$LT$MethodFamily$u20$as$u20$objc2..__macro_helpers..msg_send_retained..MsgSend$LT$Receiver$C$Return$GT$$GT$::send_message::heebd85b400119f91","symbolLocation":152,"imageIndex":0},{"imageOffset":3435020,"symbol":"tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run_return::hadf717a172d1ad3e","symbolLocation":640,"imageIndex":0},{"imageOffset":3436296,"symbol":"tao::platform_impl::platform::event_loop::EventLoop$LT$T$GT$::run::h5bedd1a1d0393d84","symbolLocation":52,"imageIndex":0},{"imageOffset":3980044,"symbol":"tao::event_loop::EventLoop$LT$T$GT$::run::h2f3698526c56c9cf","symbolLocation":88,"imageIndex":0},{"imageOffset":636172,"symbol":"_$LT$tauri_runtime_wry..Wry$LT$T$GT$$u20$as$u20$tauri_runtime..Runtime$LT$T$GT$$GT$::run::h0edfc140ae436836","symbolLocation":172,"imageIndex":0},{"imageOffset":878112,"symbol":"tauri::app::App$LT$R$GT$::run::h5a76840d935e3800","symbolLocation":640,"imageIndex":0},{"imageOffset":879776,"symbol":"tauri::app::Builder$LT$R$GT$::run::h907919b5b4253f48","symbolLocation":284,"imageIndex":0},{"imageOffset":2774288,"symbol":"auto_sub_lib::run::h8256e938c1cb1ebf","symbolLocation":776,"imageIndex":0},{"imageOffset":5064,"sourceLine":5,"sourceFile":"main.rs","symbol":"auto_sub::main::ha0c220c0bb59a533","imageIndex":0,"symbolLocation":12},{"imageOffset":4764,"sourceLine":250,"sourceFile":"function.rs","symbol":"core::ops::function::FnOnce::call_once::ha9e8ad9b93c8b446","imageIndex":0,"symbolLocation":20},{"imageOffset":5024,"sourceLine":166,"sourceFile":"backtrace.rs","symbol":"std::sys::backtrace::__rust_begin_short_backtrace::h48345fc226584c3b","imageIndex":0,"symbolLocation":24},{"imageOffset":4976,"sourceLine":206,"sourceFile":"rt.rs","symbol":"std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h77db1f6bead99b3b","imageIndex":0,"symbolLocation":28},{"imageOffset":29653244,"symbol":"std::rt::lang_start_internal::h11fc0900699d88c7","symbolLocation":952,"imageIndex":0},{"imageOffset":4936,"sourceLine":205,"sourceFile":"rt.rs","symbol":"std::rt::lang_start::h04c53eeb3f84f887","imageIndex":0,"symbolLocation":84},{"imageOffset":5108,"symbol":"main","symbolLocation":36,"imageIndex":0},{"imageOffset":130468,"symbol":"start","symbolLocation":6992,"imageIndex":12}]},{"id":222142,"threadState":{"x":[{"value":60},{"value":0},{"value":1},{"value":1},{"value":0},{"value":500000000},{"value":0},{"value":0},{"value":8425176088,"symbolLocation":0,"symbol":"clock_sem"},{"value":3},{"value":17},{"value":2},{"value":5},{"value":43781531840},{"value":72057602463256489,"symbolLocation":72057594037927937,"symbol":"OBJC_CLASS_$_NSLock"},{"value":8425328552,"symbolLocation":0,"symbol":"OBJC_CLASS_$_NSLock"},{"value":334},{"value":8448434000},{"value":0},{"value":0},{"value":6090092144},{"value":4294967295},{"value":4571821632},{"value":284},{"value":6090092768},{"value":18446744073709551615},{"value":4293984255},{"value":0},{"value":4}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6613793984},"cpsr":{"value":2684358656},"fp":{"value":6090092128},"sp":{"value":6090092080},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614987528},"far":{"value":0}},"queue":"com.apple.root.default-qos","frames":[{"imageOffset":17160,"symbol":"__semwait_signal","symbolLocation":8,"imageIndex":4},{"imageOffset":56512,"symbol":"nanosleep","symbolLocation":220,"imageIndex":6},{"imageOffset":56280,"symbol":"usleep","symbolLocation":68,"imageIndex":6},{"imageOffset":9002752,"symbol":"__ggml_metal_rsets_init_block_invoke","symbolLocation":92,"imageIndex":0},{"imageOffset":6696,"symbol":"_dispatch_call_block_and_release","symbolLocation":32,"imageIndex":14},{"imageOffset":111792,"symbol":"_dispatch_client_callout","symbolLocation":16,"imageIndex":14},{"imageOffset":232860,"symbol":"<deduplicated_symbol>","symbolLocation":76,"imageIndex":14},{"imageOffset":80604,"symbol":"_dispatch_root_queue_drain","symbolLocation":708,"imageIndex":14},{"imageOffset":82208,"symbol":"_dispatch_worker_thread2","symbolLocation":184,"imageIndex":14},{"imageOffset":11908,"symbol":"_pthread_wqthread","symbolLocation":232,"imageIndex":5},{"imageOffset":7184,"symbol":"start_wqthread","symbolLocation":8,"imageIndex":5}]},{"id":222159,"name":"com.apple.NSEventThread","threadState":{"x":[{"value":268451845},{"value":21592279046},{"value":8589934592},{"value":135252815118336},{"value":0},{"value":135252815118336},{"value":2},{"value":4294967295},{"value":0},{"value":17179869184},{"value":0},{"value":2},{"value":0},{"value":0},{"value":31491},{"value":0},{"value":18446744073709551569},{"value":8448435760},{"value":0},{"value":4294967295},{"value":2},{"value":135252815118336},{"value":0},{"value":135252815118336},{"value":21592279046},{"value":6092382344},{"value":8589934592},{"value":18446744073709550527},{"value":4412409862,"symbolLocation":2665949,"symbol":"jieba_rs::hmm::RE_SKIP::h2034573c5557ff1f"}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615049588},"cpsr":{"value":4096},"fp":{"value":6092382192},"sp":{"value":6092382112},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614973492},"far":{"value":0}},"frames":[{"imageOffset":3124,"symbol":"mach_msg2_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":79220,"symbol":"mach_msg2_internal","symbolLocation":76,"imageIndex":4},{"imageOffset":39360,"symbol":"mach_msg_overwrite","symbolLocation":480,"imageIndex":4},{"imageOffset":4032,"symbol":"mach_msg","symbolLocation":24,"imageIndex":4},{"imageOffset":515432,"symbol":"__CFRunLoopServiceMachPort","symbolLocation":160,"imageIndex":10},{"imageOffset":509524,"symbol":"__CFRunLoopRun","symbolLocation":1188,"imageIndex":10},{"imageOffset":1371104,"symbol":"_CFRunLoopRunSpecificWithOptions","symbolLocation":532,"imageIndex":10},{"imageOffset":1420388,"symbol":"_NSEventThread","symbolLocation":184,"imageIndex":7},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222201,"name":"WebCore: Scrolling","threadState":{"x":[{"value":268451845},{"value":21592279046},{"value":8589934592},{"value":215517163945984},{"value":0},{"value":215517163945984},{"value":2},{"value":4294967295},{"value":0},{"value":17179869184},{"value":0},{"value":2},{"value":0},{"value":0},{"value":50179},{"value":70368744194048},{"value":18446744073709551569},{"value":8448435760},{"value":0},{"value":4294967295},{"value":2},{"value":215517163945984},{"value":0},{"value":215517163945984},{"value":21592279046},{"value":6093529032},{"value":8589934592},{"value":18446744073709550527},{"value":4412409862,"symbolLocation":2665949,"symbol":"jieba_rs::hmm::RE_SKIP::h2034573c5557ff1f"}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615049588},"cpsr":{"value":4096},"fp":{"value":6093528880},"sp":{"value":6093528800},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614973492},"far":{"value":0}},"frames":[{"imageOffset":3124,"symbol":"mach_msg2_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":79220,"symbol":"mach_msg2_internal","symbolLocation":76,"imageIndex":4},{"imageOffset":39360,"symbol":"mach_msg_overwrite","symbolLocation":480,"imageIndex":4},{"imageOffset":4032,"symbol":"mach_msg","symbolLocation":24,"imageIndex":4},{"imageOffset":515432,"symbol":"__CFRunLoopServiceMachPort","symbolLocation":160,"imageIndex":10},{"imageOffset":509524,"symbol":"__CFRunLoopRun","symbolLocation":1188,"imageIndex":10},{"imageOffset":1371104,"symbol":"_CFRunLoopRunSpecificWithOptions","symbolLocation":532,"imageIndex":10},{"imageOffset":992548,"symbol":"CFRunLoopRun","symbolLocation":64,"imageIndex":10},{"imageOffset":2039856,"symbol":"WTF::Detail::CallableWrapper<WTF::RunLoop::create(WTF::ASCIILiteral, WTF::ThreadType, WTF::Thread::QOS)::$_0, void>::call()","symbolLocation":240,"imageIndex":9},{"imageOffset":2332876,"symbol":"WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*)","symbolLocation":296,"imageIndex":9},{"imageOffset":24712,"symbol":"WTF::wtfThreadEntryPoint(void*)","symbolLocation":16,"imageIndex":9},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222268,"name":"Log work queue","threadState":{"x":[{"value":14},{"value":4880588166},{"value":0},{"value":6098688320},{"value":8380447736,"symbolLocation":0,"symbol":"_os_log_current_test_callback"},{"value":1},{"value":69287813},{"value":6098694368},{"value":0},{"value":0},{"value":0},{"value":0},{"value":7346},{"value":7346},{"value":8425144400,"symbolLocation":0,"symbol":"OBJC_CLASS_$_OS_os_log"},{"value":8425144400,"symbolLocation":0,"symbol":"OBJC_CLASS_$_OS_os_log"},{"value":18446744073709551580},{"value":8448438256},{"value":0},{"value":5033216896},{"value":5033216936},{"value":6098694144},{"value":0},{"value":0},{"value":5033542528},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":7340636848},"cpsr":{"value":2147487744},"fp":{"value":6098693968},"sp":{"value":6098693936},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614973360},"far":{"value":0}},"frames":[{"imageOffset":2992,"symbol":"semaphore_wait_trap","symbolLocation":8,"imageIndex":4},{"imageOffset":18946736,"symbol":"IPC::StreamConnectionWorkQueue::startProcessingThread()::$_0::operator()()","symbolLocation":48,"imageIndex":8},{"imageOffset":2332876,"symbol":"WTF::Thread::entryPoint(WTF::Thread::NewThreadContext*)","symbolLocation":296,"imageIndex":9},{"imageOffset":24712,"symbol":"WTF::wtfThreadEntryPoint(void*)","symbolLocation":16,"imageIndex":9},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222334,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":2718208},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6100831256},{"value":0},{"value":4864},{"value":20890720932610},{"value":20890720932610},{"value":4864},{"value":0},{"value":20890720932608},{"value":305},{"value":8448433880},{"value":0},{"value":43822753688},{"value":43822753752},{"value":6100840672},{"value":0},{"value":0},{"value":2718208},{"value":2718209},{"value":2718464},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6100831376},"sp":{"value":6100831232},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222335,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":256},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6102977560},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":43822755480},{"value":43822755544},{"value":6102986976},{"value":0},{"value":0},{"value":256},{"value":257},{"value":512},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6102977680},"sp":{"value":6102977536},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222336,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":256},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6105123864},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":43822757272},{"value":43822757336},{"value":6105133280},{"value":0},{"value":0},{"value":256},{"value":257},{"value":512},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6105123984},"sp":{"value":6105123840},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222337,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":256},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6107270168},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":43822759064},{"value":43822759128},{"value":6107279584},{"value":0},{"value":0},{"value":256},{"value":257},{"value":512},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6107270288},"sp":{"value":6107270144},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222338,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":256},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6109416472},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":43826971800},{"value":43826971864},{"value":6109425888},{"value":0},{"value":0},{"value":256},{"value":257},{"value":512},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6109416592},"sp":{"value":6109416448},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222339,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":256},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6111562776},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":43826973592},{"value":43826973656},{"value":6111572192},{"value":0},{"value":0},{"value":256},{"value":257},{"value":512},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6111562896},"sp":{"value":6111562752},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222340,"name":"tokio-rt-worker","threadState":{"x":[{"value":4},{"value":0},{"value":0},{"value":43837505536},{"value":1024},{"value":0},{"value":43836114688},{"value":0},{"value":1024},{"value":43837505536},{"value":1},{"value":43841824504},{"value":6113711544},{"value":43843118288},{"value":18446744073709545498},{"value":4},{"value":363},{"value":8448435600},{"value":0},{"value":4883791872},{"value":43848271024},{"value":4423681688,"symbolLocation":70936,"symbol":"tao::platform_impl::platform::util::cursor::invisible_cursor::CURSOR_BYTES::h065bd60fe84f49ce"},{"value":43823703616},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":4402989120},"cpsr":{"value":1610616832},"fp":{"value":6113709968},"sp":{"value":6113709808},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614998980},"far":{"value":0}},"frames":[{"imageOffset":28612,"symbol":"kevent","symbolLocation":8,"imageIndex":4},{"imageOffset":25643072,"symbol":"mio::sys::unix::selector::Selector::select::hafab6b932786ff95","symbolLocation":200,"imageIndex":0},{"imageOffset":25626784,"symbol":"mio::poll::Poll::poll::hbae0e0a074cec30d","symbolLocation":80,"imageIndex":0},{"imageOffset":25492732,"symbol":"tokio::runtime::io::driver::Driver::turn::h8955633b70a6deec","symbolLocation":200,"imageIndex":0},{"imageOffset":25492520,"symbol":"tokio::runtime::io::driver::Driver::park::h02b5ad2959686a4d","symbolLocation":80,"imageIndex":0},{"imageOffset":25562996,"symbol":"tokio::runtime::signal::Driver::park::h1a2a4813cae6b2c0","symbolLocation":36,"imageIndex":0},{"imageOffset":25436800,"symbol":"tokio::runtime::process::Driver::park::h7225895c596a7a4c","symbolLocation":32,"imageIndex":0},{"imageOffset":25561880,"symbol":"tokio::runtime::driver::IoStack::park::h2b268599b72bfcf2","symbolLocation":104,"imageIndex":0},{"imageOffset":25343712,"symbol":"tokio::runtime::time::Driver::park_internal::he14f614974152d7b","symbolLocation":424,"imageIndex":0},{"imageOffset":25344788,"symbol":"tokio::runtime::time::Driver::park::h5cc4a76dcd959357","symbolLocation":40,"imageIndex":0},{"imageOffset":25558512,"symbol":"tokio::runtime::driver::TimeDriver::park::h02c9f591939bf24f","symbolLocation":92,"imageIndex":0},{"imageOffset":25561324,"symbol":"tokio::runtime::driver::Driver::park::h271244f23ac59555","symbolLocation":32,"imageIndex":0},{"imageOffset":25531048,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_driver::hd4e08710b10a9ece","symbolLocation":264,"imageIndex":0},{"imageOffset":25533668,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":240,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222341,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":1645056},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6115855384},{"value":0},{"value":3072},{"value":13194139536386},{"value":13194139536386},{"value":3072},{"value":0},{"value":13194139536384},{"value":305},{"value":8448433880},{"value":0},{"value":43826977176},{"value":43826977240},{"value":6115864800},{"value":0},{"value":0},{"value":1645056},{"value":1645057},{"value":1645312},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6115855504},"sp":{"value":6115855360},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222342,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":1086720},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6118001688},{"value":0},{"value":1024},{"value":4398046512130},{"value":4398046512130},{"value":1024},{"value":0},{"value":4398046512128},{"value":305},{"value":8448433880},{"value":0},{"value":43826978968},{"value":43826979032},{"value":6118011104},{"value":0},{"value":0},{"value":1086720},{"value":1086721},{"value":1086976},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6118001808},"sp":{"value":6118001664},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222343,"name":"tokio-rt-worker","threadState":{"x":[{"value":260},{"value":0},{"value":1792},{"value":0},{"value":0},{"value":160},{"value":0},{"value":0},{"value":6120147992},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":43826980760},{"value":43826980824},{"value":6120157408},{"value":0},{"value":0},{"value":1792},{"value":1793},{"value":2048},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6120148112},"sp":{"value":6120147968},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25678316,"symbol":"_$LT$parking_lot_core..thread_parker..imp..ThreadParker$u20$as$u20$parking_lot_core..thread_parker..ThreadParkerT$GT$::park::h17fd7eaa9d277104","symbolLocation":228,"imageIndex":0},{"imageOffset":25673292,"symbol":"parking_lot_core::parking_lot::park::_$u7b$$u7b$closure$u7d$$u7d$::hb60345df74bf056a","symbolLocation":748,"imageIndex":0},{"imageOffset":25668084,"symbol":"parking_lot_core::parking_lot::park::h5f72b57337891465","symbolLocation":296,"imageIndex":0},{"imageOffset":25680056,"symbol":"parking_lot::condvar::Condvar::wait_until_internal::h8d90a9cd3351e0ed","symbolLocation":160,"imageIndex":0},{"imageOffset":25474188,"symbol":"parking_lot::condvar::Condvar::wait::hc638ca3865a39696","symbolLocation":68,"imageIndex":0},{"imageOffset":25155964,"symbol":"tokio::loom::std::parking_lot::Condvar::wait::h9554695fd46e4d65","symbolLocation":36,"imageIndex":0},{"imageOffset":25531852,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park_condvar::h64e5afcde6e44349","symbolLocation":412,"imageIndex":0},{"imageOffset":25533608,"symbol":"tokio::runtime::scheduler::multi_thread::park::Inner::park::h7fac8dc8c99fc7c9","symbolLocation":180,"imageIndex":0},{"imageOffset":25534696,"symbol":"tokio::runtime::scheduler::multi_thread::park::Parker::park::hbe3e5fd07e0318e7","symbolLocation":40,"imageIndex":0},{"imageOffset":25447392,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park_internal::h67cb08af02a43c49","symbolLocation":872,"imageIndex":0},{"imageOffset":25451476,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::park::h8004c2d337f33e25","symbolLocation":968,"imageIndex":0},{"imageOffset":25450192,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Context::run::hb9dcf87a042d633d","symbolLocation":1764,"imageIndex":0},{"imageOffset":25438236,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::hc6c6d696645a335f","symbolLocation":104,"imageIndex":0},{"imageOffset":25421168,"symbol":"tokio::runtime::context::scoped::Scoped$LT$T$GT$::set::hc9d608cfb2e6bf2e","symbolLocation":148,"imageIndex":0},{"imageOffset":25497172,"symbol":"tokio::runtime::context::set_scheduler::_$u7b$$u7b$closure$u7d$$u7d$::h927ef41d657a78cb","symbolLocation":40,"imageIndex":0},{"imageOffset":25382852,"symbol":"std::thread::local::LocalKey$LT$T$GT$::try_with::hf28c8597e9d2ac0b","symbolLocation":196,"imageIndex":0},{"imageOffset":25376888,"symbol":"std::thread::local::LocalKey$LT$T$GT$::with::h5f4bcd000cc393bb","symbolLocation":24,"imageIndex":0},{"imageOffset":25496984,"symbol":"tokio::runtime::context::set_scheduler::hc53b125ae381926a","symbolLocation":68,"imageIndex":0},{"imageOffset":25438016,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::_$u7b$$u7b$closure$u7d$$u7d$::h4ed7083b09c1d37f","symbolLocation":248,"imageIndex":0},{"imageOffset":25361064,"symbol":"tokio::runtime::context::runtime::enter_runtime::h45f445116d7e4b16","symbolLocation":176,"imageIndex":0},{"imageOffset":25437672,"symbol":"tokio::runtime::scheduler::multi_thread::worker::run::hd2c9db91550c646d","symbolLocation":600,"imageIndex":0},{"imageOffset":25442216,"symbol":"tokio::runtime::scheduler::multi_thread::worker::Launch::launch::_$u7b$$u7b$closure$u7d$$u7d$::hd34921aa1b7e8da3","symbolLocation":24,"imageIndex":0},{"imageOffset":25399060,"symbol":"_$LT$tokio..runtime..blocking..task..BlockingTask$LT$T$GT$$u20$as$u20$core..future..future..Future$GT$::poll::hb50b57f771bd7b68","symbolLocation":136,"imageIndex":0},{"imageOffset":25140404,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::_$u7b$$u7b$closure$u7d$$u7d$::hecc927238bee67e6","symbolLocation":192,"imageIndex":0},{"imageOffset":25138908,"symbol":"tokio::runtime::task::core::Core$LT$T$C$S$GT$::poll::h6a5ded42705ac22c","symbolLocation":72,"imageIndex":0},{"imageOffset":25121324,"symbol":"tokio::runtime::task::harness::poll_future::_$u7b$$u7b$closure$u7d$$u7d$::hb25991776a77dbce","symbolLocation":64,"imageIndex":0},{"imageOffset":25488660,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::h34b30e6aa1b54c39","symbolLocation":44,"imageIndex":0},{"imageOffset":25515112,"symbol":"std::panicking::catch_unwind::do_call::h33b7a0e84f17c33f","symbolLocation":64,"imageIndex":0},{"imageOffset":25150172,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25147700,"symbol":"std::panic::catch_unwind::h7cbcd30bdfdad2a1","symbolLocation":116,"imageIndex":0},{"imageOffset":25120508,"symbol":"tokio::runtime::task::harness::poll_future::h8fecbf895a76008b","symbolLocation":96,"imageIndex":0},{"imageOffset":25123384,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll_inner::h587045022bbae356","symbolLocation":172,"imageIndex":0},{"imageOffset":25126632,"symbol":"tokio::runtime::task::harness::Harness$LT$T$C$S$GT$::poll::h241be6de44ee2bcd","symbolLocation":28,"imageIndex":0},{"imageOffset":25386652,"symbol":"tokio::runtime::task::raw::poll::h9424b663b968901f","symbolLocation":36,"imageIndex":0},{"imageOffset":25388636,"symbol":"tokio::runtime::task::raw::RawTask::poll::h86b59bbc50178e0a","symbolLocation":52,"imageIndex":0},{"imageOffset":25548672,"symbol":"tokio::runtime::task::UnownedTask$LT$S$GT$::run::h7f27025d7c876305","symbolLocation":64,"imageIndex":0},{"imageOffset":25293720,"symbol":"tokio::runtime::blocking::pool::Task::run::h53008843b290fab9","symbolLocation":28,"imageIndex":0},{"imageOffset":25294252,"symbol":"tokio::runtime::blocking::pool::Inner::run::hd4b09bf9b39e0a9c","symbolLocation":520,"imageIndex":0},{"imageOffset":25298056,"symbol":"tokio::runtime::blocking::pool::Spawner::spawn_thread::_$u7b$$u7b$closure$u7d$$u7d$::h6c2d326723c3deee","symbolLocation":144,"imageIndex":0},{"imageOffset":25275632,"symbol":"std::sys::backtrace::__rust_begin_short_backtrace::he2992fd6d4c495e5","symbolLocation":16,"imageIndex":0},{"imageOffset":25318888,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::ha0bf68111a3113ab","symbolLocation":116,"imageIndex":0},{"imageOffset":25489480,"symbol":"_$LT$core..panic..unwind_safe..AssertUnwindSafe$LT$F$GT$$u20$as$u20$core..ops..function..FnOnce$LT$$LP$$RP$$GT$$GT$::call_once::he671062148ba33f4","symbolLocation":44,"imageIndex":0},{"imageOffset":25516472,"symbol":"std::panicking::catch_unwind::do_call::hea3b362d8f5dbda0","symbolLocation":52,"imageIndex":0},{"imageOffset":25351520,"symbol":"__rust_try","symbolLocation":32,"imageIndex":0},{"imageOffset":25317880,"symbol":"std::thread::lifecycle::spawn_unchecked::_$u7b$$u7b$closure$u7d$$u7d$::h5474f3367235023b","symbolLocation":260,"imageIndex":0},{"imageOffset":25204916,"symbol":"core::ops::function::FnOnce::call_once$u7b$$u7b$vtable.shim$u7d$$u7d$::hec0609987beed401","symbolLocation":24,"imageIndex":0},{"imageOffset":29675680,"symbol":"std::sys::thread::unix::Thread::new::thread_start::h7cf39b7a7f135f48","symbolLocation":408,"imageIndex":0},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]},{"id":222852,"frames":[],"threadState":{"x":[{"value":6090665984},{"value":93455},{"value":6090129408},{"value":0},{"value":409604},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6090665984},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6615231496},"far":{"value":0}}},{"id":223085,"frames":[],"threadState":{"x":[{"value":6091812864},{"value":82955},{"value":6091276288},{"value":0},{"value":409604},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6091812864},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6615231496},"far":{"value":0}}},{"id":224158,"frames":[],"threadState":{"x":[{"value":6094106624},{"value":101775},{"value":6093570048},{"value":0},{"value":409604},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6094106624},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6615231496},"far":{"value":0}}},{"id":224159,"frames":[],"threadState":{"x":[{"value":6094680064},{"value":0},{"value":6094143488},{"value":0},{"value":278532},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6094680064},"esr":{"value":0},"pc":{"value":6615231496},"far":{"value":0}}},{"id":224160,"frames":[],"threadState":{"x":[{"value":6095253504},{"value":0},{"value":6094716928},{"value":0},{"value":278532},{"value":18446744073709551615},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0},{"value":0}],"flavor":"ARM_THREAD_STATE64","lr":{"value":0},"cpsr":{"value":4096},"fp":{"value":0},"sp":{"value":6095253504},"esr":{"value":0},"pc":{"value":6615231496},"far":{"value":0}}},{"id":229734,"name":"JavaScriptCore libpas scavenger","threadState":{"x":[{"value":316},{"value":0},{"value":4359936},{"value":0},{"value":0},{"value":160},{"value":0},{"value":5000032},{"value":6091239080},{"value":0},{"value":0},{"value":2},{"value":2},{"value":0},{"value":0},{"value":0},{"value":305},{"value":8448433880},{"value":0},{"value":4897237056},{"value":4897237120},{"value":6091239648},{"value":5000032},{"value":0},{"value":4359936},{"value":4361729},{"value":4361984},{"value":0},{"value":8427806720,"symbolLocation":3520,"symbol":"bmalloc_common_primitive_heap_support"}],"flavor":"ARM_THREAD_STATE64","lr":{"value":6615253288},"cpsr":{"value":1610616832},"fp":{"value":6091239200},"sp":{"value":6091239056},"esr":{"value":1442840704,"description":"(Syscall)"},"pc":{"value":6614988044},"far":{"value":0}},"frames":[{"imageOffset":17676,"symbol":"__psynch_cvwait","symbolLocation":8,"imageIndex":4},{"imageOffset":28968,"symbol":"_pthread_cond_wait","symbolLocation":980,"imageIndex":5},{"imageOffset":25877560,"symbol":"scavenger_thread_main","symbolLocation":1416,"imageIndex":9},{"imageOffset":27736,"symbol":"_pthread_start","symbolLocation":136,"imageIndex":5},{"imageOffset":7196,"symbol":"thread_start","symbolLocation":8,"imageIndex":5}]}],
  "usedImages" : [
  {
    "source" : "P",
    "arch" : "arm64",
    "base" : 4377346048,
    "size" : 44761088,
    "uuid" : "bd8d513f-f78d-3466-a1d3-102ad81e6268",
    "path" : "\/Users\/USER\/Documents\/*\/auto-sub",
    "name" : "auto-sub"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 4456579072,
    "size" : 49152,
    "uuid" : "a4dd56f1-375a-3540-844b-5e397f0b78b3",
    "path" : "\/usr\/lib\/libobjc-trampolines.dylib",
    "name" : "libobjc-trampolines.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 4886740992,
    "CFBundleShortVersionString" : "350.38",
    "CFBundleIdentifier" : "com.apple.AGXMetalG13X",
    "size" : 7913472,
    "uuid" : "95c14223-fa99-3e3d-9570-8e7862d86a54",
    "path" : "\/System\/Library\/Extensions\/AGXMetalG13X.bundle\/Contents\/MacOS\/AGXMetalG13X",
    "name" : "AGXMetalG13X",
    "CFBundleVersion" : "350.38"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 4866048000,
    "CFBundleShortVersionString" : "1.0",
    "CFBundleIdentifier" : "com.apple.AppleMetalOpenGLRenderer",
    "size" : 409600,
    "uuid" : "0a5a2e2b-9899-3606-af8c-84850db23fea",
    "path" : "\/System\/Library\/Extensions\/AppleMetalOpenGLRenderer.bundle\/Contents\/MacOS\/AppleMetalOpenGLRenderer",
    "name" : "AppleMetalOpenGLRenderer",
    "CFBundleVersion" : "1"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6614970368,
    "size" : 250512,
    "uuid" : "51565b39-f595-3e96-a217-fef29815057a",
    "path" : "\/usr\/lib\/system\/libsystem_kernel.dylib",
    "name" : "libsystem_kernel.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6615224320,
    "size" : 52028,
    "uuid" : "e7a73008-0c09-31e3-9dd9-0c61652f0e85",
    "path" : "\/usr\/lib\/system\/libsystem_pthread.dylib",
    "name" : "libsystem_pthread.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6613737472,
    "size" : 528120,
    "uuid" : "66ebd321-6899-3863-ba24-5cfc3076a0cb",
    "path" : "\/usr\/lib\/system\/libsystem_c.dylib",
    "name" : "libsystem_c.dylib"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6687268864,
    "CFBundleShortVersionString" : "6.9",
    "CFBundleIdentifier" : "com.apple.AppKit",
    "size" : 24259264,
    "uuid" : "59e23bd5-d01e-305a-b96f-a5790356049a",
    "path" : "\/System\/Library\/Frameworks\/AppKit.framework\/Versions\/C\/AppKit",
    "name" : "AppKit",
    "CFBundleVersion" : "2685.50.120"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 7321690112,
    "CFBundleShortVersionString" : "21624",
    "CFBundleIdentifier" : "com.apple.WebKit",
    "size" : 22909920,
    "uuid" : "e7e04c4f-689c-3e21-bd4a-c7591fd3d5ca",
    "path" : "\/System\/Library\/Frameworks\/WebKit.framework\/Versions\/A\/WebKit",
    "name" : "WebKit",
    "CFBundleVersion" : "21624.1.16.11.4"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 7183831040,
    "CFBundleShortVersionString" : "21624",
    "CFBundleIdentifier" : "com.apple.JavaScriptCore",
    "size" : 27954656,
    "uuid" : "cae1f78c-542e-30f7-8e08-fdca4b880e04",
    "path" : "\/System\/Library\/Frameworks\/JavaScriptCore.framework\/Versions\/A\/JavaScriptCore",
    "name" : "JavaScriptCore",
    "CFBundleVersion" : "21624.1.16.11.4"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6615511040,
    "CFBundleShortVersionString" : "6.9",
    "CFBundleIdentifier" : "com.apple.CoreFoundation",
    "size" : 5626976,
    "uuid" : "04941709-2330-3bf8-9213-6d33964db448",
    "path" : "\/System\/Library\/Frameworks\/CoreFoundation.framework\/Versions\/A\/CoreFoundation",
    "name" : "CoreFoundation",
    "CFBundleVersion" : "4424.1.402"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6831075328,
    "CFBundleShortVersionString" : "2.1.1",
    "CFBundleIdentifier" : "com.apple.HIToolbox",
    "size" : 3125344,
    "uuid" : "bcb81496-c81f-3d3e-a617-ccca047989e0",
    "path" : "\/System\/Library\/Frameworks\/Carbon.framework\/Versions\/A\/Frameworks\/HIToolbox.framework\/Versions\/A\/HIToolbox",
    "name" : "HIToolbox"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6611206144,
    "size" : 679624,
    "uuid" : "9f682dcf-340c-3bfa-bcdd-dd702f30313e",
    "path" : "\/usr\/lib\/dyld",
    "name" : "dyld"
  },
  {
    "size" : 0,
    "source" : "A",
    "base" : 0,
    "uuid" : "00000000-0000-0000-0000-000000000000"
  },
  {
    "source" : "P",
    "arch" : "arm64e",
    "base" : 6613430272,
    "size" : 291392,
    "uuid" : "e17aa23f-db2a-3302-b14c-f6b08c540fcf",
    "path" : "\/usr\/lib\/system\/libdispatch.dylib",
    "name" : "libdispatch.dylib"
  }
],
  "sharedCache" : {
  "base" : 6610075648,
  "size" : 5978570752,
  "uuid" : "7d4906c9-9ca2-3f56-8242-3ec2c1e3245b"
},
  "legacyInfo" : {
  "threadTriggered" : {
    "name" : "main",
    "queue" : "com.apple.main-thread"
  }
},
  "logWritingSignature" : "3c00286f990d55cff543d7f605c2619900cb2061",
  "bug_type" : "309",
  "roots_installed" : 0,
  "trmStatus" : 1,
  "trialInfo" : {
  "rollouts" : [
    {
      "rolloutId" : "695fd05d8ca5554688521e5e",
      "factorPackIds" : [
        "695fd08781fcd20ded79c1d3",
        "695fd0d28ca5554688521e5f",
        "695fd09c8774dc09015a80e9",
        "695fd0b18774dc09015a80ea"
      ],
      "deploymentId" : 3
    },
    {
      "rolloutId" : "60186475825c62000ccf5450",
      "factorPackIds" : [

      ],
      "deploymentId" : 240000083
    }
  ],
  "experiments" : [

  ]
}
}

Model: MacBookPro18,2, BootROM 18000.101.7, proc 10:8:2:0 processors, 32 GB, SMC 
Graphics: Apple M1 Max, Apple M1 Max, Built-In
Display: Color LCD, 3456 x 2234 Retina, Main, MirrorOff, Online
Memory Module: LPDDR5, Hynix
AirPort: spairport_wireless_card_type_wifi (0x14E4, 0x4387), wl0: Jan 20 2026 17:44:15 version 20.140.13.0.8.7.224 FWID 01-b2a3fd0d
IO80211_driverkit-1555.23 "IO80211_driverkit-1555.23" Mar  5 2026 21:45:55
AirPort: 
Bluetooth: Version (null), 0 services, 0 devices, 0 incoming serial ports
Network Service: Wi-Fi, AirPort, en0
Thunderbolt Bus: MacBook Pro, Apple Inc.
Thunderbolt Bus: MacBook Pro, Apple Inc.
Thunderbolt Bus: MacBook Pro, Apple Inc.
