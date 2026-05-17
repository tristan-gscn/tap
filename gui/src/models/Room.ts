interface Exits {
    north: string;
    south: string;
    east: string;
    west: string;
}

export interface Room {
    room: {
        id: string;
        name: string;
        description: string;
        exits: Exits;
    };
    players: string[];
    items: string[];
    npcs: string[];
}