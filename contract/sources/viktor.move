module viktor::task {
    use aptos_framework::account::new_event_handle;
    use aptos_framework::event::emit_event;
    use aptos_framework::event;

    #[event]
    struct LogStateUpdate has store, drop {
        global_root: u256,
        block_number: u256,
        block_hash: u256,
    }

    public(friend) entry fun emit_event_for_block(s: &signer, block_number: u256) {
        let event_handle = new_event_handle<LogStateUpdate>(s);
        emit_event(&mut event_handle, LogStateUpdate {
            global_root: block_number,
            block_number,
            block_hash: block_number,
        });
        event::destroy_handle(event_handle);
    }
}