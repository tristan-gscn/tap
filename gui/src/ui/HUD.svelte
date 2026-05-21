<script lang="ts">
    import StatusPanel from './panels/StatusPanel.svelte';
    import RoomInfoPanel from './panels/RoomInfoPanel.svelte';
    import InventoryPanel from './panels/InventoryPanel.svelte';
    import QuestPanel from './panels/QuestPanel.svelte';
    import ChatPanel from './panels/ChatPanel.svelte';
    import DialogueModal from './panels/DialogueModal.svelte';
    import WhoPanel from "./panels/WhoPanel.svelte";
</script>

<div class="pointer-events-none absolute inset-0">
    <div class="pointer-events-auto absolute left-4 top-5 w-72 space-y-3">
        <StatusPanel />
    </div>

    <div class="pointer-events-none absolute left-1/2 top-4 -translate-x-1/2">
        <RoomInfoPanel />
    </div>

    <div class="pointer-events-auto absolute right-4 top-5 w-72 space-y-3 overflow-y-auto pb-4" style="max-height: calc(100vh - 6rem)">
        <WhoPanel />
        <InventoryPanel />
        <QuestPanel />
    </div>

    <div class="pointer-events-auto absolute bottom-4 left-4 w-80">
        <ChatPanel />
    </div>

    <DialogueModal />
</div>
