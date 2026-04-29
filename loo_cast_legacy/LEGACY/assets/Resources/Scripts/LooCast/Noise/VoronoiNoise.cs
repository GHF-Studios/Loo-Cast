using UnityEngine;

namespace LooCast.Noise
{
    using LooCast.Random;

    public static class VoronoiNoise
    {
        public static Vector2Int[] GetCentroids(int seed, int mapWidth, int mapHeight, Vector2Int sampleCellAmount, float cellSpread)
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = new Vector2Int[sampleCellAmount.x * sampleCellAmount.y];
            for (int y = 0; y < sampleCellAmount.y; y++)
            {
                for (int x = 0; x < sampleCellAmount.x; x++)
                {
                    int centroidIndex = y * sampleCellAmount.x + x;
                    Vector2 centroidDimensions = new Vector2
                    (
                        mapWidth / sampleCellAmount.x,
                        mapHeight / sampleCellAmount.y
                    );
                    Vector2 halfCentroidDimensions = centroidDimensions / 2;
                    Vector2 centroidPositionOffset = new Vector2
                    (
                        prng.Range
                        (
                            -(halfCentroidDimensions.x * cellSpread),
                            (halfCentroidDimensions.x * cellSpread)
                        ),
                        prng.Range
                        (
                            -(halfCentroidDimensions.y * cellSpread),
                            (halfCentroidDimensions.y * cellSpread)
                        )
                    );

                    centroids[centroidIndex] = new Vector2Int
                    (
                        (int)
                        (
                            (((x + 0.5f) * centroidDimensions.x) + centroidPositionOffset.x)
                        ),
                        (int)
                        (
                            (((y + 0.5f) * centroidDimensions.y) + centroidPositionOffset.y)
                        )
                    );

                }
            }
            return centroids;
        }
    }
}
