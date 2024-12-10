module viktor::starknet_validity {
    use aptos_framework::account::new_event_handle;
    use aptos_framework::event::emit_event;
    use aptos_framework::event;

    struct State has store, drop, copy, key {
        global_root: u256,
        block_number: u256,
        block_hash: u256,
    }

    #[event]
    struct LogStateUpdate has store, drop {
        global_root: u256,
        block_number: u256,
        block_hash: u256,
    }

    public(friend) entry fun emit_event_for_block(s: &signer, block_number: u256) acquires State {
        let state = borrow_global_mut<State>(@viktor);
        state.block_number = block_number;
        state.block_hash = block_number;
        state.global_root = block_number;

        let event_handle = new_event_handle<LogStateUpdate>(s);
        emit_event(&mut event_handle, LogStateUpdate {
            global_root: block_number,
            block_number,
            block_hash: block_number,
        });
        event::destroy_handle(event_handle);
    }

    #[view]
    public fun state_root(): u256 acquires State {
        borrow_global<State>(@viktor).global_root
    }

    #[view]
    public fun state_block_number(): u256 acquires State {
        borrow_global<State>(@viktor).block_number
    }

    #[view]
    public fun state_block_hash(): u256 acquires State {
        borrow_global<State>(@viktor).block_hash
    }

    fun init_module(s: &signer) {
        let state = State {
            block_hash: 0,
            block_number: 0,
            global_root: 0
        };

        move_to(s, state);
    }
}