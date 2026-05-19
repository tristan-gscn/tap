export type Point = { x: number; y: number };

export const placeInCircle = (totalCount: number, radius: number = 30): Point[] => {
    const positions: Point[] = [];

    if (totalCount === 0) return positions;

    // Offset by -90 degrees (-PI/2) so the first sprite is at 12 o'clock instead of 3 o'clock.
    const startAngle = -Math.PI / 2;

    for (let i = 0; i < totalCount; i += 1) {
        const angle = startAngle + (2 * Math.PI * i) / totalCount;

        positions.push({
            x: 50 + Math.cos(angle) * radius,
            y: 50 + Math.sin(angle) * radius
        });
    }

    return positions;
};
