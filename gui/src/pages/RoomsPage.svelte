<script lang="ts">
    import type { Room } from '../models/Room';

    // TODO: connect backend to replace fake values
    let room: Room = {
        room: {
            id: 'room.identifier',
            name: 'Room Display Name',
            description: 'Room description text',
            exits: {
                north: 'room.north_id',
                south: 'room.south_id',
                east: '',
                west: ''
            }
        },
        players: ['username1', 'username2'],
        items: ['item.id1', 'item.id2'],
        npcs: ['npc.id1', 'npc.id2']
    };
</script>

<div class="w-screen h-screen bg-black flex items-center justify-center">
    <!-- Main Room Block -->
    <div class="h-[80%] w-[80%] overflow-hidden border-2 border-white">

    </div>
</div>