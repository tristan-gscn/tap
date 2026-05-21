<script lang="ts">
    import { game } from '../../state/game.svelte';

    let inviteName = $state('');
    let joinLeader = $state('');

    const inGroup = $derived(game.groupId !== null);

    function invite() {
        const name = inviteName.trim();
        if (!name) return;
        game.inviteToGroup(name);
        inviteName = '';
    }

    function join() {
        const leader = joinLeader.trim();
        if (!leader) return;
        game.joinGroup(leader);
        joinLeader = '';
    }
</script>

<div class="tap-panel">
    <div class="mb-2 flex items-center justify-between text-[0.82rem] font-medium text-white/90">
        <span>Group</span>
        {#if inGroup}
            <span class="text-[11px] font-normal text-white/55">#{game.groupId}</span>
        {/if}
    </div>

    {#if game.groupInvites.length > 0}
        <div class="mb-2 space-y-1">
            {#each game.groupInvites as invite (invite.groupId)}
                <div class="rounded-lg border border-white/15 bg-white/5 px-2.5 py-1.5 text-[11px]">
                    <div class="mb-1.5 text-white/80">
                        Invite from <span class="font-medium text-white">{invite.from}</span>
                        (leader {invite.leader})
                    </div>
                    <div class="flex gap-1.5">
                        <button type="button" class="tap-btn tap-btn-primary flex-1" onclick={() => game.joinGroup(invite.leader)}>
                            Join
                        </button>
                        <button type="button" class="tap-btn flex-1" onclick={() => game.declineInvite(invite.groupId)}>
                            Decline
                        </button>
                    </div>
                </div>
            {/each}
        </div>
    {/if}

    {#if inGroup}
        <ul class="mb-2 space-y-1">
            {#each game.groupMembers as member (member)}
                <li class="flex items-center justify-between rounded-lg border border-white/10 bg-white/5 px-2.5 py-1.5 text-[12px]">
                    <span class={member === game.playerName ? 'font-medium text-white' : 'text-white/80'}>{member}</span>
                    {#if member === game.groupLeader}
                        <span class="text-[10px] uppercase tracking-wide text-amber-300/80">leader</span>
                    {/if}
                </li>
            {/each}
        </ul>

        <div class="mb-2 flex gap-1.5">
            <input
                class="flex-1 rounded-md border border-white/20 bg-white/10 px-2 py-1 text-[11px] text-white placeholder-white/40 transition focus:border-white/50 focus:bg-white/15 focus:outline-none focus:ring-2 focus:ring-white/15"
                placeholder="Invite player…"
                bind:value={inviteName}
                onkeydown={(e) => e.key === 'Enter' && invite()}
            />
            <button type="button" class="tap-btn" onclick={invite} disabled={!inviteName.trim()}>Invite</button>
        </div>

        <button type="button" class="tap-btn w-full" onclick={() => game.leaveGroup()}>
            Leave group
        </button>
    {:else}
        <button type="button" class="tap-btn tap-btn-primary mb-2 w-full" onclick={() => game.createGroup()}>
            Create group
        </button>

        <div class="flex gap-1.5">
            <input
                class="flex-1 rounded-md border border-white/20 bg-white/10 px-2 py-1 text-[11px] text-white placeholder-white/40 transition focus:border-white/50 focus:bg-white/15 focus:outline-none focus:ring-2 focus:ring-white/15"
                placeholder="Leader name…"
                bind:value={joinLeader}
                onkeydown={(e) => e.key === 'Enter' && join()}
            />
            <button type="button" class="tap-btn" onclick={join} disabled={!joinLeader.trim()}>Join</button>
        </div>
    {/if}
</div>
