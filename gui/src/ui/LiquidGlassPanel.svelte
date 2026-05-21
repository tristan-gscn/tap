<script lang="ts">
    export let className = '';
    export let contrast: 'light' | 'dark' | 'light-contrast' | 'dark-contrast' = 'dark-contrast';
    export let styles =
        '{"style":{"roundness":14,"padding_x":2,"padding_y":2},"text":{"content":" ","edit":false,"font_family":"Poppins","size_weight":500,"font_size":0.1},"color":{"accent":"#D7DADD"}}';
</script>

<div class={`relative overflow-hidden rounded-[14px] p-3 text-white ${className}`}>
    <sv-liquid-glass
        class="pointer-events-none absolute inset-0 z-0 block h-full w-full"
        style="width: 100%; height: 100%;"
        {contrast}
        styles={styles}
    ></sv-liquid-glass>
    <div class="relative z-10">
        <slot />
    </div>
</div>
