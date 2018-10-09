Experimenting with vector math libraries.

This will change a lot, don't use it :)

Potential goals:
* f32 game math use case only
* SSE implementation
* no traits (will see how that goes)
* By default the best implementation for your platform will be imported to glam (e.g. sse2 on x86_64) but other implementations are still available
