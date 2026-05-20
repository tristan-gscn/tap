<script lang="ts">
    import StatusPanel from './panels/StatusPanel.svelte';
    import RoomInfoPanel from './panels/RoomInfoPanel.svelte';
    import InventoryPanel from './panels/InventoryPanel.svelte';
    import QuestPanel from './panels/QuestPanel.svelte';
    import NpcPanel from './panels/NpcPanel.svelte';
    import ChatPanel from './panels/ChatPanel.svelte';
    import WhoPanel from './panels/WhoPanel.svelte';
    import DialogueModal from './panels/DialogueModal.svelte';
</script>

<div class="pointer-events-none absolute inset-0">
    <div class="pointer-events-auto absolute left-4 top-20 w-72 space-y-3">
        <StatusPanel />
        <RoomInfoPanel />
        <NpcPanel />
    </div>

    <div class="pointer-events-auto absolute right-4 top-20 w-72 space-y-3 overflow-y-auto pb-4" style="max-height: calc(100vh - 6rem)">
        <InventoryPanel />
        <QuestPanel />
        <WhoPanel />
    </div>

    <div class="pointer-events-auto absolute bottom-4 left-4 w-80">
        <ChatPanel />
    </div>

    <DialogueModal />
</div>
