using UnityEngine;

namespace LooCast.Noise
{
    using LooCast.Random;

    public class VoronoiMapGenerator : MonoBehaviour
    {
        public enum DrawMode
        {
            Distance,
            Cell
        }
        public DrawMode drawMode;


        public int mapWidth;
        public int mapHeight;
        public Vector2Int sampleCellAmount;
        public Vector2 sampleCellOffset;
        public Vector2Int borderedSampleCellAmount
        {
            get
            {
                return new Vector2Int(sampleCellAmount.x + cellSampleBorderThickness * 2, sampleCellAmount.y + cellSampleBorderThickness * 2);
            }
        }

        public int cellSampleBorderThickness;
        [Range(0.0f, 1.0f)]
        public float cellSpread;
        public float scale;
        public float power;

        [Range(0, 2109876543)]
        public int seed;

        public bool autoUpdate;

        public void GenerateMap()
        {
            MapDisplay display = GetComponent<MapDisplay>();
            if (drawMode == DrawMode.Distance)
            {
                display.DrawTexture(TextureGenerator.TextureFromColorMap(GetDistanceColorMap(), mapWidth, mapHeight));
            }

            else if (drawMode == DrawMode.Cell)
            {
                display.DrawTexture(TextureGenerator.TextureFromColorMap(GetCellColorMap(), mapWidth, mapHeight));
            }
        }

        private Vector2Int[] GetCentroids()
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = new Vector2Int[borderedSampleCellAmount.x * borderedSampleCellAmount.y];
            for (int y = 0; y < borderedSampleCellAmount.y; y++)
            {
                for (int x = 0; x < borderedSampleCellAmount.x; x++)
                {
                    int centroidIndex = y * borderedSampleCellAmount.x + x;
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
                            -(int)
                            (
                                (halfCentroidDimensions.x * cellSpread)
                            ),
                            (int)
                            (
                                (halfCentroidDimensions.x * cellSpread)
                            )
                        ) - ((centroidDimensions.x * cellSampleBorderThickness)) - centroidDimensions.x,
                        prng.Range
                        (
                            -(int)
                            (
                                (halfCentroidDimensions.y * cellSpread)
                            ),
                            (int)
                            (
                                (halfCentroidDimensions.y * cellSpread)
                            )
                        ) - ((centroidDimensions.y * cellSampleBorderThickness)) - centroidDimensions.y
                    );

                    centroids[centroidIndex] = new Vector2Int
                    (
                        (int)
                        (
                            (
                                (
                                    (
                                        (
                                            x + sampleCellOffset.x
                                        ) * centroidDimensions.x
                                    ) + centroidPositionOffset.x
                                )
                            ) / scale
                        ),
                        (int)
                        (
                            (
                                (
                                    (
                                        (
                                            y + sampleCellOffset.y
                                        ) * centroidDimensions.y
                                    ) + centroidPositionOffset.y
                                )
                            ) / scale
                        )
                    );

                }
            }
            return centroids;
        }

        private Color[] GetCellColorMap()
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = GetCentroids();

            Color[] centroidColors = new Color[borderedSampleCellAmount.x * borderedSampleCellAmount.y];
            for (int i = 0; i < centroidColors.Length; i++)
            {
                centroidColors[i] = prng.Color();
            }

            Color[] colorMap = new Color[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    int currentPixelIndex = y * mapWidth + x;
                    Vector2Int currentPixelPosition = new Vector2Int(x, y);
                    int closestCentroidIndex = GetClosestCentroidIndex(currentPixelPosition, centroids);
                    Vector2Int closestCentroid = centroids[closestCentroidIndex];
                    if (closestCentroid == currentPixelPosition)
                    {
                        colorMap[currentPixelIndex] = Color.black;
                    }
                    else
                    {
                        colorMap[currentPixelIndex] = centroidColors[closestCentroidIndex];     
                    }
                }
            }

            return colorMap;
        }

        private Color[] GetDistanceColorMap()
        {
            Vector2Int[] centroids = GetCentroids();
            float[] distances = new float[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    distances[y * mapWidth + x] = Vector2.Distance(new Vector2Int(x, y), centroids[GetClosestCentroidIndex(new Vector2Int(x, y), centroids)]);
                }
            }

            Color[] colorMap = new Color[mapWidth * mapHeight];
            float maxDistance = GetMaxDistance(distances);
            for (int i = 0; i < distances.Length; i++)
            {
                float colorValue = Mathf.Pow(distances[i] / maxDistance, power);
                colorMap[i] = new Color(colorValue, colorValue, colorValue, 1.0f);
            }

            return colorMap;
        }

        private float GetMaxDistance(float[] distances)
        {
            float maxDistance = float.MinValue;
            for (int i = 0; i < distances.Length; i++)
            {
                if (distances[i] > maxDistance)
                {
                    maxDistance = distances[i];
                }
            }
            return maxDistance;
        }

        private int GetClosestCentroidIndex(Vector2Int pixelPos, Vector2Int[] centroids)
        {
            float minDistance = float.MaxValue;
            int centroidIndex = 0;
            for (int i = 0; i < centroids.Length; i++)
            {
                float distance = Vector2.Distance(pixelPos, centroids[i]);
                if (distance < minDistance)
                {
                    minDistance = distance;
                    centroidIndex = i;
                }
            }
            return centroidIndex;
        }

        private void OnValidate()
        {
            if (mapWidth < 1)
            {
                mapWidth = 1;
            }
            if (mapHeight < 1)
            {
                mapHeight = 1;
            }
            if (sampleCellAmount.x < 1)
            {
                sampleCellAmount.x = 1;
            }
            if (sampleCellAmount.y < 1)
            {
                sampleCellAmount.y = 1;
            }
        }
    } 
}
